// Solving https://adventofcode.com/2024/day/4
use advent_of_code_2024::utils::{
    direction::ALL_DOWN_DIRECTION,
    grid::{Grid, GridPos},
};

advent_of_code_2024::solution!(4);

const SEARCH_STR_PART_1: &str = "XMAS";
const SEARCH_STR_PART_2: &str = "MAS";

pub fn part_one(input: &str) -> Option<u32> {
    let word_search = parse_input(input);
    Some(get_matches_part_one(&word_search, SEARCH_STR_PART_1))
}

pub fn part_two(input: &str) -> Option<u32> {
    let word_search = parse_input(input);
    Some(get_diagonal_matches_part_two(
        &word_search,
        SEARCH_STR_PART_2,
    ))
}

fn get_diagonal_matches_part_two(grid: &Grid<char>, search_str: &str) -> u32 {
    let mut diagonal_count = 0;
    for (pos, _) in grid.iter_by_rows() {
        if pos.row == 0 || pos.col == 0 || pos.row == grid.rows() - 1 || pos.col == grid.cols() - 1
        {
            continue;
        }
        let diag_pos_1 = GridPos::new(pos.row - 1, pos.col - 1);
        let diag_pos_2 = GridPos::new(pos.row - 1, pos.col + 1);
        if !dir_matches(grid, &diag_pos_1, (1, 1), search_str) {
            continue;
        }
        if !dir_matches(grid, &diag_pos_2, (1, -1), search_str) {
            continue;
        }
        diagonal_count += 1;
    }
    diagonal_count
}

fn dir_matches(grid: &Grid<char>, pos: &GridPos, offset: (i32, i32), search_str: &str) -> bool {
    let search_target = search_str.chars().collect::<Vec<char>>();
    let mut search_target_reversed = search_target.clone();
    search_target_reversed.reverse();
    match grid.scan_direction(pos, offset, search_str.len()) {
        Some(search) => {
            let search: Vec<char> = search.iter().map(|(_, ch)| *ch).collect();
            search_target.eq(&search) || search_target_reversed.eq(&search)
        }
        None => false,
    }
}

fn get_matches_part_one(grid: &Grid<char>, search_str: &str) -> u32 {
    let mut match_count = 0;
    for (pos, _) in grid.iter_by_rows() {
        for offset in ALL_DOWN_DIRECTION {
            if dir_matches(grid, &pos, offset.to_offset(), search_str) {
                match_count += 1;
            }
        }
    }
    match_count
}

fn parse_input(input: &str) -> Grid<char> {
    let tmp: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();
    Grid::from(tmp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(9));
    }
}
