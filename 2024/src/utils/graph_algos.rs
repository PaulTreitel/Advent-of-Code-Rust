use std::collections::{BinaryHeap, HashMap};

use graph_builder::prelude::*;

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
