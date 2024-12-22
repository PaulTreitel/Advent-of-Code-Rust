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
        end_secrets_sum += new_secret;
    }
    Some(end_secrets_sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn parse_input(input: &str) -> Vec<u64> {
    input.lines().map(|s| s.parse::<u64>().unwrap()).collect()
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
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, None);
    }
}
