advent_of_code_2025::solution!(1);
use advent_of_code_2025::utils::direction::Direction;
use num::abs;

const DIAL_MOD: i32 = 100;

pub fn part_one(input: &str) -> Option<u64> {
    let rotations = parse_input(input);
    let mut dial = 50;
    let mut zero_count = 0;
    for (dir, rot) in rotations {
        match dir {
            Direction::Left => dial -= rot,
            Direction::Right => dial += rot,
            _ => unreachable!()
        }
        dial = dial.rem_euclid(DIAL_MOD);
        if dial == 0 {
            zero_count += 1;
        }
    }
    Some(zero_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let rotations = parse_input(input);
    let mut dial: i32 = 50;
    let mut zero_count: u64 = 0;
    for (dir, mut rot) in rotations {
        match dir {
            Direction::Left => rot = -rot,
            Direction::Right => (),
            _ => unreachable!()
        }
        while abs(rot) >= 100 {
            if rot < 0 {
                rot += 100;
            } else {
                rot -= 100;
            }
            zero_count += 1;
        }
        if dial != 0 && (dial + rot > 100 || dial + rot < 0) {
            zero_count += 1;
        }
        dial += rot;
        dial = dial.rem_euclid(DIAL_MOD);
        if dial == 0 {
            zero_count += 1;
        }
    }
    Some(zero_count)
}

fn parse_input(input: &str) -> Vec<(Direction, i32)> {
    input.lines()
        .map(|s| (Direction::from_letter(&s[0..1]).unwrap(), s[1..].parse::<i32>().unwrap()))
        .collect()
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
        assert_eq!(result, Some(6));
    }
}
