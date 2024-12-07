use advent_of_code_2024::utils::parse;
use regex::Regex;

advent_of_code_2024::solution!(7);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Calibration {
    Number(u64),
    Add,
    Mul,
    Concat,
}

fn eval(eqn: &Vec<Calibration>) -> Option<u64> {
    if let Calibration::Number(x) = eqn.first().unwrap() {
        if let Calibration::Number(y) = eqn.last().unwrap() {
            match eqn.get(1).unwrap() {
                Calibration::Number(_) => unreachable!(),
                Calibration::Add => return Some(x + y),
                Calibration::Mul => return Some(x * y),
                Calibration::Concat => {
                    let mut newstr = x.to_string();
                    newstr.push_str(&y.to_string());
                    return Some(newstr.parse().unwrap());
                },
            }
        }
        return None
    }
    None
}

fn can_be_equal(total: u64, nums: &[u64], operations: &Vec<Calibration>) -> bool {
    let mut eqn_values: Vec<u64> = nums.to_vec();
    for num in &nums[1..] {
        let mut new_eqn_values = Vec::new();
        for subtotal in eqn_values {
            for op in operations {
                let new_op_eqn = vec![
                    Calibration::Number(subtotal),
                    op.clone(),
                    Calibration::Number(*num)];
                let new_val = eval(&new_op_eqn).unwrap();
                if new_val <= total {
                    new_eqn_values.push(new_val);
                }
            }
        }
        eqn_values = new_eqn_values;
    }
    for val in eqn_values {
        if val == total {
            return true;
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let (totals, nums) = parse_input(input);
    let mut calibration_result = 0;
    for i in 0..totals.len() {
        let total = *totals.get(i).unwrap();
        let nums = nums.get(i).unwrap();
        let operations = vec![Calibration::Mul, Calibration::Add];
        if can_be_equal(total, nums, &operations) {
            calibration_result += total;
        }
    }
    Some(calibration_result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (totals, nums) = parse_input(input);
    let mut calibration_result = 0;
    for i in 0..totals.len() {
        let total = *totals.get(i).unwrap();
        let nums = nums.get(i).unwrap();
        let operations = vec![Calibration::Mul, Calibration::Add, Calibration::Concat];
        if can_be_equal(total, nums, &operations) {
            calibration_result += total;
        }
    }
    Some(calibration_result)
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<Vec<u64>>) {
    let re = Regex::new("(: )| ").unwrap();
    let mut nums = parse::into_2d_array(
        input,
        |s| re.split(s).collect(),
        |s| s.parse::<u64>().unwrap());
    let mut totals = Vec::new();
    for row_idx in 0..nums.len() {
        totals.push(nums.get_mut(row_idx).unwrap().remove(0));
    }
    (totals, nums)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(11387));
    }
}
