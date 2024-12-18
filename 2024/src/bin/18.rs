use std::collections::HashMap;

use advent_of_code_2024::utils::{graph_algos::GraphWrapper, grid::{Grid, GridPos}};
use petgraph::{
    algo::dijkstra,
    graph::{node_index, NodeIndex},
    prelude::StableGraph,
    visit::Dfs, Undirected
};

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

type MemGraph = GraphWrapper<GridPos, u64, Undirected>;

pub fn part_one(input: &str) -> Option<u64> {
    let (iterations, side_len) = get_is_real(input);
    let (mut grid, corruptions) = parse_input(input, side_len);
    for corruption_idx in 0..iterations {
        let corruption = corruptions.get(corruption_idx).unwrap();
        *grid.get_mut(corruption).unwrap() = MemSpace::Corrupted;
    }
    let memgraph = graph_from_grid(&grid);
    let (start, end) = get_start_end(&memgraph, side_len);
    let dists = dijkstra(
        &memgraph.graph(),
        start,
        Some(end),
        |_| 1
    );
    let final_dist = *dists.get(&end).unwrap();
    Some(final_dist as u64)
}

pub fn part_two(input: &str) -> Option<String> {
    let (_, side_len) = get_is_real(input);
    let (grid, corruptions) = parse_input(input, side_len);
    let mut memgraph = graph_from_grid(&grid);
    let (start, end) = get_start_end(&memgraph, side_len);

    let final_corruption = run_part_two_sim(
        &mut memgraph,
        &corruptions,
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

fn get_start_end(memgraph: &MemGraph, side_len: usize) -> (NodeIndex<usize>, NodeIndex<usize>) {
    let start = *memgraph.node_from_val(&GridPos { row: 0, col: 0 })
        .unwrap()
        .first()
        .unwrap();
    let end = *memgraph.node_from_val(&GridPos { row: side_len - 1, col: side_len - 1 })
        .unwrap()
        .first()
        .unwrap();
    (start, end)
}

fn graph_from_grid(grid: &Grid<MemSpace>) -> MemGraph {
    let num_cells = grid.rows() * grid.cols();
    let mut pos_to_node_id = HashMap::with_capacity(num_cells);
    let mut node_id_to_pos = HashMap::with_capacity(num_cells);
    let mut graph = StableGraph::with_capacity(num_cells, 4 * num_cells);

    for (pos, tile) in grid.iter_by_rows() {
        if tile == MemSpace::Path {
            let id = graph.node_count();
            pos_to_node_id.insert(pos, vec![node_index(id)]);
            node_id_to_pos.insert(node_index::<usize>(id), pos);
            graph.add_node(pos);
        }
    }

    for node_id in 0..graph.node_count() {
        for n in graph[node_index(node_id)].get_orthogonal_neighbors() {
            if let Some(neighbor_id) = pos_to_node_id.get(&n).cloned() {
                let neighbor_id = *neighbor_id.first().unwrap();
                if !graph.contains_edge(neighbor_id, node_id.into()){
                    graph.add_edge(node_id.into(), neighbor_id.into(), 1u64);
                }
            }
        }
    }
    GraphWrapper::new(graph.clone(), pos_to_node_id)
}

fn get_is_real(input: &str) -> (usize, usize) {
    if input.len() > 1000 {
        (PUZZLE_SIM_STEPS, PUZZLE_SIDE_LEN)
    } else {
        (EXAMPLE_SIM_STEPS, EXAMPLE_SIDE_LEN)
    }
}

fn run_part_two_sim(
    graph: &mut MemGraph,
    // graph: &mut StableGraph<GridPos, u64, Undirected, usize>,
    corruptions: &Vec<GridPos>,
    // pos_to_id: &HashMap<GridPos, NodeIndex<usize>>,
    start: NodeIndex<usize>,
    end: NodeIndex<usize>,
) -> Option<GridPos> {
    for corrupt in corruptions {
        let remove_id = *graph.node_from_val(corrupt).unwrap().first().unwrap();
        graph.remove_node(remove_id);
        let mut dfs = Dfs::new(&(*graph.graph()), start.into());
        let mut still_reachable = false;
        while let Some(x) = dfs.next(&(*graph.graph())) {
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
