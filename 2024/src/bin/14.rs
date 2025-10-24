// Solving https://adventofcode.com/2024/day/14
use std::{cmp::Ordering, collections::HashSet};

use regex::Regex;

advent_of_code_2024::solution!(14);

const EXAMPLE_DIMENSIONS: (i64, i64) = (11, 7);
const PROBLEM_DIMENSIONS: (i64, i64) = (101, 103);
const PART_ONE_SECONDS: i64 = 100;

pub fn part_one(input: &str) -> Option<u64> {
    let dimensions = get_dimensions(input);
    let robots = parse_input(input);
    let final_positions = run_robot_sim(&robots, dimensions, PART_ONE_SECONDS);
    Some(compute_safety_factor(&final_positions, dimensions))
}

pub fn part_two(input: &str) -> Option<u64> {
    let dimensions = get_dimensions(input);
    let mut robots = parse_input(input);
    let mut i = 0;
    loop {
        robots = robots
            .iter()
            .map(|((xpos, ypos), (xvel, yvel))| {
                (((*xpos + *xvel).rem_euclid(dimensions.0), (*ypos + *yvel).rem_euclid(dimensions.1)),
                (*xvel, *yvel))
            })
            .collect();
        i += 1;
        let positions: Vec<(i64, i64)> = robots.iter().map(|(x, _)| *x).collect();
        let p2: HashSet<(i64, i64)> = HashSet::from_iter(positions.clone());
        if positions.len() == p2.len() {
            pretty_print_robots(&positions, dimensions);
            println!("");
            break;
        }
    }
    Some(i)
}

fn parse_input(input: &str) -> Vec<((i64, i64), (i64, i64))> {
    let mut robots = vec![];
    let re = Regex::new("(p=)|( v=)|,").unwrap();
    for l in input.lines() {
        let mut l = re.split(l);
        l.next();
        let xpos: i64 = l.next().unwrap().parse().unwrap();
        let ypos: i64 = l.next().unwrap().parse().unwrap();
        let xvel: i64 = l.next().unwrap().parse().unwrap();
        let yvel: i64 = l.next().unwrap().parse().unwrap();
        robots.push(((xpos, ypos), (xvel, yvel)));
    }
    robots
}

fn compute_safety_factor(robot_positions: &Vec<(i64, i64)>, dim: (i64, i64)) -> u64 {
    let mut quadrant_sums = vec![0; 4];
    let col_threshold = dim.0 / 2;
    let row_threshold = dim.1 / 2;
    for robot in robot_positions {
        match (robot.0.cmp(&col_threshold), robot.1.cmp(&row_threshold)) {
            (Ordering::Less, Ordering::Less) => quadrant_sums[0] += 1,
            (Ordering::Less, Ordering::Greater) => quadrant_sums[1] += 1,
            (Ordering::Greater, Ordering::Less) => quadrant_sums[2] += 1,
            (Ordering::Greater, Ordering::Greater) => quadrant_sums[3] += 1,
            _ => ()
        }
    }
    quadrant_sums.iter().product()
}

fn run_robot_sim(
    robots: &Vec<((i64, i64), (i64, i64))>,
    dim: (i64, i64),
    duration: i64
) -> Vec<(i64, i64)> {
    let mut positions = vec![];
    for (start, velocity) in robots {
        let x_pos = (start.0 + velocity.0 * duration).rem_euclid(dim.0);
        let y_pos = (start.1 + velocity.1 * duration).rem_euclid(dim.1);
        positions.push((x_pos, y_pos));
    }

    positions
}

fn get_dimensions(input: &str) -> (i64, i64) {
    if input.len() < 1000 {
        EXAMPLE_DIMENSIONS
    } else {
        PROBLEM_DIMENSIONS
    }
}

fn pretty_print_robots(robot_positions: &Vec<(i64, i64)>, dim: (i64, i64)) {
    for row in 0..dim.0 {
        for col in 0..dim.1 {
            if robot_positions.contains(&(row, col)) {
                print!("X");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(1));
    }
}
