// Solving https://adventofcode.com/2024/day/1
use std::collections::HashMap;

use advent_of_code_2024::utils::parse;

advent_of_code_2024::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut list1, mut list2) = parse_input(input);
    list1.sort();
    list2.sort();
    let distances = list1
        .iter()
        .zip(list2)
        .map(|(x, y)| (*x - y).unsigned_abs())
        .reduce(|acc, e| acc + e)
        .unwrap();
    Some(distances)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (list1, list2) = parse_input(input);
    let occurrence_table = create_occurence_table(&list2);
    let mut similarity = 0;
    for num in list1 {
        similarity += num * occurrence_table.get(&num).unwrap_or(&0);
    }
    Some(similarity as u32)
}

fn create_occurence_table(lst: &[i32]) -> HashMap<i32, i32> {
    let mut table = HashMap::new();
    for num in lst {
        table.insert(*num, 1 + table.get(num).unwrap_or(&0));
    }
    table
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    parse::split_two_vertical_lists(
        input,
        |s| s.split_ascii_whitespace().collect(),
        |s| s.parse::<i32>().unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(31));
    }
}
