advent_of_code_2024::solution!(4);

const SEARCH_STR_PART_1: &str = "XMAS";
const SEARCH_STR_PART_2: &str = "MAS";

pub fn part_one(input: &str) -> Option<u32> {
    let word_search = parse_input(input);
    Some(get_matches_part_one(&word_search, SEARCH_STR_PART_1))
}

pub fn part_two(input: &str) -> Option<u32> {
    let word_search = parse_input(input);
    Some(get_diagonal_matches_part_two(&word_search, SEARCH_STR_PART_2))
}

fn get_diagonal_matches_part_two(word_search: &Vec<Vec<char>>, search_str: &str) -> u32 {
    let mut diagonal_count = 0;
    for row in 1..word_search.get(0).unwrap().len() - 1 {
        for col in 1..word_search.len() - 1 {
            if !dir_matches(word_search, row - 1, col - 1, 1, 1, search_str) {
                continue;
            }
            if !dir_matches(word_search, row - 1, col + 1, 1, -1, search_str) {
                continue;
            }
            diagonal_count += 1;
        }
    }
    diagonal_count
}

fn direction_valid(
    row: usize,
    col: usize,
    row_max: usize,
    col_max: usize,
    row_offset: i32,
    col_offset: i32,
    diag_len: usize
) -> bool {
    let row_min_fails = row_offset < 0 && row < diag_len;
    let row_max_fails = row_offset > 0 && row >= row_max - diag_len;
    let col_min_fails = col_offset < 0 && col < diag_len;
    let col_max_fails = col_offset > 0 && col >= col_max - diag_len;
    !(row_max_fails || row_min_fails) && !(col_max_fails || col_min_fails)
}

fn dir_matches(
    word_search: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    row_offset: i32,
    col_offset: i32,
    search_str: &str
) -> bool {
    let row_max = word_search.get(0).unwrap().len();
    let col_max = word_search.len();
    let diag_len = search_str.len() - 1;
    let search_target = search_str.chars().collect::<Vec<char>>();
    let mut search_target_reversed = search_target.clone();
    search_target_reversed.reverse();
    if !direction_valid(row, col, row_max, col_max, row_offset, col_offset, diag_len) {
        return false;
    }
    let mut search = Vec::new();
    for i in 0..search_target.len() as i32 {
        search.push(*word_search
            .get((row as i32 + row_offset * i) as usize)
            .unwrap()
            .get((col as i32 + col_offset * i) as usize)
            .unwrap()
        );
    }
    search_target.eq(&search) || search_target_reversed.eq(&search)
}

fn get_matches_part_one(word_search: &Vec<Vec<char>>, search_str: &str) -> u32 {
    let mut match_count = 0;
    for col in 0..word_search.get(0).unwrap().len() {
        for row in 0..word_search.len() {
            for (row_offset, col_offset) in [(0, 1), (1, 0), (1, -1), (1, 1)] {
                if dir_matches(word_search, row, col, row_offset, col_offset, search_str) {
                    match_count += 1;
                }
            }
        }
    }
    match_count
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect()
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
