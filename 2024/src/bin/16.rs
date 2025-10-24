// Solving https://adventofcode.com/2024/day/16
use std::collections::{HashMap, HashSet};

use advent_of_code_2024::utils::{
    direction::{Direction, ORTHOGONAL_DIRECTIONS},
    graph_algos::GraphWrapper,
    grid::{Grid, GridPos},
    parse
};
use petgraph::{
    graph::{node_index, NodeIndex},
    prelude::StableGraph,
    visit::NodeIndexable,
    Directed
};

advent_of_code_2024::solution!(16);

const TURN_MULTIPLIER: u64 = 1000;

type ReindeerGraph = GraphWrapper<(GridPos, Direction), u64, Directed>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum MazeCell {
    Wall,
    Path,
}

impl MazeCell {
    pub fn from_char(ch: char) -> Self {
        match ch {
            'S' => Self::Path,
            'E' => Self::Path,
            '.' => Self::Path,
            '#' => Self::Wall,
            _ => panic!("character {} is not a maze cell", ch)
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let maze = parse_input(input);
    let start = GridPos::new(maze.rows() - 2, 1);
    let end = GridPos::new(1, maze.cols() - 2);
    let maze = graph_from_grid(maze, &start);

    let start_idx = *maze.first_node_from_val(&(start, Direction::Right)).unwrap();
    let dists_paths = maze.dijkstra_with_path(start_idx);

    let mut end_nodes = vec![];
    for end_dir in ORTHOGONAL_DIRECTIONS {
        if let Some(id) = maze.nodes_from_val(&(end, end_dir)) {
            let id = id.first().unwrap();
            end_nodes.push(*id);
        }
    }

    prune_end_nodes(&mut end_nodes, &dists_paths);
    let score = dists_paths.get(end_nodes.first().unwrap()).unwrap().0;
    Some(score as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let maze = parse_input(input);
    let start = GridPos::new(maze.rows() - 2, 1);
    let end = GridPos::new(1, maze.cols() - 2);
    let maze = graph_from_grid(maze, &start);

    let start_idx = *maze.first_node_from_val(&(start, Direction::Right)).unwrap();
    let dists_paths = maze.dijkstra_with_path(start_idx);

    let mut end_nodes = vec![];
    for end_dir in ORTHOGONAL_DIRECTIONS {
        if let Some(id) = maze.nodes_from_val(&(end, end_dir)) {
            let id = id.first().unwrap();
            end_nodes.push(*id);
        }
    }

    let pts = get_path_points(&maze, &mut end_nodes, &dists_paths);
    let pts: HashSet<GridPos> = pts.iter().map(|(pos, _)| *pos).collect();
    Some(pts.len() as u64)
}

fn parse_input(input: &str) -> Grid<MazeCell> {
    let maze = parse::into_2d_array(
        input,
        parse::split_by_all_chars,
        |s| MazeCell::from_char(s.chars().next().unwrap())
    );
    Grid::from(maze)
}

fn graph_from_grid(grid: Grid<MazeCell>, start: &GridPos) -> ReindeerGraph {
    let num_cells = grid.rows() * grid.cols();
    let mut pos_dir_to_node = HashMap::with_capacity(num_cells);
    let mut graph = StableGraph::with_capacity(num_cells, 4 * num_cells);

    // add nodes
    pos_dir_to_node.insert((*start, Direction::Right), vec![node_index(0)]);
    graph.add_node((*start, Direction::Right));

    for (pos, val) in grid.iter_by_rows() {
        if val != MazeCell::Path {
            continue;
        }
        for dir in ORTHOGONAL_DIRECTIONS {
            let dir_pos = pos.position_in_dir(dir);
            if grid.is_valid_cell(&dir_pos) && *grid.get(&dir_pos).unwrap() == MazeCell::Path {
                let new_node = (pos, dir.opposite());
                let id = graph.node_bound();
                pos_dir_to_node.insert(new_node, vec![node_index(id)]);
                graph.add_node(new_node);
            }
        }
    }

    // add edges
    for node_id in 0..graph.node_count() {
        let start_pos = graph[node_index(node_id)];
        for dir in ORTHOGONAL_DIRECTIONS {
            let new_pos = start_pos.0.position_in_dir(dir);
            if !grid.is_valid_cell(&new_pos)
                || *grid.get(&new_pos).unwrap() == MazeCell::Wall
                || !pos_dir_to_node.contains_key(&(new_pos, dir))
            {
                continue;
            }
            let new_pos_id = *pos_dir_to_node
                .get(&(new_pos, dir))
                .unwrap()
                .first()
                .unwrap();
            if graph.contains_edge(node_index(node_id), new_pos_id) {
                continue;
            }
            let weight = get_edge_weight(&start_pos, &(new_pos, dir));
            graph.add_edge(node_index(node_id), new_pos_id, weight);
        }
    }

    GraphWrapper::new(graph, pos_dir_to_node)
}

fn get_edge_weight(from: &(GridPos, Direction), to: &(GridPos, Direction)) -> u64 {
    let base = {
        if from.0 == to.0 {
            0
        } else {
            1
        }
    };
    let turn = {
        if from.1 == to.1 {
            0
        } else if from.1 == to.1.opposite() {
            2 * TURN_MULTIPLIER
        } else {
            TURN_MULTIPLIER
        }
    };
    base + turn
}

fn get_path_points(
    maze: &ReindeerGraph,
    end_nodes: &mut Vec<NodeIndex<usize>>,
    dists_paths: &HashMap<NodeIndex<usize>, (u64, Vec<NodeIndex<usize>>)>,
) -> HashSet<(GridPos, Direction)> {
    prune_end_nodes(end_nodes, dists_paths);
    let mut points = HashSet::new();
    let mut positions_to_visit = end_nodes.clone();
    while let Some(id) = positions_to_visit.pop() {
        points.insert(*maze.node_weight(id).unwrap());
        for next in &dists_paths.get(&id).unwrap().1 {
            if !positions_to_visit.contains(next) {
                positions_to_visit.push(*next);
            }
        }
    }
    points
}

fn prune_end_nodes(
    end_nodes: &mut Vec<NodeIndex<usize>>,
    dists_paths: &HashMap<NodeIndex<usize>, (u64, Vec<NodeIndex<usize>>)>
) {
    let min_score = end_nodes.iter()
        .map(|id| dists_paths.get(id).unwrap().0)
        .min()
        .unwrap();
    for idx in (0..end_nodes.len()).rev() {
        let node = end_nodes.get(idx).unwrap();
        let score = dists_paths.get(node).unwrap().0;
        if score > min_score {
            end_nodes.remove(idx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example_one() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_one_example_two() {
        let input = advent_of_code_2024::template::read_file_part("examples", DAY, 2);
        let result = part_one(&input);
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two_example_one() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(45));
    }

    #[test]
    fn test_part_two_example_two() {
        let input = advent_of_code_2024::template::read_file_part("examples", DAY, 2);
        let result = part_two(&input);
        assert_eq!(result, Some(64));
    }
}
