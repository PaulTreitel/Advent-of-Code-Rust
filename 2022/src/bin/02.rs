advent_of_code_2022::solution!(2);

const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;
const WIN_SCORE: i32 = 6;
const DRAW_SCORE: i32 = 3;

pub enum RpsResult {
    Win,
    Draw,
    Lose
}

fn get_round_points(my_move: i32, their_move: i32) -> Option<i32> {
    match my_move {
        ROCK => match their_move {
            ROCK => Some(ROCK + DRAW_SCORE),
            PAPER => Some(ROCK),
            SCISSORS => Some(ROCK + WIN_SCORE),
            _ => None
        },
        PAPER => match their_move {
            ROCK => Some(PAPER + WIN_SCORE),
            PAPER => Some(PAPER + DRAW_SCORE),
            SCISSORS => Some(PAPER),
            _ => None
        },
        SCISSORS => match their_move {
            ROCK => Some(SCISSORS),
            PAPER => Some(SCISSORS + WIN_SCORE),
            SCISSORS => Some(SCISSORS + DRAW_SCORE),
            _ => None
        },
        _ => None
    }
}

fn strategy_to_int(ch: char) -> Option<i32> {
    match ch.to_ascii_uppercase() {
        'A' => Some(ROCK),
        'B' => Some(PAPER),
        'C' => Some(SCISSORS),
        'X' => Some(ROCK),
        'Y' => Some(PAPER),
        'Z' => Some(SCISSORS),
        _ => None
    }
}

fn char_to_result(ch: char) -> Option<RpsResult> {
    match ch {
        'X' => Some(RpsResult::Lose),
        'Y' => Some(RpsResult::Draw),
        'Z' => Some(RpsResult::Win),
        _ => None
    }
}

fn get_move(their_move: i32, outcome: RpsResult) -> Option<i32> {
    match their_move {
        ROCK => match outcome {
            RpsResult::Win => Some(PAPER),
            RpsResult::Draw => Some(ROCK),
            RpsResult::Lose => Some(SCISSORS)
        }
        PAPER => match outcome {
            RpsResult::Win => Some(SCISSORS),
            RpsResult::Draw => Some(PAPER),
            RpsResult::Lose => Some(ROCK)
        }
        SCISSORS => match outcome {
            RpsResult::Win => Some(ROCK),
            RpsResult::Draw => Some(SCISSORS),
            RpsResult::Lose => Some(PAPER)
        }
        _ => None
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut score = 0;
    let input = input.lines();
    for line in input {
        let mut line = line.chars();
        let their_move = line.next().expect("Error: missing value");
        let their_move = strategy_to_int(their_move).expect("Error: invalid move");
        line.next();
        let my_move = line.next().expect("Error: missing value");
        let my_move = strategy_to_int(my_move).expect("Error: invalid move");
        score += get_round_points(my_move, their_move).expect("Error: couldn't score round");
    }
    Some(score)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut score = 0;
    let input = input.lines();
    for line in input {
        let mut line = line.chars();
        let their_move = line.next().expect("Error: missing value");
        let their_move = strategy_to_int(their_move).expect("Error: invalid move");
        line.next();
        let outcome = line.next().expect("Error: missing value");
        let outcome = char_to_result(outcome).expect("Error: bad round result");
        let my_move = get_move(their_move, outcome).expect("Error: couldn't get my move");
        score += get_round_points(my_move, their_move).expect("Error: couldn't score round");
    }
    Some(score)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(12));
    }
}
