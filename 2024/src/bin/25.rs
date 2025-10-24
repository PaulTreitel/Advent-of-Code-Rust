// Solving https://adventofcode.com/2024/day/25
use advent_of_code_2024::utils::{grid::Grid, parse};

advent_of_code_2024::solution!(25);

pub fn part_one(input: &str) -> Option<u64> {
    let (locks, keys) = parse_input(input);
    let mut matches = 0;
    for lock in locks {
        for key in &keys {
            let mut is_match = true;
            for i in 0..5 {
                if lock[i] + key[i] > 5 {
                    is_match = false;
                    break;
                }
            }
            if is_match {
                matches += 1;
            }
        }
    }

    Some(matches)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn parse_input(input: &str) -> (Vec<[i32; 5]>, Vec<[i32; 5]>) {
    let mut locks = vec![];
    let mut keys = vec![];
    let input = input.split("\n\n");
    for lock_key in input {
        let (lock_key, is_lock) = get_key_lock(lock_key);
        if is_lock {
            locks.push(lock_key);
        } else {
            keys.push(lock_key);
        }
    }
    (locks, keys)
}

fn get_key_lock(lock_key: &str) -> ([i32; 5], bool) {
    let lock_key = parse::into_2d_array(
        lock_key,
        parse::split_by_all_chars,
        |s| s.chars().into_iter().next().unwrap()
    );
    let is_lock = lock_key[0][0] == '#';
    let mut heights = [0; 5];
    for i in 0..5 {
        let col_ct = lock_key.iter()
            .map(|x| *x.iter().nth(i).unwrap())
            .filter(|c| *c == '#')
            .count();
        heights[i] = col_ct as i32 - 1;

    }
    (heights, is_lock)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, None);
    }
}
