use std::{collections::{BinaryHeap, HashMap}, hash::Hash, ops::Add};

use petgraph::{
    graph::{node_index, NodeIndex},
    prelude::StableGraph,
    visit::{
        depth_first_search,
        Control,
        DfsEvent,
        EdgeRef,
        GraphBase
    },
    EdgeType
};

pub type GWNodeIdx = NodeIndex<usize>;

pub struct GraphWrapper<NWeight, EWeight, EType>
{
    graph: StableGraph<NWeight, EWeight, EType, usize>,
    vals_to_nodes: HashMap<NWeight, Vec<GWNodeIdx>>,
}

impl<NWeight, EWeight, EType> GraphWrapper<NWeight, EWeight, EType>
where
    StableGraph<NWeight, EWeight, EType, usize>:
        GraphBase<NodeId = GWNodeIdx>,
    NWeight: Clone + Eq + Hash,
    EType: EdgeType,
{
    pub fn new(
        graph: StableGraph<NWeight, EWeight, EType, usize> ,
        val_to_id: HashMap<NWeight, Vec<GWNodeIdx>>,
    ) -> Self {
        Self { graph, vals_to_nodes: val_to_id }
    }

    pub fn dfs_get_path(
        &self,
        start: GWNodeIdx,
        is_goal: impl Fn(GWNodeIdx) -> bool
    ) -> Option<Vec<NWeight>> {
        let mut goal_node= node_index(self.graph.node_count() + 1);
        let mut predecessor = vec![NodeIndex::end(); self.graph.node_count()];
        depth_first_search(&self.graph, Some(start), |event| {
            if let DfsEvent::TreeEdge(u, v) = event {
                predecessor[v.index()] = u;
                if is_goal(v) {
                    goal_node = v;
                    return Control::Break(v);
                }
            }
            Control::Continue
        });
        if goal_node.index() == self.graph.node_count() + 1 {
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
        let path = path.iter()
            .map(|id| self.node_weight(node_index(*id)).unwrap().clone())
            .collect();
        Some(path)
    }

    pub fn node_from_val(&self, val: &NWeight) -> Option<&Vec<GWNodeIdx>> {
        self.vals_to_nodes.get(val)
    }

    pub fn vals_to_nodes(&self) -> &HashMap<NWeight, Vec<GWNodeIdx>> {
        &self.vals_to_nodes
    }

    pub fn graph(&self) -> &StableGraph<NWeight, EWeight, EType, usize> {
        &self.graph
    }

    pub fn node_weight(&self, n: GWNodeIdx) -> Option<&NWeight> {
        self.graph.node_weight(n)
    }

    pub fn remove_node(&mut self, n: GWNodeIdx) -> Option<NWeight> {
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

    pub fn dijkstra_with_path(&self, start: GWNodeIdx
    ) -> HashMap<GWNodeIdx, (EWeight, Vec<GWNodeIdx>)>
    where
        EWeight: Default + Ord + Add<Output = EWeight> + Copy,
        EType: EdgeType
    {
        let mut dists_paths = HashMap::new();
        let mut to_visit = BinaryHeap::new();
        dists_paths.insert(start, (EWeight::default(), vec![]));
        to_visit.push((EWeight::default(), start));

        while let Some((dist, node)) = to_visit.pop() {
            if dists_paths.contains_key(&node) && dist > dists_paths.get(&node).unwrap().0 {
                continue;
            }

            for neighbor in self.graph.edges(node) {
                let new_dist = dist + *neighbor.weight();
                let is_present = dists_paths.contains_key(&neighbor.target());
                if !is_present
                    || new_dist < dists_paths.get(&neighbor.target()).unwrap().0
                {
                    to_visit.push((new_dist, neighbor.target()));
                    dists_paths.entry(neighbor.target())
                        .and_modify(|x| {
                            x.0 = new_dist;
                            x.1 = vec![node];
                        })
                        .or_insert((new_dist, vec![node]));
                } else if dists_paths.contains_key(&neighbor.target())
                    && new_dist == dists_paths.get(&neighbor.target()).unwrap().0
                {
                    dists_paths.get_mut(&neighbor.target()).unwrap().1.push(node);
                }
            }
        }
        dists_paths
    }
}
