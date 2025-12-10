use std::vec;

use advent_of_code_2025::utils::parse;

// Solving https://adventofcode.com/2025/day/6
advent_of_code_2025::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let (numbers, operators) = part_one_parse_input(input);
    let mut answer_total = 0;
    for i in 0..numbers.len() {
        if operators[i] == '+' {
            answer_total += numbers[i].iter().sum::<u64>();
        } else {
            answer_total += numbers[i].iter().product::<u64>();
        }
    }
    Some(answer_total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (numbers, operators) = part_two_parse_input(input);
    let mut answer_total = 0;
    for i in 0..numbers.len() {
        if operators[i] == '+' {
            answer_total += numbers[i].iter().sum::<u64>();
        } else {
            answer_total += numbers[i].iter().product::<u64>();
        }
    }
    Some(answer_total)
}

fn part_two_parse_input(input: &str) -> (Vec<Vec<u64>>, Vec<char>) {
    let operators: Vec<char> = input.lines()
        .last()
        .unwrap()
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().next().unwrap())
        .collect();
    let cols = parse::lines_to_columns(input);
    let mut numbers = vec![];
    let mut curr_number = vec![];
    for i in 0..cols.len() {
        let str = &cols[i].as_bytes()[0..cols[i].len() - 1];
        let str: String = str.iter().map(|&x| x as char).collect();
        let str = str.trim();
        if str.len() == 0 {
            numbers.push(curr_number.clone());
            curr_number.clear();
        } else {
            curr_number.push(str.parse::<u64>().unwrap());
        }
    }
    numbers.push(curr_number);
    (numbers, operators)
}

fn part_one_parse_input(input: &str) -> (Vec<Vec<u64>>, Vec<char>) {
    let mut lists = parse::split_vertical_lists(input,
        |s| s.split(" ")
            .filter(|&s| !s.is_empty()).collect(),
        |&s| s.to_string());
    let mut operators = vec![];
    for list in &mut lists {
        operators.push(list.pop().unwrap().chars().next().unwrap());
    }
    let lists = lists.iter()
        .map(|sv| sv.iter()
            .map(|s| s.parse::<u64>().unwrap())
            .collect())
        .collect();
    (lists, operators)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code_2025::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2025::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(3263827));
    }
}
