use std::{cmp::{max, min}, collections::{HashMap, HashSet}};

use advent_of_code_2024::utils::{direction::Direction, grid::Grid, parse};

advent_of_code_2024::solution!(6);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]

enum MapCell {
    Empty,
    Obstacle,
    Visited,
    GuardPosition,
}

impl MapCell {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '^' => Self::GuardPosition,
            '#' => Self::Obstacle,
            'X' => Self::Visited,
            c => panic!("Not a map cell: {}", c),
        }
    }
}

fn move_in_dir(dir: &Direction, pos: (usize, usize)) -> (usize, usize) {
    (
        (pos.0 as i32 + dir.to_offset().0) as usize,
        (pos.1 as i32 + dir.to_offset().1) as usize
    )
}

fn track_guard_around_map(map: &mut Grid<MapCell>, start_pos: (usize, usize)
) -> HashSet<(usize, usize)> {
    let mut curr_pos = start_pos;
    let mut curr_direction = Direction::Up;
    let mut visited = HashSet::new();
    loop {
        *map.get_mut(curr_pos.0, curr_pos.1).unwrap() = MapCell::Visited;
        let new_pos = move_in_dir(&curr_direction, curr_pos);

        if !map.is_valid_cell(new_pos.0, new_pos.1) {
            return visited;
        }
        if map.get(new_pos.0, new_pos.1).unwrap() != &MapCell::Obstacle {
            curr_pos = new_pos;
            visited.insert(curr_pos);
        } else {
            curr_direction = curr_direction.turn_right();
        }
    }
}

fn is_loop(reached: &mut HashMap<(usize, usize), Vec<i32>>, pos: (usize, usize)) -> bool {
    if reached.get(&pos).is_none() {
        return false;
    }
    let v = reached.get_mut(&pos).unwrap();
    let mut gaps = Vec::new();
    for first_idx in 0..v.len() {
        for second_idx in 0..v.len() {
            if first_idx != second_idx {
                let first = v.get(first_idx).unwrap();
                let second = v.get(second_idx).unwrap();
                let first_idx = max(first_idx, second_idx);
                let second_idx = min(first, second);
                let new = (first_idx, second_idx, (first - second).abs());
                if !gaps.contains(&new) {
                    gaps.push(new);
                }
            }
        }
    }
    for first_idx in 0..gaps.len() {
        for second_idx in 0..gaps.len() {
            if first_idx != second_idx {
                let first = gaps.get(first_idx).unwrap();
                let second = gaps.get(second_idx).unwrap();
                if first.2 == second.2 {
                    return true;
                }
            }
        }
    }
    false
}

fn creates_loop(map: &mut Grid<MapCell>, start_pos: (usize, usize)) -> bool {
    let mut curr_pos = start_pos;
    let mut curr_direction = Direction::Up;
    let mut positions_reached: HashMap<(usize, usize), Vec<i32>> = HashMap::new();
    let mut iter = 0;
    loop {
        *map.get_mut(curr_pos.0, curr_pos.1).unwrap() = MapCell::Visited;
        let new_pos = move_in_dir(&curr_direction, curr_pos);

        if !map.is_valid_cell(new_pos.0, new_pos.1) {
            break;
        }
        if map.get(new_pos.0, new_pos.1).unwrap() != &MapCell::Obstacle {
            curr_pos = new_pos;
            if positions_reached.contains_key(&curr_pos) {
                positions_reached.get_mut(&curr_pos).unwrap().push(iter);
            } else {
                positions_reached.insert(curr_pos, vec![iter]);
            }
            if is_loop(&mut positions_reached, curr_pos) {
                return true;
            }
        } else {
            curr_direction = curr_direction.turn_right();
        }
        iter += 1;
    }
    false
}

fn find_obstacle_loop_positions(
    map: &mut Grid<MapCell>,
    start_pos: (usize, usize),
    visited: &HashSet<(usize, usize)>
) -> u32 {
    let mut ct = 0;
    for r in 0..map.rows() {
        for c in 0..map.cols() {
            if !visited.contains(&(r, c)) {
                continue;
            }
            let curr_cell = map.get(r, c).unwrap();
            if *curr_cell == MapCell::GuardPosition || *curr_cell == MapCell::Obstacle {
                continue;
            }
            *map.get_mut(r, c).unwrap() = MapCell::Obstacle;
            if creates_loop(map, start_pos) {
                ct += 1;
            }
            *map.get_mut(r, c).unwrap() = MapCell::Empty;
        }
    }
    ct
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = parse_input(input);
    let start_pos = map.index_of(&MapCell::GuardPosition).unwrap();
    track_guard_around_map(&mut map, start_pos);
    Some(map.count(|x| x == &MapCell::Visited) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = parse_input(input);
    let start_pos = map.index_of(&MapCell::GuardPosition).unwrap();
    let visited = track_guard_around_map(&mut map.clone(), start_pos);
    Some(find_obstacle_loop_positions(&mut map, start_pos, &visited))
}

fn parse_input(input: &str) -> Grid<MapCell> {
    let map = parse::into_2d_array(
        input,
        |s| s.split("").filter(|&s| !s.eq("")).collect(),
        |s| MapCell::from_char(s.chars().next().unwrap())
    );
    Grid::from(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(6));
    }
}
