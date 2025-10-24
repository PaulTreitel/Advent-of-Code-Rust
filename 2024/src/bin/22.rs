// Solving https://adventofcode.com/2024/day/22
use std::collections::{HashMap, HashSet};

advent_of_code_2024::solution!(22);

const MODULUS: u64 = 16777216;
const STEP_1_CONST: u64 = 64;
const STEP_2_CONST: u64 = 32;
const STEP_3_CONST: u64 = 2048;
const ITERATIONS: u64 = 2000;


pub fn part_one(input: &str) -> Option<u64> {
    let mut end_secrets_sum = 0;
    let start_secrets = parse_input(input);
    for secret in  start_secrets {
        let mut new_secret = secret;
        for _ in 0..ITERATIONS {
            new_secret = compute_new_secret_number(new_secret);
        }
        end_secrets_sum += new_secret as u64;
    }
    Some(end_secrets_sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let start_secrets = parse_input(input);
    let secret_sequences = get_secret_sequences(&start_secrets);
    let real_prices = get_real_prices(&secret_sequences);
    let price_changes = get_price_changes(&real_prices);

    let mut max_bananas = 0;
    let mut all_price_diff_results = Vec::with_capacity(start_secrets.len());
    for seq_idx in 0..secret_sequences.len() {
        let mut price_diff_results: HashMap<[i8; 4], u64> = HashMap::with_capacity(2000);
        add_price_diffs(&real_prices[seq_idx], &price_changes[seq_idx], &mut price_diff_results);
        all_price_diff_results.push(price_diff_results);
    }

    let relevant_windows = get_relevant_windows(&all_price_diff_results);
    for window in relevant_windows {
        let mut window_bananas = 0;
        for price_seqence in &all_price_diff_results {
            window_bananas += price_seqence.get(&window).unwrap_or(&0);
        }
        max_bananas = std::cmp::max(max_bananas, window_bananas)
    }
    Some(max_bananas)
}

fn parse_input(input: &str) -> Vec<u64> {
    input.lines().map(|s| s.parse::<u64>().unwrap()).collect()
}

fn get_relevant_windows(
    price_diff_results: &Vec<HashMap<[i8; 4], u64>>
) -> HashSet<[i8; 4]> {
    let mut all_price_change_windows = HashSet::new();
    for res in price_diff_results {
        all_price_change_windows.extend(res.keys());
    }
    all_price_change_windows
}

fn add_price_diffs(
    real_prices: &Vec<i8>,
    price_changes: &Vec<i8>,
    price_diff_results: &mut HashMap<[i8; 4], u64>
) {
    for change_idx in 0..price_changes.len() - 1 {
        let window_start = std::cmp::max(0,change_idx as i64 - 4) as usize;
        let window = &price_changes[window_start..change_idx];
        if window.len() != 4 {
            continue;
        }
        // Makes the window length a compile time constant for type safety
        let window = [window[0], window[1], window[2], window[3]];
        if !price_diff_results.contains_key(&window) {
            // 1 to offset the fact that the first secret doesn't have an
            // associated price change
            let bananas_bought = real_prices[change_idx] as u64;
            price_diff_results.insert(window, bananas_bought);
        }
    }
}

fn get_real_prices(secret_sequences: &Vec<Vec<i64>>) -> Vec<Vec<i8>> {
    let mut all_real_prices = vec![];
    for sequence in secret_sequences {
        let mut seq_real_prices = vec![];
        for v in sequence {
            let tmp = v.to_string();
            seq_real_prices.push(tmp[tmp.len() - 1..].parse::<i8>().unwrap());
        }
        all_real_prices.push(seq_real_prices);
    }
    all_real_prices
}

fn get_price_changes(real_prices: &Vec<Vec<i8>>) -> Vec<Vec<i8>> {
    let mut all_price_changes = vec![];
    for sequence in real_prices {
        let mut sequence_changes = vec![];
        for i in 0..sequence.len() - 1 {
            sequence_changes.push(sequence[i + 1] - sequence[i]);
        }
        all_price_changes.push(sequence_changes);
    }
    all_price_changes
}

fn get_secret_sequences(start_secrets: &Vec<u64>) -> Vec<Vec<i64>> {
    let mut all_secret_sequences = vec![];
    for &secret in start_secrets {
        let mut new_secret_sequence = vec![secret as i64];
        let mut curr_secret = secret;
        for _ in 0..ITERATIONS {
            curr_secret = compute_new_secret_number(curr_secret);
            new_secret_sequence.push(curr_secret as i64);
        }
        all_secret_sequences.push(new_secret_sequence);
    }
    all_secret_sequences
}

fn compute_new_secret_number(secret: u64) -> u64{
    let step_1 = mix_and_prune(secret, secret * STEP_1_CONST);
    let step_2 = mix_and_prune(step_1, step_1 / STEP_2_CONST);
    let step_3 = mix_and_prune(step_2,  step_2 * STEP_3_CONST);
    step_3
}

fn mix_and_prune(secret: u64, new: u64) -> u64 {
    (secret ^ new) % MODULUS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2024::template::read_file_part("examples", DAY, 2);
        let result = part_two(&input);
        assert_eq!(result, Some(23));
    }
}
