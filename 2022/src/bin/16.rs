advent_of_code_2022::solution!(16);

use petgraph::{
    algo::floyd_warshall,
    graph::{NodeIndex, UnGraph},
    Graph, Undirected,
};
use std::{
    cmp::max,
    collections::{HashMap, HashSet},
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

pub fn part_one(input: &str) -> Option<i32> {
    let (start, graph) = get_graph(input);
    let dists = floyd_warshall(&graph, |_| 1).ok().unwrap();
    let max_pressure = part_one_solve(start, &graph, &dists, PART_ONE_TIME);
    Some(max_pressure)
}

pub fn part_two(input: &str) -> Option<i32> {
    let (start, graph) = get_graph(input);
    let dists = floyd_warshall(&graph, |_| 1).ok().unwrap();
    let mut state = Part2State {
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
    let max_pressure = part_two_solve(&mut state);
    Some(max_pressure)
}

fn part_two_solve(state: &mut Part2State) -> i32 {
    for i in 0..PART_TWO_TIME {
        // println!("\n\nROUND {}", i + 1);
        update_destinations(state, PART_TWO_TIME - i);
        // print_updates(state);
        state.me_eta -= 1;
        state.elephant_eta -= 1;
        state.released += get_current_pressure_release(state);
        // println!(
        //     "RELEASED {} PRESSURE",
        //     get_current_pressuure_release(state)
        // );
    }
    state.released
}

fn print_updates(state: &Part2State) {
    println!("I'm at {:?}, heading to {:?} with {} pressure in {} minutes",
        state.me_pos, state.me_dest, state.graph.node_weight(state.me_dest).unwrap(), state.me_eta
    );
    println!("Elephant is at {:?}, heading to {:?} with {} pressure in {} minutes",
        state.elephant_pos, state.elephant_dest,
        state.graph.node_weight(state.elephant_dest).unwrap(), state.elephant_eta
    );
}

fn update_destinations(
    state: &mut Part2State,
    rounds_left: i32,
) {
    if state.me_eta == 0 {
        state.me_pos = state.me_dest;
        state.opened.insert(state.me_pos);
        let mut me_values =
            get_release_potential(&state, state.me_pos, rounds_left);

        if let Some(target) = set_my_target(
            state,
            &mut me_values,
            state.elephant_dest,
            state.elephant_eta,
            rounds_left,
        ) {
            state.me_dest = target.0;
            state.me_eta = target.1;
        }
    }
    if state.elephant_eta == 0 {
        state.elephant_pos = state.elephant_dest;
        state.opened.insert(state.elephant_pos);
        let mut elephant_values =
            get_release_potential(&state, state.elephant_pos, rounds_left);

        if let Some(target) = set_my_target(
            state,
            &mut elephant_values,
            state.me_dest,
            state.me_eta,
            rounds_left,
        ) {
            state.elephant_dest = target.0;
            state.elephant_eta = target.1;
        }
    }
}

fn set_my_target(
    state: &Part2State,
    targets: &mut Vec<(NodeIndex, i32, i32)>,
    other_dest: NodeIndex,
    other_eta: i32,
    rounds_left: i32
) -> Option<(NodeIndex, i32)> {
    let mut other_targets = get_release_potential(
        state,
        other_dest,
        rounds_left - other_eta
    );
    // println!("getting other targets from {:?} with {} rounds remaining",
    //     other_dest, rounds_left - other_eta
    // );
    remove_current_destinations(state, targets);
    remove_current_destinations(state, &mut other_targets);
    // println!("targets = {:?}\n\nother targets = {:?}", targets, other_targets);
    if targets.is_empty() {
        return None;
    }
    if targets.len() == 1 {
        let t = targets.first().unwrap();
        return Some((t.0, t.1));
    }

    // Find the value that I take my best target and the elephant takes their next best target.
    let me_best = targets.get(targets.len() - 1).unwrap();
    let mut me_best_other = other_targets.get(other_targets.len() - 1).unwrap();
    if me_best.0 == me_best_other.0 {
        me_best_other = other_targets.get(other_targets.len() - 2).unwrap();
    }
    let value_i_take_best = me_best.2 + me_best_other.2;


    // Find the value that I take my 2nd best target and the elephant takes their next best target.
    let me_2nd_best = targets.get(targets.len() - 2).unwrap();
    let mut me_2nd_best_other = other_targets.get(other_targets.len() - 1).unwrap();
    if me_2nd_best.0 == me_2nd_best_other.0 {
        me_2nd_best_other = other_targets.get(other_targets.len() - 2).unwrap();
    }
    let value_i_take_2nd_best = me_2nd_best.2 + me_2nd_best_other.2;

    // println!("values are {} and {}", value_i_take_best, value_i_take_2nd_best);
    if value_i_take_best > value_i_take_2nd_best {
        Some((me_best.0, me_best.1))
    } else {
        Some((me_2nd_best.0, me_2nd_best.1))
    }
}

fn remove_current_destinations(
    state: &Part2State,
    targets: &mut Vec<(NodeIndex, i32, i32)>
) {
    let tmp_idx = targets
        .iter()
        .position(|(x, _, _)| *x == state.me_dest);
    if let Some(idx) = tmp_idx {
        targets.remove(idx);
    }
    let tmp_idx = targets
        .iter()
        .position(|(x, _, _)| *x == state.elephant_dest);
    if let Some(idx) = tmp_idx {
        targets.remove(idx);
    }
}

fn get_current_pressure_release(
    state: &Part2State
) -> i32 {
    let mut release = 0;
    for node in &state.opened {
        release += state.graph.node_weight(*node).unwrap();
    }
    release
}

fn get_release_potential(
    state: &Part2State,
    start: NodeIndex,
    rounds_left: i32,
) -> Vec<(NodeIndex, i32, i32)> {
    let mut release_potentials = vec![];
    for target in state.graph.node_indices() {
        let pressure = *state.graph.node_weight(target).unwrap();
        if pressure == 0 || state.opened.contains(&target) {
            continue;
        }
        let dist = *state.dists.get(&(start, target)).unwrap() + 1;
        let total_pressure_released = (rounds_left - dist) * pressure;
        release_potentials.push((target, dist, total_pressure_released));
    }
    release_potentials.sort_by(|(_, _, p1), (_, _, p2)| p1.cmp(p2));
    release_potentials
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
