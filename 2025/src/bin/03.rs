use std::collections::HashMap;

use advent_of_code_2025::utils::parse::split_by_all_chars;

// Solving https://adventofcode.com/2025/day/3
advent_of_code_2025::solution!(3);

const PART_TWO_NUM_BATTERIES_ON: u8 = 12;

pub fn part_one(input: &str) -> Option<u64> {
    let battery_banks = parse_input(input);
    let mut total_joltage = 0;
    for bank in battery_banks {
        let mut max_joltage = 0;
        for tens_idx in 0..bank.len() {
            for ones_idx in (tens_idx + 1)..bank.len() {
                let joltage = bank[tens_idx] * 10 + bank[ones_idx];
                max_joltage = std::cmp::max(max_joltage, joltage as u64);
            }
        }
        total_joltage += max_joltage;
    }
    Some(total_joltage)
}

pub fn part_two(input: &str) -> Option<u64> {
    let battery_banks = parse_input(input);
    let mut max_joltage = 0;
    for bank in &battery_banks {
        let mut indices = HashMap::new();
        for i in 0..bank.len() {
            let indexes = indices.entry(bank[i]).or_insert(vec![]);
            indexes.push(i);
        }
        let new_joltage = part_two_recursive(bank, &indices, 0, PART_TWO_NUM_BATTERIES_ON);
        max_joltage += new_joltage;
    }
    Some(max_joltage)
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines()
        .map(|s| split_by_all_chars(s).iter()
            .map(|&n| n.parse::<u8>().unwrap())
            .collect())
        .collect()
}

fn part_two_recursive(
    bank: &[u8],
    indices: &HashMap<u8, Vec<usize>>,
    start_idx: usize,
    digits_remaining: u8
) -> u64 {
    if digits_remaining == 0 || start_idx == bank.len() {
        return 0;
    }
    if digits_remaining == 1 {
        return *bank[start_idx..bank.len()].iter().max().unwrap() as u64;
    }
    let mut max_joltage = 0;
    for digit in (1..10).rev() {
        if !indices.contains_key(&digit) {
            continue;
        }
        let digit_indexes = &indices[&(digit as u8)];
        for &idx in digit_indexes {
            if idx < start_idx {
                continue;
            }
            let subvalue = part_two_recursive(bank, indices, idx + 1, digits_remaining - 1);
            if subvalue == 0 {
                continue;
            }
            let value = subvalue + place_valued(digit, digits_remaining);
            max_joltage = std::cmp::max(max_joltage, value);
            return max_joltage;
        }
    }
    0
}

fn place_valued(n: u8, place: u8) -> u64 {
    let mut place_value = n as u64;
    for _ in 1..place {
        place_value *= 10;
    }
    place_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code_2025::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2025::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(3121910778619));
    }
}
