use std::collections::HashSet;

use advent_of_code_2024::utils::{
    direction::Direction,
    grid::{Grid, GridPos},
    parse,
};

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

fn move_in_dir(dir: &Direction, pos: &GridPos) -> GridPos {
    GridPos::new(
        (pos.row as i32 + dir.to_offset().0) as usize,
        (pos.col as i32 + dir.to_offset().1) as usize,
    )
}

fn track_guard_around_map(map: &mut Grid<MapCell>, start_pos: &GridPos) {
    let mut curr_pos = *start_pos;
    let mut curr_direction = Direction::Up;
    loop {
        *map.get_mut(&curr_pos).unwrap() = MapCell::Visited;
        let new_pos = move_in_dir(&curr_direction, &curr_pos);

        if !map.is_valid_cell(&new_pos) {
            return;
        }
        if map.get(&new_pos).unwrap() != &MapCell::Obstacle {
            curr_pos = new_pos;
        } else {
            curr_direction.turn_right();
        }
    }
}

fn creates_loop(map: &mut Grid<MapCell>, start_pos: &GridPos) -> bool {
    let mut curr_pos = *start_pos;
    let mut curr_direction = Direction::Up;
    let mut positions_reached: HashSet<(GridPos, Direction)> = HashSet::new();
    loop {
        *map.get_mut(&curr_pos).unwrap() = MapCell::Visited;
        let new_pos = move_in_dir(&curr_direction, &curr_pos);

        if !map.is_valid_cell(&new_pos) {
            break;
        }
        if map.get(&new_pos).unwrap() != &MapCell::Obstacle {
            curr_pos = new_pos;
            if positions_reached.contains(&(curr_pos, curr_direction)) {
                return true;
            }
            positions_reached.insert((curr_pos, curr_direction));
        } else {
            curr_direction.turn_right();
        }
    }
    false
}

fn find_obstacle_loop_positions(
    map: &mut Grid<MapCell>,
    start_pos: &GridPos,
    visited: &HashSet<GridPos>,
) -> u32 {
    let mut ct = 0;
    for r in 0..map.rows() {
        for c in 0..map.cols() {
            let pos = GridPos::new(r, c);
            if !visited.contains(&pos) {
                continue;
            }
            let curr_cell = map.get(&pos).unwrap();
            if *curr_cell == MapCell::GuardPosition || *curr_cell == MapCell::Obstacle {
                continue;
            }
            *map.get_mut(&pos).unwrap() = MapCell::Obstacle;
            if creates_loop(map, start_pos) {
                ct += 1;
            }
            *map.get_mut(&pos).unwrap() = MapCell::Empty;
        }
    }
    ct
}

fn get_visited_set(map: &Grid<MapCell>) -> HashSet<GridPos> {
    map.iterate_by_rows()
        .filter(|(_, c)| *c == MapCell::Visited)
        .map(|(pos, _)| pos)
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = parse_input(input);
    let start_pos = map.index_of(&MapCell::GuardPosition).unwrap();
    track_guard_around_map(&mut map, &start_pos);
    Some(map.count(|x| x == &MapCell::Visited) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = parse_input(input);
    let start_pos = map.index_of(&MapCell::GuardPosition).unwrap();
    let mut new_map = map.clone();
    track_guard_around_map(&mut new_map, &start_pos);
    let visited = get_visited_set(&new_map);
    Some(find_obstacle_loop_positions(&mut map, &start_pos, &visited))
}

fn parse_input(input: &str) -> Grid<MapCell> {
    let map = parse::into_2d_array(input, parse::split_by_all_chars, |s| {
        MapCell::from_char(s.chars().next().unwrap())
    });
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
