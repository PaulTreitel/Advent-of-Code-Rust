use std::collections::HashMap;

use advent_of_code_2024::utils::{
    direction::Direction as Dir,
    graph_algos::{GWNodeIdx, GraphWrapper},
    parse::{self, split_by_all_chars, to_first_char}
};
use petgraph::Directed;

advent_of_code_2024::solution!(21);

// ==== State ====
// position on numeric keypad
// position on directional keypad controlling numeric robot
// position on directional keypad controlling directional robot

// We know we have to visit every position in the numeric keypad in sequence
// but the state of the 2nd and 3rd order robots depends on the path taken.
//
// Repeatedly moving the numeric robot in the same direction requires the 1st
// order robot to push the same arrow repeatedly, which requires the 2nd order
// robot to push the A button repeatedly to cause it to hit the arrow button.
// This requires me to hit the A button repeatedly to cause the 2nd order robot
// to hit A repeatedly. Therefore moving in the same direction as much as
// possible is ideal. This logic extends to moving the 1st order robot. The 2nd
// order robot is directly moved by our button presses so this doesn't apply.
//
// If the numeric robot must go to 1, 4, or 7 from 0 or A, it is *probably*
// advantageous to move it upwards then left due to the bottom left gap. In
// reverse, going from 1, 4, or 7 to 0 or A is *probably* faster moving right
// then down. Similar logic applies to the 1st order robot going from Up or A
// to Left (move down then left) or the reverse (move right then up). Again,
// this doesn't apply to the 2nd order robot.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    moves_made: u64,
    numeric_pos: char,
    first_order_pos: char,
    second_order_pos: char,
}

const START_CHAR: char = 'A';

type KeypadGraph = GraphWrapper<char, u64, Directed>;
type KeypadPaths = HashMap<(GWNodeIdx, GWNodeIdx), Vec<Vec<Dir>>>;

