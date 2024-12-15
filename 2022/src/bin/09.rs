use advent_of_code_2022::utils::direction::Direction;

advent_of_code_2022::solution!(9);

type Position = (i32, i32);

struct Move {
    direction: Direction,
    num_steps: i32,
}

pub fn part_one(input: &str) -> Option<i32> {
    let moves = get_move_list(input);
    let mut current_position = vec![(0, 0), (0, 0)];
    run_moves(&mut current_position, moves)
}

fn run_moves(current_position: &mut Vec<Position>, moves: Vec<Move>) -> Option<i32> {
    let mut locations: Vec<Position> = vec![(0, 0)];
    for m in moves {
        let mut new_locations = get_locations_from_move(current_position, m);
        locations.append(&mut new_locations);
    }
    locations.sort();
    locations.dedup();
    Some(locations.len() as i32)
}

fn get_locations_from_move(pos: &mut Vec<Position>, m: Move) -> Vec<Position> {
    let mut result: Vec<Position> = vec![];
    for _ in 0..m.num_steps {
        let dir = m.direction.to_offset();
        pos.get_mut(0).unwrap().0 += dir.0;
        pos.get_mut(0).unwrap().1 += dir.1;
        if let Some(new_result) = update_rope(pos) {
            result.push(new_result)
        }
    }
    result
}

fn update_rope(pos: &mut Vec<Position>) -> Option<Position> {
    let tmp = pos.len();
    let mut head_knot = &mut pos.first().unwrap().clone();
    let mut count = 0;
    for x in pos {
        if count == 0 {
            count += 1;
            continue;
        }
        if count == tmp - 1 {
            return update_knot(head_knot, x);
        }
        update_knot(head_knot, x);
        head_knot = x;
        count += 1;
    }
    None
}

fn update_knot(start: &Position, end: &mut Position) -> Option<Position> {
    let diffs = get_position_diff(*start, *end);
    if diffs.0.abs() <= 1 && diffs.1.abs() <= 1 {
        return None;
    }
    if diffs.0 == 0 {
        end.1 += diffs.1 / 2;
    } else if diffs.1 == 0 {
        end.0 += diffs.0 / 2;
    } else {
        let pos_change = (diffs.0 / diffs.0.abs(), diffs.1 / diffs.1.abs());
        *end = (end.0 + pos_change.0, end.1 + pos_change.1);
    }
    Some(*end)
}

fn get_position_diff(start: Position, end: Position) -> Position {
    ((start.0 - end.0), (start.1 - end.1))
}

pub fn part_two(input: &str) -> Option<i32> {
    let moves = get_move_list(input);
    let mut current_position = vec![(0, 0); 10];
    run_moves(&mut current_position, moves)
}

fn get_move_list(input: &str) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    for line in input.lines() {
        let mut line = line.split_ascii_whitespace();
        let dir = direction_from_letter(line.next().unwrap()).unwrap();
        let steps: i32 = line.next().unwrap().parse().unwrap();
        let new_move = Move {
            direction: dir,
            num_steps: steps,
        };
        moves.push(new_move);
    }
    moves
}

fn direction_from_letter(letter: &str) -> Option<Direction> {
    let letter = letter.chars().next().unwrap();
    if letter == 'R' {
        return Some(Direction::Right);
    } else if letter == 'U' {
        return Some(Direction::Down);
    } else if letter == 'D' {
        return Some(Direction::Up);
    } else if letter == 'L' {
        return Some(Direction::Left);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(1));
    }
}
