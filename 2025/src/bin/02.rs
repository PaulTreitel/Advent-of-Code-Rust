// Solving https://adventofcode.com/2025/day/2
advent_of_code_2025::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = parse_input(input);
    let mut invalid_sum = 0;
    for range in ranges {
        for i in range[0]..range[1]+1 {
            let id = i.to_string();
            if id[..id.len()/2].eq(&id[id.len()/2..]) {
                invalid_sum += i;
            }
        }
    }
    Some(invalid_sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges = parse_input(input);
    let mut invalid_sum = 0;
    for range in ranges {
        for i in range[0]..range[1]+1 {
            let id = i.to_string();
            for block_len in 1..id.len()/2 + 1 {
                let num_blocks = id.len() / block_len;
                let block = &id[..block_len];
                if id.eq(&block.repeat(num_blocks)) {
                    invalid_sum += i;
                    break;
                }
            }
        }
    }
    Some(invalid_sum)
}

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    let ranges: Vec<&str> = input.split(",").collect();
    ranges.iter()
        .map(|&s| s.trim().split("-")
            .into_iter()
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code_2025::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2025::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(4174379265));
    }
}
