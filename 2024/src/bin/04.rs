advent_of_code_2024::solution!(4);

const SEARCH_STR_PART_1: &str = "XMAS";
const SEARCH_STR_PART_2: &str = "MAS";

pub fn part_one(input: &str) -> Option<u32> {
    let mut xmas_count = 0;
    let word_search = parse_input(input);
    xmas_count += get_horizontal_matches(&word_search, SEARCH_STR_PART_1);
    xmas_count += get_vertical_matches(&word_search, SEARCH_STR_PART_1);
    xmas_count += get_diagonal_matches_part_one(&word_search, SEARCH_STR_PART_1);
    Some(xmas_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let word_search = parse_input(input);
    Some(get_diagonal_matches_part_two(&word_search, SEARCH_STR_PART_2))
}

fn get_diagonal_matches_part_two(word_search: &Vec<Vec<char>>, search_str: &str) -> u32 {
    let mut diagonal_count = 0;
    println!("{}", word_search.get(0).unwrap().len() - 1);
    for row in 1..word_search.get(0).unwrap().len() - 1 {
        for col in 1..word_search.len() - 1 {
            // println!("checking position ({}, {}) for top left -> bottom right diagonal", row, col);
            if !check_diag(word_search, row - 1, col - 1, 1, 1, search_str) {
                // println!("position ({}, {}) failed on top left -> bottom right diagonal", row, col);
                continue;
            }
            // println!("checking position ({}, {}) for top right -> bottom left diagonal", row, col);
            if !check_diag(word_search, row - 1, col + 1, 1, -1, search_str) {
                // println!("position ({}, {}) failed on top right -> bottom left diagonal", row, col);
                continue;
            }
            diagonal_count += 1;
        }
    }
    // println!("diagonal part 2 {}", diagonal_count);
    diagonal_count
}

fn get_diagonal_matches_part_one(word_search: &Vec<Vec<char>>, search_str: &str) -> u32 {
    let mut diagonal_count = 0;
    for row in 0..word_search.len() {
        for col in 0..word_search.len() {
            if check_diag(word_search, row, col, 1, -1, search_str) {
                diagonal_count += 1;
            }
            if check_diag(word_search, row, col, 1, 1, search_str) {
                diagonal_count += 1;
            }
        }
    }
    // println!("diagonal part 1 {}", diagonal_count);
    diagonal_count
}

fn diag_valid(
    row: usize,
    col: usize,
    row_max: usize,
    col_max: usize,
    row_offset: i32,
    col_offset: i32,
    diag_len: usize
) -> bool {
    if row_offset < 0 && row < diag_len {
        false
    } else if row_offset > 0 && row >= row_max - diag_len {
        false
    } else if col_offset < 0 && col < diag_len {
        false
    } else if col_offset > 0 && col >= col_max - diag_len {
        false
    } else {
        true
    }
}

fn check_diag(
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
    // println!("Check at ({},{}) with offsets ({},{})", row, col, row_offset, col_offset);
    if !diag_valid(row, col, row_max, col_max, row_offset, col_offset, diag_len) {
        return false;
    }
    let mut search = Vec::new();
    for i in 0..search_str.len() {
        search.push(*word_search
            .get((row as i32 + row_offset * i as i32) as usize)
            .unwrap()
            .get((col as i32 + col_offset * i as i32) as usize)
            .unwrap()
        );
    }
    let formed_str = form_str(&search, 0, search_str.len());
    let reversed: String = formed_str.chars().rev().collect();
    if formed_str.eq(search_str) || reversed.eq(search_str) {
        // println!("found diag at ({},{}) with offsets ({},{}): {}", row, col, row_offset, col_offset, formed_str);
        true
    } else {
    //     println!("unable to find diag at ({},{}) with offsets ({},{}): {}", row, col, row_offset, col_offset, formed_str);
        false
    }

}

fn form_str(search: &Vec<char>, start: usize, length: usize) -> String {
    let mut formed_str = String::new();
    for i in 0..length {
        formed_str.push(*search.get(start + i).unwrap());
    }
    formed_str
}

fn get_horizontal_matches(word_search: &Vec<Vec<char>>, search_str: &str) -> u32 {
    let mut horizontal_count = 0;
    for col in 0..word_search.len() {
        let search_col = word_search.get(col).unwrap();
        let row_limit = word_search.len() - search_str.len() + 1;
        for row in 0..row_limit {
            let formed_str = form_str(search_col, row, search_str.len());
            let reversed: String = formed_str.chars().rev().collect();
            if formed_str.eq(search_str) || reversed.eq(search_str) {
                horizontal_count += 1;
            }
        }
    }
    // println!("horizontal {}", horizontal_count);
    horizontal_count
}

fn get_vertical_matches(word_search: &Vec<Vec<char>>, search_str: &str) -> u32 {
    let mut vertical_count = 0;
    let col_len = word_search.get(0).unwrap().len();
    for row in 0..col_len {
        let search_row: Vec<char> = word_search
            .iter()
            .map(|x| *x.get(row).unwrap())
            .collect();
        let col_limit = col_len - search_str.len() + 1;
        for col in 0..col_limit {
            let formed_str = form_str(&search_row, col, search_str.len());
            let reversed: String = formed_str.chars().rev().collect();
            if formed_str.eq(search_str) || reversed.eq(search_str) {
                vertical_count += 1;
            }
        }
    }
    // println!("vertical {}", vertical_count);
    vertical_count
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut word_search = Vec::new();
    for line in input.lines() {
        let line: Vec<char> = line.chars().collect();
        word_search.push(line);
    }
    word_search
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
