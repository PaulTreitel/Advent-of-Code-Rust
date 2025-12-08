// Solving https://adventofcode.com/2025/day/5
advent_of_code_2025::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, ids) = parse_input(input);
    let mut num_fresh = 0;
    for id in ids {
        for range in &ranges {
            if range[0] <= id && range[1] >= id {
                num_fresh += 1;
                break;
            }
        }
    }
    Some(num_fresh)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut ranges, _) = parse_input(input);
    ranges.sort();
    ranges.reverse();
    let mut new_ranges = Vec::new();
    while !ranges.is_empty() {
        let mut new_range = ranges.pop().unwrap();
        while !ranges.is_empty() && ranges[ranges.len() - 1][0] <= new_range[1] {
            let tmp = ranges.pop().unwrap();
            new_range[1] = std::cmp::max(new_range[1], tmp[1]);
        }
        new_ranges.push(new_range);
    }
    let total_fresh_ids = new_ranges.iter()
        .map(|x| x[1] - x[0] + 1)
        .reduce(|acc, e| acc + e)
        .unwrap();
    Some(total_fresh_ids)
}

fn parse_input(input: &str) -> (Vec<Vec<u64>>, Vec<u64>) {
    let parts: Vec<_> = input.split("\n\n").collect();
    let fresh_ranges: Vec<_> = parts[0].lines()
        .map(|s| s.split("-")
            .into_iter()
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>())
        .collect();
    let ids: Vec<_> = parts[1].lines()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    (fresh_ranges, ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code_2025::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2025::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(14));
    }
}
