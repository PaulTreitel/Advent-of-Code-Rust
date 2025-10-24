// Solving https://adventofcode.com/2024/day/20
use std::collections::HashMap;

use advent_of_code_2024::utils::{direction::ORTHOGONAL_DIRECTIONS, grid::{Grid, GridPos}, parse};

advent_of_code_2024::solution!(20);

const PART_ONE_EXAMPLE_CHEAT_CUTTOFF: i64 = 20;
const PART_TWO_EXAMPLE_CHEAT_CUTOFF: i64 = 70;
const PART_TWO_CHEAT_DURATION: i64 = 20;
const PUZZLE_CHEAT_CUTOFF: i64 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum RaceTile {
    Path,
    Wall,
    Start,
    End,
}

impl RaceTile {
    pub fn from_ch(ch: char) -> Self {
        match ch {
            '.' => Self::Path,
            '#' => Self::Wall,
            'S' => Self::Start,
            'E' => Self::End,
            _ => panic!("character {} not a race map tile", ch)
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let cheat_cutoff = {
        if input.len() > 1000 {
            PUZZLE_CHEAT_CUTOFF
        } else {
            PART_ONE_EXAMPLE_CHEAT_CUTTOFF
        }
    };
    let track = parse_input(input);
    let dists = run_track(&track);
    let mut good_cheat_ct = 0;
    for (cheat_start_pos, &cheat_start_cost) in &dists {
        for dir in ORTHOGONAL_DIRECTIONS {
            if *track.get(&cheat_start_pos.position_in_dir(dir)).unwrap() == RaceTile::Path {
                continue;
            }
            let cheat_end_pos = cheat_start_pos
                .position_in_dir(dir)
                .position_in_dir(dir);
            if !track.is_valid_cell(&cheat_end_pos)
                || *track.get(&cheat_end_pos).unwrap() == RaceTile::Wall
            {
                continue;
            }
            let cheat_end_cost = *dists.get(&cheat_end_pos).unwrap() as i64;
            if cheat_start_cost as i64 + 2 <= cheat_end_cost - cheat_cutoff {
                good_cheat_ct += 1;
            }
        }
    }
    Some(good_cheat_ct)
}

pub fn part_two(input: &str) -> Option<u64> {
    let cheat_cutoff = {
        if input.len() > 1000 {
            PUZZLE_CHEAT_CUTOFF
        } else {
            PART_TWO_EXAMPLE_CHEAT_CUTOFF
        }
    };
    let track = parse_input(input);
    let dists = run_track(&track);
    let mut good_cheat_ct = 0;
    for (cheat_start_pos, _) in &dists {
        good_cheat_ct += get_cheat_ct(&dists, cheat_start_pos, cheat_cutoff);
    }
    Some(good_cheat_ct)
}

fn parse_input(input: &str) -> Grid<RaceTile> {
    let maze = parse::into_2d_array(
        input,
        parse::split_by_all_chars,
        |s| RaceTile::from_ch(s.chars().next().unwrap())
    );
    Grid::from(maze)
}

fn run_track(track: &Grid<RaceTile>) -> HashMap<GridPos, u64> {
    let mut track_run = HashMap::new();
    let start = track.index_of(|t| *t == RaceTile::Start).unwrap();
    let end = track.index_of(|t| *t == RaceTile::End).unwrap();
    let mut curr_pos = start;
    let mut dist_traveled = 0;
    track_run.insert(curr_pos, 0);
    while curr_pos != end {
        let neighbors = curr_pos.get_orthogonal_neighbors();
        for n in neighbors {
            if *track.get(&n).unwrap() == RaceTile::Wall || track_run.contains_key(&n) {
                continue;
            }
            dist_traveled += 1;
            curr_pos = n;
            track_run.insert(n, dist_traveled);
        }
    }
    track_run
}

fn get_cheat_ct(dists: &HashMap<GridPos, u64>, start: &GridPos, cheat_cutoff: i64) -> u64 {
    let lower_offset_bound = -PART_TWO_CHEAT_DURATION as i32;
    let upper_offset_bound = PART_TWO_CHEAT_DURATION as i32 + 1;
    let start_cost = *dists.get(start).unwrap() as i64;
    let mut cheat_ct = 0;
    for row_offset in lower_offset_bound..upper_offset_bound {
        for col_offset in lower_offset_bound..upper_offset_bound {
            let cheat_dist = (row_offset.abs() + col_offset.abs()) as i64;
            if cheat_dist > 20 {
                continue;
            }
            let end_pos = start.position_at_offset(row_offset, col_offset);
            if !dists.contains_key(&end_pos) {
                continue;
            }
            let end_cost = *dists.get(&end_pos).unwrap() as i64;
            let savings = end_cost - (start_cost + cheat_dist);
            if savings >= cheat_cutoff {
                // println!("new cheat {} to {} saving {}", start, end_pos, savings);
                cheat_ct += 1;
            }
        }
    }
    cheat_ct
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(41));
    }
}
