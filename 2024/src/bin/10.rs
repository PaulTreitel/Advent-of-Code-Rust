use std::collections::{HashMap, HashSet, VecDeque};

use advent_of_code_2024::utils::{direction::Direction, grid::{Grid, GridPos}, parse};

const TRAILHEAD: u32 = 0;
const TRAILEND: u32 = 9;

type TrailHeadMap = HashMap<GridPos, Vec<GridPos>>;

advent_of_code_2024::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_input(input);
    let mut trailheads = get_trailheads(&map);
    find_full_trails(&map, &mut trailheads);
    let trail_ct = trailheads
        .values_mut()
        .map(|x| {
            x.sort();
            x.dedup();
            x.len() as u32
        })
        .sum();
    Some(trail_ct)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse_input(input);
    let mut trailheads = get_trailheads(&map);
    find_full_trails(&map, &mut trailheads);
    let trail_ct = trailheads
        .values_mut()
        .map(|x| x.len() as u32)
        .sum();
    Some(trail_ct)
}

fn parse_input(input: &str) -> Grid<u32> {
    let grid = parse::into_2d_array(
        input,
        parse::split_by_all_chars,
        |s| s.parse::<u32>().unwrap());
    Grid::from(grid)
}

fn get_trailheads(map: &Grid<u32>) -> TrailHeadMap {
    HashMap::from_iter(map
        .iter_by_rows()
        .filter(|(_, val)| *val == TRAILHEAD)
        .map(|(pos, _)| (pos, vec![])))
}

fn find_full_trails(map: &Grid<u32>, trailheads: &mut TrailHeadMap) {
    for head in trailheads.clone().keys() {
        let endpoints = trailheads.get_mut(head).unwrap();
        trailhead_bfs(map, head, endpoints);
    }
}

fn trailhead_bfs(map: &Grid<u32>, head: &GridPos, ends: &mut Vec<GridPos>) {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back(*head);
    while let Some(pos) = queue.pop_front() {
        visited.insert(pos);
        let current_val = *map.get(&pos).unwrap();
        let new_positions = get_new_positions(&pos);
        for new_pos in new_positions {
            if !map.is_valid_cell(&new_pos) {
                continue;
            }
            if *map.get(&new_pos).unwrap() == current_val + 1 {
                if current_val + 1 == TRAILEND {
                    ends.push(new_pos);
                } else if !visited.contains(&new_pos) {
                    queue.push_back(new_pos);
                }
            }
        }
    }
}

fn get_new_positions(pos: &GridPos) -> Vec<GridPos> {
    let mut left = pos.clone();
    let mut right = pos.clone();
    let mut up = pos.clone();
    let mut down = pos.clone();
    left.move_in_dir(Direction::Left);
    right.move_in_dir(Direction::Right);
    up.move_in_dir(Direction::Up);
    down.move_in_dir(Direction::Down);

    vec![left, right, up, down]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(81));
    }
}
