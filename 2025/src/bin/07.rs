
use std::collections::HashMap;

use advent_of_code_2025::utils::{direction::Direction, grid::{self, GridPos}, parse};

// Solving https://adventofcode.com/2025/day/7
advent_of_code_2025::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut splits = 0;
    let (mut grid, start) = parse_input(input);
    *grid.get_mut(&start)? = '|';
    for row in 1..grid.rows() {
        for col in 0..grid.cols() {
            splits += split_beam(&mut grid, row, col)?;
        }
    }
    Some(splits)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (grid, start) = parse_input(input);
    let mut cache = HashMap::new();
    return Some(part_two_recursive(&grid, &mut cache, start))
}

fn parse_input(input: &str) -> (grid::Grid<char>, grid::GridPos) {
    let board = parse::into_2d_array(input,
        parse::split_by_all_chars,
        |&s| s.chars().next().unwrap());
    let start = board[0].iter().position(|&c| c == 'S').unwrap();
    let start_pos = grid::GridPos::new(0, start);
    let board = grid::Grid::from(board);
    (board, start_pos)
}

fn part_two_recursive(grid: &grid::Grid<char>, cache: &mut HashMap<GridPos, u64>, mut pos: grid::GridPos) -> u64 {
    let mut next = pos.position_in_dir(Direction::Down);
    while grid.is_valid_cell(&next) && *grid.get(&next).unwrap() == '.' {
        pos = next;
        next = pos.position_in_dir(Direction::Down);
    }
    if cache.contains_key(&pos) {
        return *cache.get(&pos).unwrap();
    }
    if !grid.is_valid_cell(&next) {
        cache.insert(pos, 1);
        return 1;
    }
    let left = part_two_recursive(grid, cache, pos.position_in_dir(Direction::DownLeft));
    let right = part_two_recursive(grid, cache, pos.position_in_dir(Direction::DownRight));
    cache.insert(pos, left + right);
    return left + right;
}

fn split_beam(grid: &mut grid::Grid<char>, row: usize, col: usize) -> Option<u64> {
    let mut splits = 0;
    let pos = grid::GridPos::new(row, col);
    if *grid.get(&pos.position_in_dir(Direction::Up))? == '|' {
        if *grid.get(&pos)? == '.' || *grid.get(&pos)? == '|' {
            *grid.get_mut(&pos)? = '|';
        } else {
            *grid.get_mut(&pos.position_in_dir(Direction::Left))? = '|';
            *grid.get_mut(&pos.position_in_dir(Direction::Right))? = '|';
            splits += 1;
        }
    }
    Some(splits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code_2025::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2025::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(40));
    }
}
