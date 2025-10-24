// Solving https://adventofcode.com/2022/day/22
advent_of_code_2022::solution!(22);

use std::{cmp::max, collections::{HashMap, HashSet}};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum SideName {
    Top,
    Bottom,
    East,
    West,
    North,
    South,
}

struct ContextPart1 {
    edges: HashMap<(i32, i32), HashSet<(i32, i32)>>,
    start: (i32, i32),
    moves: Vec<i32>,
    turns: Vec<char>,
}

struct ContextPart2 {
    edges: HashMap<(i32, i32), HashSet<(i32, i32, Option<Direction>)>>,
    start: (i32, i32),
    moves: Vec<i32>,
    turns: Vec<char>,
}

pub fn part_one(input: &str) -> Option<i32> {
    let ctx = get_graph(input);
    let mut curr_pos = ctx.start;
    let mut curr_dir = Direction::Right;

    for i in 0..ctx.moves.len() {
        make_move(&ctx, &mut curr_pos, &curr_dir, ctx.moves.get(i).unwrap());
        if i < ctx.turns.len() {
            curr_dir = new_direction(curr_dir, *ctx.turns.get(i).unwrap());
        }
    }
    Some((curr_pos.0 + 1) * 1000 + (curr_pos.1 + 1) * 4 + get_dir_val(&curr_dir))
}

pub fn part_two(input: &str) -> Option<i32> {
    let ctx = get_graph_part2(input);
    let mut curr_pos = ctx.start;
    let mut curr_dir = Direction::Right;

    for i in 0..ctx.moves.len() {
        make_move_part2(&ctx, &mut curr_pos, &mut curr_dir, ctx.moves.get(i).unwrap());
        if i < ctx.turns.len() {
            curr_dir = new_direction(curr_dir, *ctx.turns.get(i).unwrap());
        }
    }
    Some((curr_pos.0 + 1) * 1000 + (curr_pos.1 + 1) * 4 + get_dir_val(&curr_dir))
}

fn make_move_part2(
    ctx: &ContextPart2,
    pos: &mut (i32, i32),
    dir: &mut Direction,
    mv: &i32
) -> () {
    for _ in 0..*mv {
        let neighbors = ctx.edges.get(pos).unwrap();
        for (r, c, side_dir) in neighbors {
            match *dir {
                Direction::Left => {
                    if *r == pos.0 && *c == pos.1 - 1 {
                        *pos = (*r, *c);
                        break;
                    } else if (r - pos.0).abs() + (c - pos.1).abs() > 1 {
                        *pos = (*r, *c);
                        *dir = <Option<Direction> as Clone>::clone(&side_dir).unwrap();
                        break;
                    }
                },
                Direction::Right => {
                    if *r == pos.0 && *c == pos.1 + 1 {
                        *pos = (*r, *c);
                        break;
                    } else if (r - pos.0).abs() + (c - pos.1).abs() > 1 {
                        *pos = (*r, *c);
                        *dir = <Option<Direction> as Clone>::clone(&side_dir).unwrap();
                        break;
                    }
                },
                Direction::Up => {
                    if *r == pos.0 - 1 && *c == pos.1 {
                        *pos = (*r, *c);
                        break;
                    } else if (r - pos.0).abs() + (c - pos.1).abs() > 1 {
                        *pos = (*r, *c);
                        *dir = <Option<Direction> as Clone>::clone(&side_dir).unwrap();
                        break;
                    }
                },
                Direction::Down => {
                    if *r == pos.0 + 1 && *c == pos.1 {
                        *pos = (*r, *c);
                        break;
                    } else if (r - pos.0).abs() + (c - pos.1).abs() > 1 {
                        *pos = (*r, *c);
                        *dir = <Option<Direction> as Clone>::clone(&side_dir).unwrap();
                        break;
                    }
                },
            }
        }
    }
}

