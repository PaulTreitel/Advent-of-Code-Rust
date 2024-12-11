advent_of_code_2022::solution!(23);

use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    North,
    South,
    West,
    East,
}

const NUM_ROUNDS: i32 = 10;

pub fn part_one(input: &str) -> Option<i32> {
    let mut dir_order = vec![
        Direction::North, Direction::South, Direction::West, Direction::East
    ];
    let mut elves = get_elves(input);
    for _ in 0..NUM_ROUNDS {
        let proposed_moves = get_moves(&dir_order, &elves);
        make_moves(&proposed_moves, &mut elves);
        let tmp = dir_order.remove(0);
        dir_order.push(tmp);
    }

    let (smallest, greatest) = get_corners(&elves);
    Some((greatest.0 - smallest.0 + 1) * (greatest.1 - smallest.1 + 1) - elves.len() as i32)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut dir_order = vec![
        Direction::North, Direction::South, Direction::West, Direction::East
    ];
    let mut elves = get_elves(input);
    let mut num_rounds = 0;
    loop {
        let proposed_moves = get_moves(&dir_order, &elves);
        let num_moves = make_moves(&proposed_moves, &mut elves);
        let tmp = dir_order.remove(0);
        dir_order.push(tmp);
        num_rounds += 1;
        if num_moves == 0 {
            break;
        }
    }
    Some(num_rounds)
}

fn get_corners(elves: &HashSet<(i32, i32)>) -> ((i32, i32), (i32, i32)) {
    let mut smallest = (i32::MAX, i32::MAX);
    let mut greatest = (i32::MIN, i32::MIN);

    for elf in elves {
        if elf.0 < smallest.0 {
            smallest = (elf.0, smallest.1);
        }
        if elf.1 < smallest.1 {
            smallest = (smallest.0, elf.1);
        }
        if elf.0 > greatest.0 {
            greatest = (elf.0, greatest.1);
        }
        if elf.1 > greatest.1 {
            greatest = (greatest.0, elf.1);
        }
    }
    (smallest, greatest)
}

fn make_moves(moves: &HashMap<(i32, i32), Vec<(i32, i32)>>, elves: &mut HashSet<(i32, i32)>) -> i32 {
    let mut moves_made = 0;
    for mv in moves {
        if mv.1.len() > 1 {
            continue;
        }
        elves.remove(mv.1.first().unwrap());
        elves.insert(*mv.0);
        moves_made += 1
    }
    moves_made
}

fn get_moves(
    dir_order: &Vec<Direction>,
    elves: &HashSet<(i32, i32)>
) -> HashMap<(i32, i32), Vec<(i32, i32)>> {
    let mut moves: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
    // let mut proposed_spaces = HashMap::new();
    for elf in elves {
        if !should_move(elves, elf) {
            // println!("elf {:?} shouldn't move", elf);
            continue;
        }

        for dir in dir_order {
            let (proposed_cell, check_cells) = propose_check_cells(dir, elf);
            let mut can_propose = true;
            for cell in check_cells {
                if elves.contains(&cell) {
                    can_propose = false;
                    break;
                }
            }
            if !can_propose {
                // println!("elf {:?} can't propose to go {:?}", elf, dir);
                continue;
            }
            if let std::collections::hash_map::Entry::Vacant(e) = moves.entry(proposed_cell) {
                e.insert(vec![*elf]);
                break;
            } else {
                moves.get_mut(&proposed_cell).unwrap().push(*elf);
                break;
            }
        }
    }
    moves
}

fn propose_check_cells(
    dir: &Direction,
    elf: &(i32, i32)
) -> ((i32, i32), Vec<(i32, i32)>) {
    let mut check_cells = Vec::new();
    let proposed_cell;
    match dir {
        Direction::North => {
            check_cells.extend([
                (elf.0 - 1, elf.1 - 1), (elf.0 - 1, elf.1), (elf.0 - 1, elf.1 + 1)
            ]);
            proposed_cell = (elf.0 - 1, elf.1);
        },
        Direction::South => {
            check_cells.extend([
                (elf.0 + 1, elf.1 - 1), (elf.0 + 1, elf.1), (elf.0 + 1, elf.1 + 1)
            ]);
            proposed_cell = (elf.0 + 1, elf.1);
        },
        Direction::West => {
            check_cells.extend([
                (elf.0 - 1, elf.1 - 1), (elf.0, elf.1 - 1), (elf.0 + 1, elf.1 - 1)
            ]);
            proposed_cell = (elf.0, elf.1 - 1);
        },
        Direction::East => {
            check_cells.extend([
                (elf.0 - 1, elf.1 + 1), (elf.0, elf.1 + 1), (elf.0 + 1, elf.1 + 1)
            ]);
            proposed_cell = (elf.0, elf.1 + 1);
        },
    };
    (proposed_cell, check_cells)
}

fn should_move(elves: &HashSet<(i32, i32)>, elf: &(i32, i32)) -> bool {
    let mut should_move = false;
    for rmod in [-1, 0, 1] {
        for cmod in [-1, 0, 1] {
            if (rmod != 0 || cmod != 0)
                && elves.contains(&(elf.0 + rmod, elf.1 + cmod)
            ) {
                should_move = true;
                break;
            }
        }
    }
    should_move
}

fn get_elves(input: &str) -> HashSet<(i32, i32)> {
    let mut elves = HashSet::new();
    let mut row = 0;
    for line in input.lines() {
        let line: Vec<char> = line.chars().collect();
        for col in 0..line.len() {
            if *line.get(col).unwrap() == '#' {
                elves.insert((row, col as i32));
            }
        }
        row += 1;
    }
    elves
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(110));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(20));
    }
}