pub fn part_one(input: &str) -> Option<u64> {
    let codes = parse_input(input);
    let (numeric_graph, num_dir_map) = numeric_graph();
    let numeric_paths = get_paths(&numeric_graph, &num_dir_map);
    let (dir_graph, dir_dir_map) = directional_graph();
    let dir_paths = get_paths(&dir_graph, &dir_dir_map);

        let start_state = State {
            moves_made: 0,
            numeric_pos: START_CHAR,
            first_order_pos: START_CHAR,
            second_order_pos: START_CHAR,
        };

    for code in codes {
        let mut curr_states = vec![start_state.clone()];
        for &cell in &code {
            let target_node = numeric_graph.nodes_from_val(&cell).unwrap()[0];
            // curr_states = paths_to_numeric_cell(&numeric_graph, &curr_states, &numeric_paths, &dir_paths, target_node);
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    parse::into_2d_array(input, split_by_all_chars, to_first_char)
}

fn paths_to_numeric_cell(
    numeric_graph: &KeypadGraph,
    numeric_paths: &KeypadPaths,
    dir_graph: &KeypadGraph,
    dir_paths: &KeypadPaths,
    curr_states: &Vec<State>,
    numeric_target: GWNodeIdx
) -> Vec<State> {
    let mut end_states = vec![];
    for state in curr_states {
        let start_node = *numeric_graph.first_node_from_val(&state.numeric_pos).unwrap();
        let num_paths = numeric_paths.get(&(start_node, numeric_target)).unwrap();
        for num_path in num_paths {
            let first_order_paths = get_first_order_paths(state, dir_graph, dir_paths, num_path);
        }
    }
    end_states
}

fn get_first_order_paths(
    state: &State,
    dir_graph: &KeypadGraph,
    dir_paths: &KeypadPaths,
    numeric_path: &Vec<Dir>
) -> Vec<State> {
    let mut end_states = vec![*state];
    for numeric_move in numeric_path {
        let mut new_states = vec![];
        let target_char = dir_to_key(*numeric_move);
        let target_node = *dir_graph.first_node_from_val(&target_char).unwrap();
        for curr_state in end_states {
            let start_node = *dir_graph.first_node_from_val(&curr_state.first_order_pos).unwrap();
        }
        end_states = new_states;

    }
    end_states
}

fn dir_to_key(d: Dir) -> char {
    match d {
        Dir::Up => '^',
        Dir::Down => 'v',
        Dir::Left => '<',
        Dir::Right => '>',
        _ => unreachable!()
    }
}

fn numeric_graph() -> (KeypadGraph, HashMap<(char, char), Dir>) {
    let nodes = vec!['7', '8', '9', '4', '5', '6', '1', '2', '3', '0', 'A'];
    let edges = vec![
        ('7', '8', 1),
        ('7', '4', 1),
        ('8', '9', 1),
        ('8', '5', 1),
        ('9', '6', 1),
        ('4', '5', 1),
        ('4', '1', 1),
        ('5', '2', 1),
        ('5', '6', 1),
        ('1', '2', 1),
        ('2', '3', 1),
        ('2', '0', 1),
        ('3', 'A', 1),
        ('0', 'A', 1),
    ];
    let dir_map = vec![
        (('7', '8'), Dir::Left),
        (('8', '7'), Dir::Right),
        (('7', '4'), Dir::Down),
        (('4', '7'), Dir::Up),
        (('8', '9'), Dir::Left),
        (('9', '8'), Dir::Right),
        (('8', '5'), Dir::Down),
        (('5', '8'), Dir::Up),
        (('9', '6'), Dir::Down),
        (('6', '9'), Dir::Up),
        (('4', '5'), Dir::Left),
        (('5', '4'), Dir::Right),
        (('4', '1'), Dir::Down),
        (('1', '4'), Dir::Up),
        (('5', '2'), Dir::Down),
        (('2', '5'), Dir::Up),
        (('5', '6'), Dir::Left),
        (('6', '5'), Dir::Right),
        (('1', '2'), Dir::Left),
        (('2', '1'), Dir::Right),
        (('2', '3'), Dir::Left),
        (('3', '2'), Dir::Right),
        (('2', '0'), Dir::Down),
        (('0', '2'), Dir::Up),
        (('3', 'A'), Dir::Down),
        (('A', '3'), Dir::Up),
        (('0', 'A'), Dir::Left),
        (('A', '0'), Dir::Right),
    ];
    (GraphWrapper::from_nodes_edges(nodes, edges), HashMap::from_iter(dir_map))
}

fn directional_graph() -> (KeypadGraph, HashMap<(char, char), Dir>) {
    let nodes = vec!['^', 'A', '<', 'v', '>'];
    let edges = vec![
        ('^', 'A', 1),
        ('A', '^', 1),
        ('^', 'v', 1),
        ('v', '^', 1),
        ('A', '>', 1),
        ('>', 'A', 1),
        ('>', 'v', 1),
        ('v', '>', 1),
        ('v', '<', 1),
        ('<', 'v', 1),
    ];
    let dir_map = vec![
        (('^', 'A'), Dir::Left),
        (('A', '^'), Dir::Right),
        (('^', 'v'), Dir::Down),
        (('v', '^'), Dir::Up),
        (('A', '>'), Dir::Down),
        (('>', 'A'), Dir::Up),
        (('>', 'v'), Dir::Left),
        (('v', '>'), Dir::Right),
        (('v', '<'), Dir::Left),
        (('<', 'v'), Dir::Right),
    ];

    (GraphWrapper::from_nodes_edges(nodes, edges), HashMap::from_iter(dir_map))
}

fn get_paths(graph: &KeypadGraph, dir_map: &HashMap<(char, char), Dir>) -> KeypadPaths {
    let mut all_paths = HashMap::new();
    for start_node in graph.node_indices() {
        let paths_from_start = graph.dijkstra_with_path(start_node);
        let dijkstra_paths = graph.full_paths_from_dijkstra(paths_from_start, start_node);
        for (end_node, paths_to_end) in dijkstra_paths {
            add_new_paths(graph, dir_map, &mut all_paths, start_node, end_node, &paths_to_end);
        }
    }
    all_paths
}

fn add_new_paths(
    graph: &KeypadGraph,
    dir_map: &HashMap<(char, char), Dir>,
    all_paths: &mut KeypadPaths,
    start_node: GWNodeIdx,
    end_node: GWNodeIdx,
    paths_to_end: &Vec<Vec<GWNodeIdx>>,
) {
    for path in paths_to_end {
        let mut dir_path = vec![];
        for idx in 0..path.len() - 1 {
            let from_pos = graph.node_weight(*path.get(idx).unwrap()).unwrap();
            let to_pos = graph.node_weight(*path.get(idx + 1).unwrap()).unwrap();
            dir_path.push(dir_map[&(*from_pos, *to_pos)]);
        }
        all_paths
            .entry((start_node, end_node))
            .and_modify(|x| x.push(dir_path.clone()))
            .or_insert(vec![dir_path]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, None);
    }
}
