advent_of_code_2022::solution!(16);

use petgraph::{
    algo::floyd_warshall, graph::{self, NodeIndex, UnGraph}, visit::NodeRef, Graph, Undirected
};
use std::{
    cmp::max,
    collections::{HashMap, HashSet, VecDeque},
    hash::{DefaultHasher, Hash, Hasher}, num::Wrapping, ops::{Deref, DerefMut}
};

const PART_ONE_TIME: i32 = 30;
const PART_TWO_TIME: i32 = 26;

#[derive(Debug)]
struct Part1State {
    pos: NodeIndex,
    opened: HashSet<NodeIndex>,
    mins_left: i32,
    released: i32,
}

struct Part2State {
    graph: Graph<i32, (), Undirected>,
    dists: HashMap<(NodeIndex, NodeIndex), i32>,
    me_pos: NodeIndex,
    me_dest: NodeIndex,
    me_eta: i32,
    elephant_pos: NodeIndex,
    elephant_dest: NodeIndex,
    elephant_eta: i32,
    opened: HashSet<NodeIndex>,
    released: i32,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct HashSetWrapper<O: Eq + Hash + PartialEq>(HashSet<O>);

impl<O: Eq + Hash + PartialEq> Hash for HashSetWrapper<O> {
    // IMPLEMENTING HASH HERE
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Use Wrapping<u64> to allow wrapping overflow
        let mut sum = Wrapping::default();
        for value in &self.0 {
            let mut hasher = DefaultHasher::new();
            Hash::hash(value, &mut hasher);
            sum += hasher.finish();
        }
        state.write_u64(sum.0);
    }
}

impl<O: Eq + Hash + PartialEq> Deref for HashSetWrapper<O> {
    type Target = HashSet<O>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<O: Eq + Hash + PartialEq> DerefMut for HashSetWrapper<O> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let (start, graph) = get_graph(input);
    let dists = floyd_warshall(&graph, |_| 1).ok().unwrap();
    let max_pressure = part_one_solve(start, &graph, &dists, PART_ONE_TIME);
    Some(max_pressure)
}

pub fn part_two(input: &str) -> Option<i32> {
    let (start, graph) = get_graph(input);
    let dists = floyd_warshall(&graph, |_| 1).ok().unwrap();
    let state = Part2State {
        graph,
        dists,
        me_pos: start,
        me_dest: start,
        me_eta: 0,
        elephant_pos: start,
        elephant_dest: start,
        elephant_eta: 0,
        opened: HashSet::new(),
        released: 0,
    };
    let valve_release_values = valve_release_bfs(&state, start, PART_TWO_TIME);
    let mut best_soln = 0;
    for (set1, res1) in &valve_release_values {
        for (set2, res2) in &valve_release_values {
            if set1.is_disjoint(&set2) {
                best_soln = max(best_soln, res1 + res2);
            }
        }
    }
    Some(best_soln)
}

fn valve_release_bfs(state: &Part2State, start: NodeIndex, time_left: i32
) -> HashMap<HashSetWrapper<NodeIndex>, i32> {
    let nonzero_nodes: Vec<NodeIndex> = state.graph
        .node_indices()
        .filter(|x| *state.graph.node_weight(*x).unwrap() > 0)
        .collect();
    let mut release_values = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, time_left, 0, HashSetWrapper(HashSet::<NodeIndex>::new())));

    while let Some((node, time, pressure, opened)) = queue.pop_front() {
        for neighbor in &nonzero_nodes {
            let dist = state.dists.get(&(node, *neighbor)).unwrap() + 1;
            let time_remaining = time - dist;
            let neighbor_pressure = *state.graph.node_weight(*neighbor).unwrap();
            if opened.contains(neighbor) || dist > time {
                continue;
            }

            let mut new_opened = opened.clone();
            new_opened.insert(*neighbor);
            let new_released = pressure + neighbor_pressure * time_remaining;
            release_values
                .entry(new_opened.clone())
                .and_modify(|x| {
                    if *x < new_released {
                        *x = new_released;
                    }
                })
                .or_insert(new_released);

            queue.push_back((*neighbor, time - dist, new_released, new_opened));
        }
    }
    release_values
}

fn part_one_solve(
    start: NodeIndex,
    graph: &Graph<i32, (), Undirected>,
    dists: &HashMap<(NodeIndex, NodeIndex), i32>,
    time_left: i32,
) -> i32 {
    let mut to_visit = vec![Part1State {
        pos: start,
        opened: HashSet::new(),
        mins_left: time_left,
        released: 0,
    }];
    to_visit.get_mut(0).unwrap().opened.insert(start);
    let mut max_pressure = 0;

    while let Some(curr) = to_visit.pop() {
        if curr.opened.len() == graph.node_count() {
            max_pressure = max(curr.released, max_pressure);
            continue;
        }
        let mut added = false;
        for n in graph.node_indices() {
            let pressure = graph.node_weight(n).unwrap();
            let cost = 1 + dists.get(&(curr.pos, n)).unwrap();
            if n == curr.pos || curr.opened.contains(&n) || *pressure == 0 || cost > curr.mins_left
            {
                continue;
            }

            let mut new_open = Part1State {
                pos: n,
                opened: curr.opened.clone(),
                mins_left: curr.mins_left - cost,
                released: curr.released + pressure * (curr.mins_left - cost),
            };
            new_open.opened.insert(n);
            to_visit.push(new_open);
            added = true;
        }
        if !added {
            max_pressure = max(curr.released, max_pressure);
        }
    }
    max_pressure
}

fn get_graph(input: &str) -> (NodeIndex, Graph<i32, (), petgraph::Undirected>) {
    let mut indices = HashMap::new();
    let mut start = NodeIndex::new(0);
    let mut graph = UnGraph::<i32, ()>::new_undirected();
    for line in input.lines() {
        let mut line = line.split_ascii_whitespace();
        line.next();
        let name = line.next().unwrap().to_string();
        line.next();
        line.next();
        let rate: i32 = line
            .next()
            .unwrap()
            .matches(char::is_numeric)
            .fold("".to_string(), |acc, ch| acc + ch)
            .parse()
            .unwrap();
        line.next();
        line.next();
        line.next();
        line.next();
        let mut neighbors = Vec::new();
        for valve_name in line {
            neighbors.push(valve_name.replace(",", ""));
        }
        let new_node = graph.add_node(rate);
        indices.insert(name.clone(), (new_node, neighbors));
        if name == "AA" {
            start = new_node;
        }
    }
    for v_name in indices.keys() {
        let vertex = indices.get(v_name).unwrap();
        for neighbor in vertex.1.clone() {
            graph.add_edge(vertex.0, indices.get(&neighbor).unwrap().0, ());
        }
    }
    (start, graph)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(1707));
    }
}
