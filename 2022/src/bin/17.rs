advent_of_code_2022::solution!(17);

use std::collections::{HashMap, HashSet};

#[derive(Debug, Hash, Eq, Clone)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq)]
enum RockType {
    Row,
    Cross,
    L,
    Col,
    Square
}

#[derive(Debug)]
enum MoveType {
    Left,
    Right
}

const ROCK_ORDER: [RockType; 5] = [RockType::Row, RockType::Cross, RockType::L, RockType::Col, RockType::Square];


pub fn part_one(input: &str) -> Option<i32> {
    let moves = get_move_sequence(input);
    let mut highest: [i64; 7] = [-1; 7];
    let mut move_idx: i32 = 0;
    let mut state: HashSet<Point> = HashSet::new();
    for i in 0..7 {
        state.insert(Point { x: i, y: -1 });
    }
    for i in 0..2022 {
        add_rock(i, &moves, &mut highest, &mut move_idx, &mut state);
    }
    Some(1 + *highest.iter().max_by(|x, y| x.cmp(y)).unwrap() as i32)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut target: i64 = 1_000_000_000_000;
    let moves = get_move_sequence(input);
    let mut highest = [-1; 7];
    let mut move_idx: i32 = 0;
    let mut state: HashSet<Point> = HashSet::new();
    let mut heights: HashMap<(i32, i32), (i64, i64, [i64; 7])> = HashMap::new();
    for i in 0..7 {
        state.insert(Point { x: i, y: -1 });
    }

    let mut start = 0;
    let mut pattern_iters: i64 = 0;
    let mut pattern_height_diff = 0;
    let mut base_iter = 0;
    let mut base_height = 0;
    for i in 0..5*moves.len() {
        add_rock(i as i64, &moves, &mut highest, &mut move_idx, &mut state);
        let curr_height = 1 + *highest.iter().max_by(|x, y| x.cmp(y)).unwrap();
        if heights.contains_key(&(move_idx, (i % 5) as i32)) {
            let old = heights.get(&(move_idx, (i % 5) as i32)).unwrap();
            if !heights_match(&highest, &old.2) {
                continue;
            }
            pattern_iters = i as i64 - old.0;
            pattern_height_diff = curr_height - old.1;
            base_iter = old.0;
            base_height = old.1 - 1;
            start = i + 1;
            break;
        }
        heights.insert((move_idx, (i % 5) as i32), (i as i64, curr_height, highest));
    }

    target -= base_iter;
    let estimate = pattern_height_diff * (target / pattern_iters as i64) + base_height;
    let curr_height = highest.iter().max_by(|x, y| x.cmp(y)).unwrap().clone();

    for i in start as i64..start as i64 + target % pattern_iters as i64 {
        add_rock(i as i64, &moves, &mut highest, &mut move_idx, &mut state);
    }
    let final_height = *highest.iter().max_by(|x, y| x.cmp(y)).unwrap();
    println!("base iteration {}, base height {}, {} iterations in pattern, pattern height {}, {} repeats, {} left, {} est",
        base_iter,
        base_height,
        pattern_iters,
        pattern_height_diff,
        target / pattern_iters as i64,
        target % pattern_iters as i64,
        estimate);
    println!("curr_height {}, final_height {}", curr_height, final_height);
    Some(estimate + (final_height - curr_height))
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn heights_match(current: &[i64; 7], old: &[i64; 7]) -> bool {
    let current = current.clone();
    let old = old.clone();
    let diffs: Vec<i64> = current.clone()
        .iter()
        .zip(old.iter())
        .map(|(x, y)| x - y)
        .collect();
    let base_diff = diffs.get(0).unwrap();
    for i in 1..7 {
        if diffs.get(i).unwrap() != base_diff {
            return false;
        }
    }
    true
}

fn add_rock(
    iter: i64,
    moves: &Vec<MoveType>,
    highest: &mut [i64; 7],
    move_idx: &mut i32,
    state: &mut HashSet<Point>,
) -> () {
    let rocktype = ROCK_ORDER.get(iter as usize % 5).unwrap();
    let bottom_pos = 4 + *highest.iter().max_by(|x, y| x.cmp(y)).unwrap();
    let mut rock = new_rock(bottom_pos, rocktype);
    loop {
        let curr_move = moves.get(*move_idx as usize).unwrap();
        *move_idx = (*move_idx + 1) % moves.len() as i32;
        let mut new_rock: HashSet<Point>;
        match *curr_move {
            MoveType::Left => {
                new_rock = rock.iter()
                    .map(|p| Point { x: p.x - 1, y: p.y })
                    .filter(|p| p.x >= 0 && p.x < 7)
                    .collect();
            }
            MoveType::Right => {
                new_rock = rock.iter()
                    .map(|p| Point { x: p.x + 1, y: p.y})
                    .filter(|p| p.x >= 0 && p.x < 7)
                    .collect();
            }
        }
        let intersect: HashSet<&Point> = state.intersection(&new_rock).collect();
        if new_rock.len() == rock.len() && intersect.len() == 0 {
            rock = new_rock;
        }
        new_rock = rock.iter()
            .map(|p| Point { x: p.x, y: p.y - 1 })
            .collect();
        let intersect: HashSet<&Point> = state.intersection(&new_rock).collect();
        if intersect.len() == 0 {
            rock = new_rock;
        } else {
            for p in rock.clone() {
                if *highest.get(p.x as usize).unwrap() < p.y {
                    *highest.get_mut(p.x as usize).unwrap() = p.y;
                }
            }
            state.extend(rock);
            break;
        }
    }
}

fn new_rock(bottom_pos: i64, rock: &RockType) -> HashSet<Point> {
    let mut new_rock = HashSet::new();
    match *rock {
        RockType::Row => {
            for i in 2..6 {
                new_rock.insert(Point { x: i, y: bottom_pos });
            }
        },
        RockType::Col => {
            for i in 0..4 {
                new_rock.insert(Point { x: 2, y: bottom_pos + i });
            }
        },
        RockType::L => {
            for i in 2..5 {
                new_rock.insert(Point { x: i, y: bottom_pos });
            }
            new_rock.insert(Point { x: 4, y: bottom_pos + 1 });
            new_rock.insert(Point { x: 4, y: bottom_pos + 2 });
        },
        RockType::Cross => {
            new_rock.insert(Point { x: 3, y: bottom_pos });
            new_rock.insert(Point { x: 3, y: bottom_pos + 1 });
            new_rock.insert(Point { x: 3, y: bottom_pos + 2 });
            new_rock.insert(Point { x: 4, y: bottom_pos + 1 });
            new_rock.insert(Point { x: 2, y: bottom_pos + 1 });
        },
        RockType::Square => {
            new_rock.insert(Point { x: 2, y: bottom_pos });
            new_rock.insert(Point { x: 3, y: bottom_pos });
            new_rock.insert(Point { x: 2, y: bottom_pos + 1 });
            new_rock.insert(Point { x: 3, y: bottom_pos + 1 });
        },
    }
    new_rock
}

fn get_move_sequence(input: &str) -> Vec<MoveType> {
    let mut moves = Vec::new();
    for ch in input.chars() {
        if ch == '<' {
            moves.push(MoveType::Left);
        } else if ch == '>' {
            moves.push(MoveType::Right)
        }
    }
    moves
}

fn main() {
    let input = advent_of_code_2022::template::read_file("inputs", DAY);
    let res = part_one(&input).unwrap();
    println!("{}", res);
    let res: i64 = part_two(&input).unwrap();
    println!("{}", res);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(3068)); // fill in
    }
    #[test]
    fn test_part_two() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(1514285714288)); // fill in
    }
}
