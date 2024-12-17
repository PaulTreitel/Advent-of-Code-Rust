use std::collections::HashMap;

use advent_of_code_2024::utils::{direction::{Direction, ORTHOGONAL_DIRECTIONS}, grid::{Grid, GridPos}, parse};

advent_of_code_2024::solution!(16);

const TURN_MULTIPLIER: u64 = 1000;

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
    Some(dfs_reindeer_path(&maze, &start, &end))
    // None
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn parse_input(input: &str) -> Grid<MazeCell> {
    let maze = parse::into_2d_array(
        input,
        parse::split_by_all_chars,
        |s| MazeCell::from_char(s.chars().next().unwrap())
    );
    Grid::from(maze)
}

fn dfs_reindeer_path(maze: &Grid<MazeCell>, start: &GridPos, end: &GridPos) -> u64 {
    // let mut visited = HashSet::new();
    let mut visited = HashMap::new();
    let mut stack = vec![];
    // length, turns, direction, position
    stack.push((0, 0, Direction::Right, *start));
    while let Some((len, turns, dir, pos)) = stack.pop() {
        let score = get_score(turns, len);
        if !visited.contains_key(&pos) {
            visited.insert(pos, score);
        } else if *visited.get(&pos).unwrap() <= score {
            continue;
        } else {
            *visited.get_mut(&pos).unwrap() = score;
        }
        if pos == *end {
            continue;
        }
        for new_pos in pos.get_orthogonal_neighbors() {
            if *maze.get(&new_pos).unwrap() != MazeCell::Path  {
                continue;
            }
            let new_dir = get_turn_dir(&pos, &new_pos).unwrap();
            let new_turns = {
                if new_dir == dir {
                    turns
                } else {
                    turns + 1
                }
            };
            stack.push((len + 1, new_turns, new_dir, new_pos));
        }
    }
    *visited.get(end).unwrap()
}

fn get_score(turns: u64, len: u64) -> u64 {
    turns * TURN_MULTIPLIER + len
}

fn get_turn_dir(from: &GridPos, to: &GridPos) -> Option<Direction> {
    for dir in ORTHOGONAL_DIRECTIONS {
        if from.position_in_dir(dir) == *to {
            return Some(dir);
        }
    }
    None
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two_example_two() {
        let input = advent_of_code_2024::template::read_file_part("examples", DAY, 2);
        let result = part_two(&input);
        assert_eq!(result, None);
    }
}