fn get_graph_part2(input: &str) -> ContextPart2 {
    let mut points = HashSet::new();
    let mut edges = HashMap::new();
    let mut start = (0, 0);

    let mut input: Vec<&str> = input.lines().collect();
    let (moves, turns) = get_instructions(input.pop().unwrap());
    input.pop().unwrap();
    let char_vecs: Vec<Vec<char>> = input.iter()
        .map(|l| l.chars().collect())
        .collect();

    get_points_start(&char_vecs, &mut points, &mut start);
    add_normal_edges_part2(&mut edges, &points);
    add_cube_edges(&mut edges, &points, &char_vecs);
    ContextPart2 { edges: edges, start: start, moves: moves, turns: turns }
}

fn add_cube_edges(
    _edges: &mut HashMap<(i32, i32), HashSet<(i32, i32, Option<Direction>)>>,
    _points: &HashSet<(i32, i32)>,
    char_vecs: &Vec<Vec<char>>
) -> () {
    let (sides, _side_names, step) = get_sides(&char_vecs);


    for side in sides.clone() {
        let neighbors = [
            (side.0 - step, side.1), (side.0 + step, side.1),
            (side.0, side.1 - step), (side.0, side.1 + step)
        ];
        for neighbor in neighbors {
            if sides.contains(&neighbor) {
                continue;
            }
            todo!("Not Done Yet: add_cube_edges");
        }
    }
}

fn add_normal_edges_part2(
    edges: &mut HashMap<(i32, i32), HashSet<(i32, i32, Option<Direction>)>>,
    points: &HashSet<(i32, i32)>
) -> () {
    for (x, y) in points.clone() {
        let mut neighbors = HashSet::new();
        if points.contains(&(x + 1, y)) {
            neighbors.insert((x + 1, y, None));
        }
        if points.contains(&(x - 1, y)) {
            neighbors.insert((x - 1, y, None));
        }
        if points.contains(&(x, y + 1)) {
            neighbors.insert((x, y + 1, None));
        }
        if points.contains(&(x, y - 1)) {
            neighbors.insert((x, y - 1, None));
        }
        edges.insert((x, y), neighbors);
    }
}

fn get_sides(
    char_vecs: &Vec<Vec<char>>
) -> (
    HashSet<(i32, i32)>,
    HashMap<(i32, i32), SideName>,
    i32
) {
    let step = {
        if char_vecs.get(0).unwrap().len() > 40 {
            50
        } else {
            5
        }
    };
    let mut side_corners = HashSet::new();
    // side_names maps sides to human names, for my own sanity
    let mut side_names = HashMap::new();

    for row_idx in (0..char_vecs.len()).step_by(step) {
        let row = char_vecs.get(row_idx).unwrap();
        for col_idx in (0..row.len()).step_by(step) {
            if *row.get(col_idx).unwrap() != ' ' {
                if side_corners.len() == 0 {
                    side_names.insert((row_idx as i32, col_idx as i32), SideName::North);
                }
                side_corners.insert((row_idx as i32, col_idx as i32));
            }
        }
    }
    fill_in_names(&side_corners, &mut side_names, step as i32);

    (side_corners, side_names, step as i32)
}

