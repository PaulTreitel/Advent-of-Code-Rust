use advent_of_code_2024::utils::grid::{Grid, ALL_DOWN_OFFSETS};

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
    for ((row, col), _) in grid.iterate_by_rows() {
        if row == 0 || col == 0 || row == grid.rows() - 1 || col == grid.cols() - 1 {
            continue;
        }
        if !dir_matches(grid, row - 1, col - 1, (1, 1), search_str) {
            continue;
        }
        if !dir_matches(grid, row - 1, col + 1, (1, -1), search_str) {
            continue;
        }
        diagonal_count += 1;
    }
    diagonal_count
}

fn dir_matches(
    grid: &Grid<char>,
    row: usize,
    col: usize,
    offset: (i32, i32),
    search_str: &str,
) -> bool {
    let search_target = search_str.chars().collect::<Vec<char>>();
    let mut search_target_reversed = search_target.clone();
    search_target_reversed.reverse();
    match grid.directional_scan(row, col, offset, search_str.len()) {
        Some(search) => search_target.eq(&search) || search_target_reversed.eq(&search),
        None => false,
    }
}

fn get_matches_part_one(grid: &Grid<char>, search_str: &str) -> u32 {
    let mut match_count = 0;
    for ((row, col), _) in grid.iterate_by_rows() {
        for offset in ALL_DOWN_OFFSETS {
            if dir_matches(grid, row, col, offset, search_str) {
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
