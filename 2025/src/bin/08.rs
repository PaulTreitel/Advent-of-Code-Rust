
use std::{collections::HashSet, fmt::Display};

use advent_of_code_2025::utils::parse;

// Solving https://adventofcode.com/2025/day/8
advent_of_code_2025::solution!(8);

const PART_ONE_TEST_ITERS: u64 = 10;
const PART_ONE_REAL_ITERS: u64 = 1000;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        return f64::powf(
            num::pow(self.x - other.x, 2) as f64
            + num::pow(self.y - other.y, 2) as f64
            + num::pow(self.z - other.z, 2) as f64,
            0.5);
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {}, {})", self.x, self.y, self.z))
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let boxes = parse_input(input);
    let mut min_dists = get_min_dists(&boxes);
    let mut connected = HashSet::new();
    let mut circuits: Vec<HashSet<&Point>> = vec![];
    for _ in 0..PART_ONE_TEST_ITERS {
        connect_boxes(&mut min_dists, &mut connected, &mut circuits);
    }
    let mut sizes = circuits.iter().map(|f| f.len()).collect::<Vec<usize>>();
    sizes.sort_by(|x, y| y.cmp(x));
    Some((sizes[0] * sizes[1] * sizes[2]) as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let boxes = parse_input(input);
    let mut min_dists = get_min_dists(&boxes);
    let mut connected = HashSet::new();
    let mut circuits = vec![];
    loop {
        let next = min_dists[min_dists.len() - 1];
        connect_boxes(&mut min_dists, &mut connected, &mut circuits);
        if circuits.len() == 1 && circuits[0].len() == boxes.len() {
            return Some((next.0.x * next.1.x) as u64);
        }
    }
}

fn parse_input(input: &str) -> Vec<Point> {
    parse::into_2d_array(input,
        |s| s.split(",").collect(),
    |&s| s.parse::<i64>().unwrap())
        .iter().map(|v| Point { x: v[0], y: v[1], z: v[2]})
        .collect()
}

fn connect_boxes<'a>(
    min_dists: &mut Vec<(&'a Point, &'a Point, f64)>,
    connected: &mut HashSet<(&'a Point, &'a Point)>,
    circuits: &mut Vec<HashSet<&'a Point>>
) {
    let next = min_dists.pop().unwrap();
    connected.insert((next.0, next.1));
    let mut added = -1;
    for idx in (0..circuits.len()).rev() {
        if circuits[idx].contains(next.0) && circuits[idx].contains(next.1) {
            added = -2;
            break;
        }
        if circuits[idx].contains(next.0) || circuits[idx].contains(next.1) {
            if added == -1  {
                added = idx as i32;
                circuits[idx].extend([next.0, next.1]);
            } else {
                let tmp = circuits[idx].clone();
                circuits[added as usize].extend(tmp);
                circuits.remove(idx);
            }
        }
    }
    if added == -1 {
        circuits.push(HashSet::from([next.0, next.1]));
    }
}

fn get_min_dists(boxes: &[Point]) -> Vec<(&Point, &Point, f64)> {
    let mut min_dists = vec![];
    let mut added = HashSet::new();
    for i in 0..boxes.len() {
        for j in 0..boxes.len() {
            if i != j && !added.contains(&(i, j)) && !added.contains(&(j, i)) {
                min_dists.push((&boxes[i], &boxes[j], boxes[i].distance(&boxes[j])));
                added.insert((i, j));
            }
        }
    }
    min_dists.sort_by(|x, y| y.2.total_cmp(&x.2));
    min_dists
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code_2025::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2025::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(25272));
    }
}
