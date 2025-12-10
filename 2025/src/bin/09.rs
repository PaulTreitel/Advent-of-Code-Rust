use std::cmp::max;

use advent_of_code_2025::utils::{grid::{self, GridPos}, parse};
use num::abs;

// Solving https://adventofcode.com/2025/day/9
advent_of_code_2025::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let tiles = parse_input(input);
    let mut max_square = 0;
    for i in 0..tiles.len() {
        for j in 0..tiles.len() {
            max_square = max(max_square, square_area(tiles[i], tiles[j]));
        }
    }
    Some(max_square as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let tiles = parse_input(input);
    let green_segments = get_green_segments(&tiles);
    let mut max_square = 0;
    for i in 0..tiles.len() {
        for j in (i + 1)..tiles.len() {
            let mut valid_square = true;
            for segment in &green_segments {
                if segment_inside(tiles[i], tiles[j], segment) {
                    valid_square = false;
                    break;
                }
            }
            if valid_square {
                max_square = max(max_square, square_area(tiles[i], tiles[j]));
            }
        }
    }
    Some(max_square as u64)
}

fn parse_input(input: &str) -> Vec<GridPos> {
    parse::into_2d_array(input,
        |s| s.split(",").collect(),
        |s| s.parse::<usize>().unwrap())
        .iter().map(|vx| grid::GridPos::new(vx[0], vx[1]))
        .collect()
}

fn segment_inside(a: GridPos, b: GridPos, segment: &(GridPos, GridPos)) -> bool {
    let top = if a.row > b.row { a } else { b };
    let bottom = if top == a { b } else { a };
    let left = if b.col > a.col { a } else { b };
    let right = if left == a { b } else { a };
    let up_down_segment = if segment.0.row > segment.1.row { segment } else { &(segment.1, segment.0) };
    let left_right_segment = if segment.1.col > segment.0.col { segment } else { &(segment.1, segment.0) };
    if top.row < up_down_segment.0.row || bottom.row > up_down_segment.1.row {
        return false;
    }
    if left.col > left_right_segment.0.col || right.col < left_right_segment.1.col {
        return false;
    }
    if segment.0.row == segment.1.row && segment.0.row > bottom.row && segment.0.row < top.row {
        return true;
    }
    if segment.0.col == segment.1.col && segment.0.col > left.col && segment.0.col < right.col {
        return true;
    }
    false
}

fn get_green_segments(tiles: &Vec<GridPos>) -> Vec<(GridPos, GridPos)> {
    let mut segments = vec![];
    segments.push((tiles[0], tiles[tiles.len() - 1]));
    for i in 0..(tiles.len() - 1) {
        segments.push((tiles[i], tiles[i+1]));
    }
    segments
}

fn square_area(a: GridPos, b: GridPos) -> i64 {
    abs(a.row as i64 - b.row as i64 + 1) * abs(a.col as i64 - b.col as i64 + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code_2025::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2025::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(24));
    }
}
