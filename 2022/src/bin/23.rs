advent_of_code_2022::solution!(23);

use std::collections::{HashMap, HashSet};

use advent_of_code_2022::utils::direction::MapDirection;

const NUM_ROUNDS: i32 = 10;
const DIRECTION_ORDER: [MapDirection; 4] = [
    MapDirection::North,
    MapDirection::South,
    MapDirection::West,
    MapDirection::East
];

pub fn part_one(input: &str) -> Option<i32> {
    let mut dir_order = DIRECTION_ORDER.clone().to_vec();
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
    let mut dir_order = DIRECTION_ORDER.clone().to_vec();
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
    dir_order: &Vec<MapDirection>,
    elves: &HashSet<(i32, i32)>
) -> HashMap<(i32, i32), Vec<(i32, i32)>> {
    let mut moves: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
    for elf in elves {
        if !should_move(elves, elf) {
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
                continue;
            }

            moves
                .entry(proposed_cell)
                .and_modify(|x| x.push(*elf))
                .or_insert(vec![*elf]);
            break;
        }
    }
    moves
}

fn propose_check_cells(
    dir: &MapDirection,
    elf: &(i32, i32)
) -> ((i32, i32), Vec<(i32, i32)>) {
    let check_cells;
    let proposed_cell;
    match dir {
        MapDirection::North => {
            check_cells = vec![
                (elf.0 - 1, elf.1 - 1),
                (elf.0 - 1, elf.1),
                (elf.0 - 1, elf.1 + 1)
            ];
            proposed_cell = (elf.0 - 1, elf.1);
        },
        MapDirection::South => {
            check_cells = vec![
                (elf.0 + 1, elf.1 - 1),
                (elf.0 + 1, elf.1),
                (elf.0 + 1, elf.1 + 1)
            ];
            proposed_cell = (elf.0 + 1, elf.1);
        },
        MapDirection::West => {
            check_cells = vec![
                (elf.0 - 1, elf.1 - 1),
                (elf.0, elf.1 - 1),
                (elf.0 + 1, elf.1 - 1)
            ];
            proposed_cell = (elf.0, elf.1 - 1);
        },
        MapDirection::East => {
            check_cells = vec![
                (elf.0 - 1, elf.1 + 1),
                (elf.0, elf.1 + 1),
                (elf.0 + 1, elf.1 + 1)
            ];
            proposed_cell = (elf.0, elf.1 + 1);
        },
        _ => unreachable!()
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
