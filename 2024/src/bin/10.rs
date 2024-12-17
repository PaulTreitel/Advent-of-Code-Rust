use std::collections::HashMap;

use advent_of_code_2024::utils::{grid::{Grid, GridPos}, parse};

const TRAILHEAD: u32 = 0;
const TRAILEND: u32 = 9;

type TrailHeadMap = HashMap<GridPos, Vec<GridPos>>;

advent_of_code_2024::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_input(input);
    let mut trailheads = get_trailheads(&map);
    find_full_trails(&map, &mut trailheads);
    let trail_ct: u32 = trailheads
        .values_mut()
        .map(|x| {
            x.sort();
            x.dedup();
            x.len() as u32
        })
        .sum();
    Some(trail_ct)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse_input(input);
    let mut trailheads = get_trailheads(&map);
    find_full_trails(&map, &mut trailheads);
    let trail_ct = trailheads
        .values_mut()
        .map(|x| x.len() as u32)
        .sum();
    Some(trail_ct)
}

fn parse_input(input: &str) -> Grid<u32> {
    let grid = parse::into_2d_array(
        input,
        parse::split_by_all_chars,
        |s| s.parse::<u32>().unwrap());
    Grid::from(grid)
}

fn get_trailheads(map: &Grid<u32>) -> TrailHeadMap {
    HashMap::from_iter(map
        .iter_by_rows()
        .filter(|(_, val)| *val == TRAILHEAD)
        .map(|(pos, _)| (pos, vec![])))
}

fn find_full_trails(map: &Grid<u32>, trailheads: &mut TrailHeadMap) {
    for head in trailheads.clone().keys() {
        let endpoints = trailheads.get_mut(head).unwrap();
        *endpoints = map.bfs_all_matches(
            *head,
            |(_, v1), (_, v2)| *v2 == *v1 + 1,
            |_, v| *v == TRAILEND
        )
            .iter()
            .map(|(_, pos)| *pos)
            .collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(81));
    }
}
