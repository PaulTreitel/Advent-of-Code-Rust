use std::collections::HashMap;

use advent_of_code_2024::utils::grid::{Grid, GridPos};
use petgraph::{algo::dijkstra, matrix_graph::MatrixGraph, visit::Dfs, Undirected};

advent_of_code_2024::solution!(18);

const EXAMPLE_SIDE_LEN: usize = 7;
const PUZZLE_SIDE_LEN: usize = 71;
const EXAMPLE_SIM_STEPS: usize = 12;
const PUZZLE_SIM_STEPS: usize = 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum MemSpace {
    Path,
    Corrupted,
}

pub fn part_one(input: &str) -> Option<u64> {
    let (iterations, side_len) = get_is_real(input);
    let (mut grid, corruptions) = parse_input(input, side_len);
    for corruption_idx in 0..iterations {
        let corruption = corruptions.get(corruption_idx).unwrap();
        *grid.get_mut(corruption).unwrap() = MemSpace::Corrupted;
    }
    let (
        graph,
        _,
        pos_to_node
    ) = graph_from_grid(&grid);
    let start = *pos_to_node.get(&GridPos { row: 0, col: 0 }).unwrap();
    let end = *pos_to_node.get(&GridPos { row: side_len - 1, col: side_len - 1 }).unwrap();
    let dists = dijkstra(
        &graph,
        start.into(),
        Some(end.into()),
        |_| 1
    );
    let final_dist = *dists.get(&end.into()).unwrap();
    Some(final_dist as u64)
}

pub fn part_two(input: &str) -> Option<String> {
    let (_, side_len) = get_is_real(input);
    let (grid, corruptions) = parse_input(input, side_len);
    let (
        mut graph,
        _,
        pos_to_node
    ) = graph_from_grid(&grid);
    let start = *pos_to_node.get(&GridPos { row: 0, col: 0 }).unwrap();
    let end = *pos_to_node.get(&GridPos { row: side_len - 1, col: side_len - 1 }).unwrap();
    let final_corruption = run_part_two_sim(
        &mut graph,
        &corruptions,
        &pos_to_node,
        start,
        end
    ).unwrap();
    let output = format!("{},{}", final_corruption.row, final_corruption.col);
    Some(output)
}

fn parse_input(input: &str, side_len: usize) -> (Grid<MemSpace>, Vec<GridPos>) {
    let grid = Grid::new(side_len, side_len, MemSpace::Path);
    let mut corruption_sequence = vec![];
    for l in input.lines() {
        let mut l = l.split(",");
        let pos = GridPos {
            row: l.next().unwrap().parse().unwrap(),
            col: l.next().unwrap().parse().unwrap()
        };
        corruption_sequence.push(pos);
    };
    (grid, corruption_sequence)
}

fn graph_from_grid(grid: &Grid<MemSpace>) -> (
    MatrixGraph<(), u64, Undirected, Option<u64>, usize>,
    HashMap<usize, GridPos>,
    HashMap<GridPos, usize>,
) {
    let mut node_id_to_pos = HashMap::with_capacity(grid.rows() * grid.cols());
    let mut pos_to_node_id = HashMap::with_capacity(grid.rows() * grid.cols());
    let mut edges = vec![];
    for (idx, (pos, tile)) in grid.iter_by_rows().enumerate() {
        if tile == MemSpace::Corrupted {
            continue;
        }
        node_id_to_pos.insert(idx, pos);
        pos_to_node_id.insert(pos, idx);
    }

    for (node_id, pos) in &node_id_to_pos {
        for neighbor in pos.get_orthogonal_neighbors() {
            if !grid.is_valid_cell(&neighbor)
                || *grid.get(&neighbor).unwrap() == MemSpace::Corrupted
            {
                continue;
            }
            let neighbor_id = *pos_to_node_id.get(&neighbor).unwrap();
            if !edges.contains(&(neighbor_id, *node_id, 1u64)) {
                edges.push((*node_id, neighbor_id, 1u64));
            }
        }
    }
    let graph = MatrixGraph::from_edges(edges);
    (graph, node_id_to_pos, pos_to_node_id)
}

fn get_is_real(input: &str) -> (usize, usize) {
    if input.len() > 1000 {
        (PUZZLE_SIM_STEPS, PUZZLE_SIDE_LEN)
    } else {
        (EXAMPLE_SIM_STEPS, EXAMPLE_SIDE_LEN)
    }
}

fn run_part_two_sim(
    graph: &mut MatrixGraph<(), u64, Undirected, Option<u64>, usize>,
    corruptions: &Vec<GridPos>,
    pos_to_id: &HashMap<GridPos, usize>,
    start: usize,
    end: usize,
) -> Option<GridPos> {
    for corrupt in corruptions {
        let remove_id = *pos_to_id.get(corrupt).unwrap();
        graph.remove_node(remove_id.into());
        let mut dfs = Dfs::new(&(*graph), start.into());
        let mut still_reachable = false;
        while let Some(x) = dfs.next(&(*graph)) {
            if x == end.into() {
                still_reachable = true;
                break;
            }
        }
        if !still_reachable {
            return Some(*corrupt)
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some("6,1".to_string()));
    }
}