fn fill_in_names(
    side_corners: &HashSet<(i32, i32)>,
    side_names: &mut HashMap<(i32, i32), SideName>,
    step: i32
) -> () {
    let mut used_names = HashSet::new();
    while side_names.len() < 5 {
        for corner in side_corners {
            if side_names.contains_key(corner) {
                continue;
            }

            let above = (corner.0 - step, corner.1);
            let left = (corner.0, corner.1 - step);
            let right = (corner.0, corner.1 + step);
            if let Some(above_name) = side_names.get(&above) {
                if *above_name == SideName::North {
                    side_names.insert(*corner, SideName::Top);
                    used_names.insert(SideName::Top);
                } else if *above_name == SideName::Top {
                    side_names.insert(*corner, SideName::South);
                    used_names.insert(SideName::South);
                } else if *above_name == SideName::South {
                    side_names.insert(*corner, SideName::Bottom);
                    used_names.insert(SideName::Bottom);
                }
            } else if let Some(left_name) = side_names.get(&left) {
                if *left_name == SideName::North {
                    side_names.insert(*corner, SideName::East);
                    used_names.insert(SideName::East);
                } else if *left_name == SideName::Top {
                    side_names.insert(*corner, SideName::East);
                    used_names.insert(SideName::East);
                } else if *left_name == SideName::South {
                    side_names.insert(*corner, SideName::East);
                    used_names.insert(SideName::East);
                }
            } else if let Some(right_name) = side_names.get(&right) {
                if *right_name == SideName::Top {
                    side_names.insert(*corner, SideName::West);
                    used_names.insert(SideName::West);
                } else if *right_name == SideName::South {
                    side_names.insert(*corner, SideName::West);
                    used_names.insert(SideName::West);
                }
            }
        }
    }

    for corner in side_corners {
        if !side_names.contains_key(corner) {
            for name in [SideName::East, SideName::West, SideName::South, SideName::Top, SideName::Bottom] {
                if !used_names.contains(&name) {
                    side_names.insert(*corner, name);
                }
            }
        }
    }
}

fn get_dir_val(dir: &Direction) -> i32 {
    match dir {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
    }
}

fn make_move(
    ctx: &ContextPart1,
    pos: &mut (i32, i32),
    dir: &Direction,
    mv: &i32
) -> () {
    for _ in 0..*mv {
        let neighbors = ctx.edges.get(pos).unwrap();
        let mut moved = false;
        for (r, c) in neighbors {
            match *dir {
                Direction::Left => {
                    if *r == pos.0 && *c == pos.1 - 1 {
                        *pos = (*r, *c);
                        moved = true;
                        break;
                    } else if *r == pos.0 && *c > pos.1 + 1 {
                        *pos = (*r, *c);
                        moved = true;
                        break;
                    }
                },
                Direction::Right => {
                    if *r == pos.0 && *c == pos.1 + 1 {
                        *pos = (*r, *c);
                        moved = true;
                        break;
                    } else if *r == pos.0 && *c < pos.1 - 1 {
                        *pos = (*r, *c);
                        moved = true;
                        break;
                    }
                },
                Direction::Up => {
                    if *r == pos.0 - 1 && *c == pos.1 {
                        *pos = (*r, *c);
                        moved = true;
                        break;
                    } else if *r > pos.0 + 1 && *c == pos.1 {
                        *pos = (*r, *c);
                        moved = true;
                        break;
                    }
                },
                Direction::Down => {
                    if *r == pos.0 + 1 && *c == pos.1 {
                        *pos = (*r, *c);
                        moved = true;
                        break;
                    } else if *r < pos.0 - 1 && *c == pos.1 {
                        *pos = (*r, *c);
                        moved = true;
                        break;
                    }
                },
            }
        }
        if !moved {
            break;
        }
    }
}

fn new_direction(dir: Direction, turn: char) -> Direction {
    if turn == 'L' {
        match dir {
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
        }
    } else if turn == 'R' {
        match dir {
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
        }
    } else {
        unreachable!("turn is neither left nor right");
    }
}

fn get_graph(input: &str) -> ContextPart1 {
    let mut points = HashSet::new();
    let mut edges = HashMap::new();
    let mut start = (0, 0);

    let mut input: Vec<&str> = input.lines().collect();
    let (moves, turns) = get_instructions(input.pop().unwrap());
    input.pop().unwrap();
    let char_vecs: Vec<Vec<char>> = input.iter()
        .map(|l| l.chars().collect())
        .collect();

    get_points_start(&char_vecs, &mut points, &mut start);
    add_normal_edges(&mut edges, &points);
    add_wraparound_edges(&char_vecs, &mut edges);
    ContextPart1 { edges: edges, start: start, moves: moves, turns: turns }
}

