use advent_of_code_2022::utils::parse;
use regex::Regex;

advent_of_code_2022::solution!(4);

type Position = (i32, i32);

pub fn part_one(input: &str) -> Option<i32> {
    let mut num_fully_overlapping = 0;
    let elf_plans = parse_input(input);
    for (elf0, elf1) in elf_plans {
        if elf_range_fully_contains(elf0, elf1) {
            num_fully_overlapping += 1;
        }
    }
    Some(num_fully_overlapping)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut num_overlapping = 0;
    let elf_plans = parse_input(input);
    for (elf0, elf1) in elf_plans {
        if elf_range_overlaps(elf0, elf1) {
            num_overlapping += 1;
        }
    }
    Some(num_overlapping)
}

fn elf_range_fully_contains(elf1: Position, elf2: Position) -> bool {
    (elf1.0 <= elf2.0 && elf1.1 >= elf2.1) || (elf2.0 <= elf1.0 && elf2.1 >= elf1.1)
}

fn elf_range_overlaps(elf1: Position, elf2: Position) -> bool {
    !(elf1.1 < elf2.0 || elf1.0 > elf2.1) || (elf2.1 >= elf1.0 && elf2.0 <= elf1.1)
}

fn parse_input(input: &str) -> Vec<(Position, Position)> {
    let re = Regex::new(r"-|,").unwrap();
    parse::into_2d_array(
        input,
        |s| re.split(s).collect(),
        |s| s.parse::<i32>().unwrap(),
    )
    .iter()
    .map(|x| {
        (
            (*x.first().unwrap(), *x.get(1).unwrap()),
            (*x.get(2).unwrap(), *x.get(3).unwrap()),
        )
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(4));
    }
}
