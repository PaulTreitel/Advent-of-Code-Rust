// Solving https://adventofcode.com/2022/day/2
use advent_of_code_2022::utils::parse;

advent_of_code_2022::solution!(2);

const ROCK_SCORE: i32 = 1;
const PAPER_SCORE: i32 = 2;
const SCISSORS_SCORE: i32 = 3;
const WIN_SCORE: i32 = 6;
const DRAW_SCORE: i32 = 3;

enum RpsResult {
    Win,
    Draw,
    Lose
}

enum Move {
    Rock,
    Paper,
    Scissors
}

fn get_round_points(my_move: &Move, their_move: &Move) -> Option<i32> {
    match my_move {
        Move::Rock => match their_move {
            Move::Rock => Some(ROCK_SCORE + DRAW_SCORE),
            Move::Paper => Some(ROCK_SCORE),
            Move::Scissors => Some(ROCK_SCORE + WIN_SCORE),
        },
        Move::Paper => match their_move {
            Move::Rock => Some(PAPER_SCORE + WIN_SCORE),
            Move::Paper => Some(PAPER_SCORE + DRAW_SCORE),
            Move::Scissors => Some(PAPER_SCORE),
        },
        Move::Scissors => match their_move {
            Move::Rock => Some(SCISSORS_SCORE),
            Move::Paper => Some(SCISSORS_SCORE + WIN_SCORE),
            Move::Scissors => Some(SCISSORS_SCORE + DRAW_SCORE),
        },
    }
}

fn char_to_move(ch: char) -> Option<Move> {
    match ch.to_ascii_uppercase() {
        'A' => Some(Move::Rock),
        'B' => Some(Move::Paper),
        'C' => Some(Move::Scissors),
        'X' => Some(Move::Rock),
        'Y' => Some(Move::Paper),
        'Z' => Some(Move::Scissors),
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

fn get_move(their_move: &Move, outcome: RpsResult) -> Move {
    match their_move {
        Move::Rock => match outcome {
            RpsResult::Win => Move::Paper,
            RpsResult::Draw => Move::Rock,
            RpsResult::Lose => Move::Scissors
        }
        Move::Paper => match outcome {
            RpsResult::Win => Move::Scissors,
            RpsResult::Draw => Move::Paper,
            RpsResult::Lose => Move::Rock
        }
        Move::Scissors => match outcome {
            RpsResult::Win => Move::Rock,
            RpsResult::Draw => Move::Scissors,
            RpsResult::Lose => Move::Paper
        }
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut score = 0;
    let (their_moves, my_moves) = parse_input(input);
    let round_iter = my_moves
        .iter()
        .zip(their_moves)
        .map(|(x, y)| (char_to_move(*x).unwrap(), char_to_move(y).unwrap()));
    for (me, them) in round_iter {
        score += get_round_points(&me, &them).unwrap();
    }
    Some(score)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut score = 0;
    let (their_moves, outcomes) = parse_input(input);
    let round_iter = their_moves
        .iter()
        .zip(outcomes)
        .map(|(x, y)| (char_to_move(*x).unwrap(), char_to_result(y).unwrap()));
    for (them, outcome) in round_iter {
        let me = get_move(&them, outcome);
        score += get_round_points(&me, &them).unwrap();
    }
    Some(score)
}

fn parse_input(input: &str) -> (Vec<char>, Vec<char>) {
    parse::split_two_vertical_lists(
        input,
        |s| s.split(" ").collect(),
        |s| s.chars().next().unwrap(),
    )
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
