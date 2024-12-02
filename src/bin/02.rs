advent_of_code::solution!(2);

enum LevelDirection {
    Inc,
    Dec,
}

pub fn part_one(input: &str) -> Option<u32> {
    let reports = parse_input(input);
    let mut safe_count = 0;
    for report in reports {
        if report_safe(&report, LevelDirection::Inc) {
            safe_count += 1;
        } else if report_safe(&report, LevelDirection::Dec) {
            safe_count += 1;
        }
    }
    Some(safe_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = parse_input(input);
    let mut safe_count = 0;
    for report in reports {
        if report_safe(&report, LevelDirection::Inc) {
            safe_count += 1;
        } else if report_safe(&report, LevelDirection::Dec) {
            safe_count += 1;
        } else {
            for i in 0..report.len() {
                let mut new_report = report.clone();
                new_report.remove(i);
                if report_safe(&new_report, LevelDirection::Inc) {
                    safe_count += 1;
                    break;
                } else if report_safe(&new_report, LevelDirection::Dec) {
                    safe_count += 1;
                    break;
                }
            }
        }
    }
    Some(safe_count)
}

fn report_safe(report: &Vec<i32>, direction: LevelDirection) -> bool {
    for i in 1..report.len() {
        let left = report.get(i - 1).unwrap();
        let right = report.get(i).unwrap();
        if (left - right).abs() == 0 || (left - right).abs() > 3 {
            return false;
        }
        match direction {
            LevelDirection::Inc => {
                if left > right {
                    return false;
                }
            },
            LevelDirection::Dec => {
                if right > left {
                    return false;
                }
            },
        }
    }
    true
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let mut reports = Vec::new();
    for line in input.lines() {
        let line: Vec<i32> = line.split_ascii_whitespace()
            .into_iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        reports.push(line);
    }
    reports
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
