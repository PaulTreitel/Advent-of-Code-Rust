use advent_of_code_2024::utils::parse;

advent_of_code_2024::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let reports = parse_input(input);
    let sum = reports.iter().filter(|x| report_safe(x)).map(|_| 1).sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = parse_input(input);
    let sum = reports
        .iter()
        .filter(|x| report_safe(x) || could_be_safe(x))
        .map(|_| 1)
        .sum();
    Some(sum)
}

fn could_be_safe(report: &[i32]) -> bool {
    for i in 0..report.len() {
        let mut new_report = report.to_vec();
        new_report.remove(i);
        if report_safe(&new_report) {
            return true;
        }
    }
    false
}

fn report_safe(report: &[i32]) -> bool {
    let mut inc_safe = true;
    let mut dec_safe = true;
    for i in 1..report.len() {
        let left = report.get(i - 1).unwrap();
        let right = report.get(i).unwrap();
        if (left - right).abs() == 0 || (left - right).abs() > 3 {
            return false;
        }
        if left > right {
            inc_safe = false;
        }
        if right > left {
            dec_safe = false;
        }
    }
    inc_safe || dec_safe
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    parse::into_2d_array(
        input,
        |x| x.split_whitespace().collect(),
        |x| x.parse::<i32>().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(4));
    }
}
