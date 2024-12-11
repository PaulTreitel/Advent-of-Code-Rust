advent_of_code_2022::solution!(24);

use std::collections::HashSet;



#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn part_one(input: &str) -> Option<i32> {
    let (blizzs, bounds) = get_blizzards_bounds(input);
    let (iters, _) = cross_valley(blizzs, bounds, (-1, 0), (bounds.0, bounds.1 - 1));
    Some(iters)
}

pub fn part_two(input: &str) -> Option<i32> {
    let (blizzs, bounds) = get_blizzards_bounds(input);
    let (start, end) = ((-1, 0), (bounds.0, bounds.1 - 1));
    let (iter1, blizzs) = cross_valley(blizzs, bounds, start, end);
    let (iter2, blizzs) = cross_valley(blizzs, bounds, end, start);
    let (iter3, _) = cross_valley(blizzs, bounds, start, end);
    Some(iter1 + iter2 + iter3)
}

fn cross_valley(
    start_blizzs: HashSet<(i32, i32, Direction)>,
    bounds: (i32, i32),
    start: (i32, i32),
    end: (i32, i32)
) -> (i32, HashSet<(i32, i32, Direction)>) {
    let mut blizzs = start_blizzs.clone();
    let mut states = HashSet::new();
    states.insert(start);
    let mut iterations = 0;
    loop {
        blizzs = update_blizzards(&blizzs, &bounds);
        let mut new_states = HashSet::new();
        for pos in states {
            if is_empty(&blizzs, pos) {
                new_states.insert(pos);
            }
            if is_empty(&blizzs, (pos.0 - 1, pos.1))
                && valid_tile(&bounds, (pos.0 - 1, pos.1)) {
                if (pos.0 - 1, pos.1) == end {
                    return (iterations + 1, blizzs);
                }
                new_states.insert((pos.0 - 1, pos.1));
            }
            if is_empty(&blizzs, (pos.0 + 1, pos.1))
                && valid_tile(&bounds, (pos.0 + 1, pos.1)) {
                if (pos.0 + 1, pos.1) == end {
                    return (iterations + 1, blizzs);
                }
                new_states.insert((pos.0 + 1, pos.1));
            }
            if is_empty(&blizzs, (pos.0, pos.1 - 1))
                && valid_tile(&bounds, (pos.0, pos.1 - 1)) {
                new_states.insert((pos.0, pos.1 - 1));
            }
            if is_empty(&blizzs, (pos.0, pos.1 + 1))
                && valid_tile(&bounds, (pos.0, pos.1 + 1)) {
                new_states.insert((pos.0, pos.1 + 1));
            }
        }
        states = new_states;
        iterations += 1;
    }
}

fn valid_tile(bounds: &(i32, i32), pos: (i32, i32)) -> bool {
    if pos == (-1, 0) || pos == (bounds.0, bounds.1 - 1) {
        true
    } else {
        pos.0 >= 0 && pos.0 < bounds.0 && pos.1 >= 0 && pos.1 < bounds.1
    }
}

fn is_empty(blizzards: &HashSet<(i32, i32, Direction)>, pos: (i32, i32)) -> bool {
    !(
        blizzards.contains(&(pos.0, pos.1, Direction::Up))
        || blizzards.contains(&(pos.0, pos.1, Direction::Right))
        || blizzards.contains(&(pos.0, pos.1, Direction::Down))
        || blizzards.contains(&(pos.0, pos.1, Direction::Left))
    )
}

fn update_blizzards(
    blizzards: &HashSet<(i32, i32, Direction)>,
    bounds: &(i32, i32)
) -> HashSet<(i32, i32, Direction)> {
    let mut new_blizzards = HashSet::new();
    for blizz in blizzards {
        match blizz.2 {
            Direction::Up => {
                if blizz.0 == 0 {
                    new_blizzards.insert((bounds.0 - 1, blizz.1, blizz.2));
                } else {
                    new_blizzards.insert((blizz.0 - 1, blizz.1, blizz.2));
                }
            },
            Direction::Down => {
                if blizz.0 == bounds.0 - 1 {
                    new_blizzards.insert((0, blizz.1, blizz.2));
                } else {
                    new_blizzards.insert((blizz.0 + 1, blizz.1, blizz.2));
                }
            },
            Direction::Left => {
                if blizz.1 == 0 {
                    new_blizzards.insert((blizz.0, bounds.1 - 1, blizz.2));
                } else {
                    new_blizzards.insert((blizz.0, blizz.1 - 1, blizz.2));
                }
            },
            Direction::Right => {
                if blizz.1 == bounds.1 - 1 {
                    new_blizzards.insert((blizz.0, 0, blizz.2));
                } else {
                    new_blizzards.insert((blizz.0, blizz.1 + 1, blizz.2));
                }
            },
        };
    }

    new_blizzards
}

fn get_blizzards_bounds(input: &str) -> (HashSet<(i32, i32, Direction)>, (i32, i32)) {
    let mut blizzards = HashSet::new();
    let mut rows = 0;
    let mut cols = 0;
    for line in input.lines() {
        if cols == 0 {
            cols = line.len() as i32;
            continue;
        }
        let mut line = line.chars();
        line.next();
        line.next_back();
        let mut col_idx = 0;
        for c in line {
            match c {
                '#' => {
                    break;
                },
                '^' => {
                    blizzards.insert((rows, col_idx, Direction::Up));
                },
                '>' => {
                    blizzards.insert((rows, col_idx, Direction::Right));
                },
                'v' => {
                    blizzards.insert((rows, col_idx, Direction::Down));
                },
                '<' => {
                    blizzards.insert((rows, col_idx, Direction::Left));
                },
                _ => (),
            }
            col_idx += 1;
        }
        rows += 1;
    }
    (blizzards, (rows - 1, cols - 2))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(54));
    }
}
