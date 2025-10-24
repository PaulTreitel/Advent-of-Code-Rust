// Solving https://adventofcode.com/2024/day/11
use std::collections::HashMap;

advent_of_code_2024::solution!(11);

const PART_ONE_ROUNDS: i32 = 25;
const PART_TWO_ROUNDS: i32 = 75;

pub fn part_one(input: &str) -> Option<u64> {
    Some(run_sim(input, PART_ONE_ROUNDS))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(run_sim(input, PART_TWO_ROUNDS))
}

fn parse_input(input: &str) -> HashMap<u64, u64> {
    let mut stonemap = HashMap::new();
    let stones: Vec<u64> = input
        .trim()
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    for stone in stones {
        stonemap
            .entry(stone)
            .and_modify(|x| *x += 1)
            .or_insert(1);
    }
    stonemap
}

fn run_sim(input: &str, num_rounds: i32) -> u64 {
    let mut stones = parse_input(input);
    for _ in 0..num_rounds {
        stones = next_round(&stones);
    }
    let count = stones.values().sum();
    count
}

fn next_round(stones: &HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new = HashMap::new();
    for (&stone, &ct) in stones {
        let new_stones = get_new_stones(stone);
        for s in new_stones {
            new
                .entry(s)
                .and_modify(|x| *x += ct)
                .or_insert(ct);
        }
    }
    new
}

fn get_new_stones(stone: u64) -> Vec<u64> {
    let mut res = vec![];
    let str_stone = stone.to_string();
    if stone == 0 {
        res.push(1);
    } else if str_stone.len() % 2 == 0 {
        let div_point = str_stone.len() / 2;
        let first = &str_stone[..div_point];
        let second = &str_stone[div_point..];

        res.push(first.parse().unwrap());
        res.push(second.parse().unwrap());
    } else {
        res.push(stone * 2024);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, None);
    }
}
