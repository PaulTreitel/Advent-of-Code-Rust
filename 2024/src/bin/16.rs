use std::collections::{HashMap, HashSet};

use advent_of_code_2024::utils::{direction::{Direction, ORTHOGONAL_DIRECTIONS}, graph_algos::directed_dijkstra, grid::{Grid, GridPos}, parse};
use graph_builder::{DirectedCsrGraph, GraphBuilder};

advent_of_code_2024::solution!(16);

const TURN_MULTIPLIER: i64 = 1000;

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
    let (graph, _, pos_dirs) = graph_from_grid(maze, &start);

    let start_idx = *pos_dirs.get(&(start, Direction::Right)).unwrap();
    let dists_paths = directed_dijkstra(&graph, start_idx);

    let mut end_nodes = vec![];
    for end_dir in ORTHOGONAL_DIRECTIONS {
        if let Some(id) = pos_dirs.get(&(end, end_dir)) {
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
    let (graph, node_ids, pos_dirs) = graph_from_grid(maze, &start);

    let start_idx = *pos_dirs.get(&(start, Direction::Right)).unwrap();
    let dists_paths = directed_dijkstra(&graph, start_idx);

    let mut end_nodes = vec![];
    for end_dir in ORTHOGONAL_DIRECTIONS {
        if let Some(id) = pos_dirs.get(&(end, end_dir)) {
            end_nodes.push(*id);
        }
    }

    let pts = get_path_points(&mut end_nodes, &dists_paths, &node_ids);
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

fn graph_from_grid(grid: Grid<MazeCell>, start: &GridPos
) -> (
    DirectedCsrGraph<usize, (), i64>,
    HashMap<usize, (GridPos, Direction)>,
    HashMap<(GridPos, Direction), usize>,
) {
    let mut pos_dir_to_node_id = HashMap::with_capacity(grid.rows() * grid.cols());
    let mut node_id_to_pos_dir = HashMap::with_capacity(grid.rows() * grid.cols());
    let mut edges: Vec<(usize, usize, i64)> = vec![];
    let mut node_id = 0;
    node_id_to_pos_dir.insert(node_id, (*start, Direction::Right));
    pos_dir_to_node_id.insert((*start, Direction::Right), node_id);
    node_id += 1;

    // set node identifiers
    for (pos, cell) in grid.iter_by_rows() {
        if cell == MazeCell::Wall {
            continue;
        }
        for dir in ORTHOGONAL_DIRECTIONS {
            let dir_pos = pos.position_in_dir(dir);
            // nodes for each Path and each direction we could come at it from
            if grid.is_valid_cell(&dir_pos) && *grid.get(&dir_pos).unwrap() == MazeCell::Path {
                node_id_to_pos_dir.insert(node_id, (pos, dir.opposite()));
                pos_dir_to_node_id.insert((pos, dir.opposite()), node_id);
                node_id += 1;
            }
        }
    }

    // construct edges
    for (id, (pos, dir)) in &node_id_to_pos_dir {
        // println!("starting at position {}, direction {}", pos, dir);
        for new_pos_dir in ORTHOGONAL_DIRECTIONS {
            let new_pos = pos.position_in_dir(new_pos_dir);
            if !grid.is_valid_cell(&new_pos) || *grid.get(&new_pos).unwrap() == MazeCell::Wall {
                continue;
            }
            // println!("finding new position id for pos {}, dir {}", new_pos, new_pos_dir);
            let new_pos_id = *pos_dir_to_node_id.get(&(new_pos, new_pos_dir)).unwrap();
            let edge_weight = 1 + {
                if new_pos_dir == *dir {
                    0
                } else if new_pos_dir == dir.opposite() {
                    2 * TURN_MULTIPLIER
                } else {
                    TURN_MULTIPLIER
                }
            };
            edges.push((*id, new_pos_id, edge_weight));
        }
    }
    let graph = GraphBuilder::new()
        .edges_with_values(edges)
        .build();
    (graph, node_id_to_pos_dir, pos_dir_to_node_id)
}

fn get_path_points(
    end_nodes: &mut Vec<usize>,
    dists_paths: &HashMap<usize, (i64, Vec<usize>)>,
    node_ids: &HashMap<usize, (GridPos, Direction)>
) -> HashSet<(GridPos, Direction)> {
    prune_end_nodes(end_nodes, dists_paths);
    let mut points = HashSet::new();
    let mut positions_to_visit = end_nodes.clone();
    while let Some(id) = positions_to_visit.pop() {
        points.insert(*node_ids.get(&id).unwrap());
        for next in &dists_paths.get(&id).unwrap().1 {
            if !positions_to_visit.contains(next) {
                positions_to_visit.push(*next);
            }
        }
    }
    points
}

fn prune_end_nodes(
    end_nodes: &mut Vec<usize>,
    dists_paths: &HashMap<usize, (i64, Vec<usize>)>
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
