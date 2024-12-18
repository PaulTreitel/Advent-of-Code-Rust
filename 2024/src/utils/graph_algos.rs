use std::{collections::{BinaryHeap, HashMap}, hash::Hash, usize};

use graph_builder::{DirectedNeighborsWithValues, UndirectedNeighborsWithValues};
use petgraph::{
    graph::{node_index, NodeIndex}, prelude::StableGraph, visit::{
        depth_first_search,
        Control,
        DfsEvent,
        GraphBase,
        GraphRef,
        IntoNeighbors,
        NodeCount,
        VisitMap,
        Visitable
    }, EdgeType
};

pub struct GraphWrapper<NWeight, EWeight, EType>
{
    graph: StableGraph<NWeight, EWeight, EType, usize>,
    vals_to_nodes: HashMap<NWeight, Vec<NodeIndex<usize>>>,
}

impl<NWeight, EWeight, EType> GraphWrapper<NWeight, EWeight, EType>
where
    StableGraph<NWeight, EWeight, EType, usize>:
        GraphBase<NodeId = NodeIndex<usize>>,
    NWeight: Clone + Eq + Hash,
    EType: EdgeType,
{
    pub fn new(
        graph: StableGraph<NWeight, EWeight, EType, usize> ,
        val_to_id: HashMap<NWeight, Vec<NodeIndex<usize>>>,
    ) -> Self {
        Self { graph, vals_to_nodes: val_to_id }
    }

    pub fn dfs_get_path(
        &self,
        start: NodeIndex<usize>,
        is_goal: impl Fn(NodeIndex<usize>) -> bool
    ) -> Option<Vec<NWeight>> {
        let path = dfs_get_path(&self.graph, start, is_goal)?;
        let path = path.iter()
            .map(|id| self.graph
                .node_weight(node_index(*id))
                .unwrap()
                .clone()
            )
            .collect();
        Some(path)
    }

    pub fn node_from_val(&self, val: &NWeight) -> Option<&Vec<NodeIndex<usize>>> {
        self.vals_to_nodes.get(val)
    }

    pub fn vals_to_nodes(&self) -> &HashMap<NWeight, Vec<NodeIndex<usize>>> {
        &self.vals_to_nodes
    }

    pub fn graph(&self) -> &StableGraph<NWeight, EWeight, EType, usize> {
        &self.graph
    }

    pub fn remove_node(&mut self, n: NodeIndex<usize>) -> Option<NWeight> {
        let val = self.graph.node_weight(n)?;
        {
            let x = self.vals_to_nodes.get_mut(val)?;
            x.remove(x.iter().position(|a| *a == n)?);
            if x.is_empty() {
                self.vals_to_nodes.remove(val);
            }
        }
        self.graph.remove_node(n)
    }
}

pub fn dfs_get_path<G, VM>(
    graph: G,
    start: NodeIndex<usize>,
    is_goal: impl Fn(NodeIndex<usize>) -> bool
) -> Option<Vec<usize>>
where
    VM: VisitMap<NodeIndex>,
    G: GraphRef + Visitable<NodeId = NodeIndex<usize>, Map = VM> + NodeCount + IntoNeighbors,
{
    let mut goal_node= node_index(graph.node_count() + 1);
    let mut predecessor = vec![NodeIndex::end(); graph.node_count()];
    depth_first_search(&graph, Some(start), |event| {
        if let DfsEvent::TreeEdge(u, v) = event {
            predecessor[v.index()] = u;
            if is_goal(v) {
                goal_node = v;
                return Control::Break(v);
            }
        }
        Control::Continue
    });
    if goal_node.index() == graph.node_count() + 1 {
        return None;
    }

    let mut next = goal_node;
    let mut path = vec![next.index()];
    while next != start {
        let pred = predecessor[next.index()];
        path.push(pred.index());
        next = pred;
    }
    path.reverse();
    Some(path)
}

pub fn directed_dijkstra<T, Graph, Dist>(
    graph: &Graph,
    start: T
) -> HashMap<T, (Dist, Vec<T>)>
where
    T: graph_builder::index::Idx + std::hash::Hash,
    Dist: std::ops::Add + Ord + num::Zero + Copy,
    Graph: graph_builder::Graph<T> + DirectedNeighborsWithValues<T, Dist>,
{
    let mut dists_paths = HashMap::new();
    let mut heap = BinaryHeap::new();
    dists_paths.insert(start, (Dist::zero(), vec![]));
    heap.push((Dist::zero(), start));

    while let Some((dist, node)) = heap.pop() {
        if dists_paths.contains_key(&node) && dist > dists_paths.get(&node).unwrap().0 {
            continue;
        }

        for neighbor in graph.out_neighbors_with_values(node) {
            let new_dist = dist + neighbor.value;
            let is_present = dists_paths.contains_key(&neighbor.target);
            if !is_present
                || dist + neighbor.value < dists_paths.get(&neighbor.target).unwrap().0
            {
                heap.push((new_dist, neighbor.target));
                dists_paths.entry(neighbor.target)
                    .and_modify(|x| {
                        x.0 = new_dist;
                        x.1 = vec![node];
                    })
                    .or_insert((new_dist, vec![node]));
            } else if dists_paths.contains_key(&neighbor.target)
                && new_dist == dists_paths.get(&neighbor.target).unwrap().0
            {
                dists_paths.get_mut(&neighbor.target).unwrap().1.push(node);
            }
        }
    }
    dists_paths
}

pub fn undirected_dijkstra<T, Graph, Dist>(
    graph: &Graph,
    start: T
) -> HashMap<T, (Dist, Vec<T>)>
where
    T: graph_builder::index::Idx + std::hash::Hash,
    Dist: std::ops::Add + Ord + num::Zero + Copy,
    Graph: graph_builder::Graph<T> + UndirectedNeighborsWithValues<T, Dist>
{
    let mut dists_paths = HashMap::new();
    let mut heap = BinaryHeap::new();
    dists_paths.insert(start, (Dist::zero(), vec![]));
    heap.push((Dist::zero(), start));

    while let Some((dist, node)) = heap.pop() {
        if dists_paths.contains_key(&node) && dist > dists_paths.get(&node).unwrap().0 {
            continue;
        }

        for neighbor in graph.neighbors_with_values(node) {
            let new_dist = dist + neighbor.value;
            let is_present = dists_paths.contains_key(&neighbor.target);
            if !is_present
                || dist + neighbor.value < dists_paths.get(&neighbor.target).unwrap().0
            {
                heap.push((new_dist, neighbor.target));
                dists_paths.entry(neighbor.target)
                    .and_modify(|x| {
                        x.0 = new_dist;
                        x.1 = vec![node];
                    })
                    .or_insert((new_dist, vec![node]));
            } else if dists_paths.contains_key(&neighbor.target)
                && new_dist == dists_paths.get(&neighbor.target).unwrap().0
            {
                dists_paths.get_mut(&neighbor.target).unwrap().1.push(node);
            }
        }
    }
    dists_paths
}
