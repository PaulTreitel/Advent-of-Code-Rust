advent_of_code_2022::solution!(16);

use petgraph::{
    algo::floyd_warshall, data::Build, graph::{NodeIndex, UnGraph}, Graph, Undirected
};
use std::{
    cmp::max,
    collections::{HashMap, HashSet},
};

#[derive(Debug)]
struct Part1State {
    pos: NodeIndex,
    opened: HashSet<NodeIndex>,
    mins_left: i32,
    released: i32,
}


pub fn part_one(input: &str) -> Option<i32> {
    let (start, graph) = get_graph(input);
    let dists = floyd_warshall(&graph, |_| 1).ok().unwrap();
    let max_pressure = part_one_solve(start, &graph, &dists, 30);
    Some(max_pressure)
}

pub fn part_two(input: &str) -> Option<i32> {
    let (start, graph) = get_graph(input);
    let dists = floyd_warshall(&graph, |_| 1).ok().unwrap();
    let nonzero_valve_ct: u32 = graph.node_weights()
        .filter(|&x| *x > 0)
        .map(|x| *x)
        .count() as u32;
    println!("{:?}", graph);
    for i in 1..i32::pow(2, nonzero_valve_ct) + 1 {

    }
    None
}

fn part_one_solve(
    start: NodeIndex,
    graph: &Graph<i32, (), Undirected>,
    dists: &HashMap<(NodeIndex, NodeIndex), i32>,
    time_left: i32
) -> i32 {
    let mut to_visit = vec![Part1State {
        pos: start,
        opened: HashSet::new(),
        mins_left: time_left,
        released: 0,
    }];
    to_visit
        .get_mut(0)
        .unwrap()
        .opened
        .insert(start);
    let mut max_pressure = 0;

    while !to_visit.is_empty() {
        let curr = to_visit.pop().unwrap();
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

fn get_graph(
    input: &str,
) -> (
    NodeIndex,
    Graph<i32, (), petgraph::Undirected>,
) {
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
        while let Some(valve_name) = line.next() {
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

fn main() {
    let input = advent_of_code_2022::template::read_file("inputs", DAY);
    // let res = part_one(&input).unwrap();
    // println!("{}", res);
    let res = part_two(&input).unwrap();
    println!("{}", res);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(1651)); // fill in
    }
    #[test]
    fn test_part_two() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(1707)); // fill in
    }
}