fn get_instructions(input: &str) -> (Vec<i32>, Vec<char>) {
    let input = input.to_string();
    let turns = input.split(|x: char| x.is_numeric())
        .filter(|x| x.len() > 0)
        .map(|x| x.chars().next().unwrap())
        .collect();
    let moves: Vec<&str> = input.split(|p: char| p.is_alphabetic())
        .collect();
    let moves = moves.iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    (moves, turns)
}

fn add_wraparound_edges(
    char_vecs: &Vec<Vec<char>>,
    edges: &mut HashMap<(i32, i32), HashSet<(i32, i32)>>
) -> () {
    let mut max_width = 0;
    for row_idx in 0..char_vecs.len() {
        let row = char_vecs.get(row_idx).unwrap();
        max_width = max(max_width, row.len());
        if *row.get(row.len() - 1).unwrap() == '#' {
            continue;
        }
        for col_idx in 0..row.len() {
            let cell = row.get(col_idx).unwrap();
            if *cell == '#' {
                break;
            } else if *cell == '.' {
                edges.get_mut(&(row_idx as i32, col_idx as i32)).unwrap()
                    .insert((row_idx as i32, row.len() as i32 - 1));
                edges.get_mut(&(row_idx as i32, row.len() as i32 - 1)).unwrap()
                    .insert((row_idx as i32, col_idx as i32));
                break;
            }
        }
    }

    for col_idx in 0..max_width {
        let mut col_start = (i32::MAX, i32::MAX);
        let mut col_end = (i32::MAX, i32::MAX);
        for row_idx in 0..char_vecs.len() {
            let row = char_vecs.get(row_idx).unwrap();
            if row.len() <= col_idx {
                continue;
            }
            let val = row.get(col_idx).unwrap();
            if *val != ' ' {
                if col_start == (i32::MAX, i32::MAX) {
                    if *val == '#' {
                        break;
                    }
                    col_start = (row_idx as i32, col_idx as i32);
                }
                col_end = (row_idx as i32, col_idx as i32);
            }
        }
        if col_end == (i32::MAX, i32::MAX) || col_start == (i32::MAX, i32::MAX) {
            continue;
        }
        let endpoint = char_vecs.get(col_end.0 as usize).unwrap()
            .get(col_end.1 as usize).unwrap();
        if *endpoint == '.' {
            edges.get_mut(&col_start).unwrap().insert(col_end);
            edges.get_mut(&col_end).unwrap().insert(col_start);
        }
    }
}

fn add_normal_edges(
    edges: &mut HashMap<(i32, i32), HashSet<(i32, i32)>>,
    points: &HashSet<(i32, i32)>
) -> () {
    for (x, y) in points.clone() {
        let mut neighbors = HashSet::new();
        if points.contains(&(x + 1, y)) {
            neighbors.insert((x + 1, y));
        }
        if points.contains(&(x - 1, y)) {
            neighbors.insert((x - 1, y));
        }
        if points.contains(&(x, y + 1)) {
            neighbors.insert((x, y + 1));
        }
        if points.contains(&(x, y - 1)) {
            neighbors.insert((x, y - 1));
        }
        edges.insert((x, y), neighbors);
    }
}

fn get_points_start(
    char_vecs: &Vec<Vec<char>>,
    points: &mut HashSet<(i32, i32)>,
    start: &mut (i32, i32)
) -> () {
    for row_idx in 0..char_vecs.len() {
        let row = char_vecs.get(row_idx).unwrap();
        for col_idx in 0..row.len() {
            if *row.get(col_idx).unwrap() == '.' {
                if points.len() == 0 {
                    *start = (row_idx as i32, col_idx as i32);
                }
                points.insert((row_idx as i32, col_idx as i32));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(6032));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(5031));
    }
}

