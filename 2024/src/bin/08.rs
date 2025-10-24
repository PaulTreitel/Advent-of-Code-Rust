// Solving https://adventofcode.com/2024/day/8
use std::collections::{HashMap, HashSet};

use advent_of_code_2024::utils::{
    grid::{Grid, GridPos},
    parse,
};

advent_of_code_2024::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_input(input);
    let antennae = get_antennae(&map);
    let mut antinodes = HashSet::new();
    for (_ch, positions) in antennae {
        add_antenna_antinodes_part_one(&map, &mut antinodes, &positions);
    }
    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse_input(input);
    let antennae = get_antennae(&map);
    let mut antinodes = HashSet::new();
    for (_ch, positions) in antennae {
        add_antenna_antinodes_part_two(&map, &mut antinodes, &positions);
    }
    Some(antinodes.len() as u32)
}

fn parse_input(input: &str) -> Grid<char> {
    let input = parse::into_2d_array(input, parse::split_by_all_chars, parse::to_first_char);
    Grid::from(input)
}

fn get_antennae(map: &Grid<char>) -> HashMap<char, Vec<GridPos>> {
    let mut unique_letters: HashMap<char, Vec<GridPos>> = HashMap::new();
    for (pos, ch) in map.iter_by_rows() {
        if ch != '.' {
            unique_letters
                .entry(ch)
                .and_modify(|x| x.push(pos))
                .or_insert(vec![pos]);
        }
    }
    unique_letters
}

fn add_antenna_antinodes_part_one(
    map: &Grid<char>,
    antinodes: &mut HashSet<GridPos>,
    positions: &Vec<GridPos>,
) {
    for pos_1 in positions {
        for pos_2 in positions {
            if pos_1 == pos_2 {
                continue;
            }
            let offset = (
                pos_1.row as i32 - pos_2.row as i32,
                pos_1.col as i32 - pos_2.col as i32,
            );
            // pos2 + offset = pos1
            // antinode 1 = pos2 - offset
            // antinode 2 = pos1 + offset
            let antinode_1 = (pos_2.row as i32 - offset.0, pos_2.col as i32 - offset.1);
            let antinode_2 = (pos_1.row as i32 + offset.0, pos_1.col as i32 + offset.1);
            if is_valid_node(map, antinode_1) {
                let new = GridPos {
                    row: antinode_1.0 as usize,
                    col: antinode_1.1 as usize,
                };
                antinodes.insert(new);
            }
            if is_valid_node(map, antinode_2) {
                let new = GridPos {
                    row: antinode_2.0 as usize,
                    col: antinode_2.1 as usize,
                };
                antinodes.insert(new);
            }
        }
    }
}

fn add_antenna_antinodes_part_two(
    map: &Grid<char>,
    antinodes: &mut HashSet<GridPos>,
    positions: &Vec<GridPos>,
) {
    for pos_1 in positions {
        for pos_2 in positions {
            if pos_1 == pos_2 {
                continue;
            }
            let offset = (
                pos_1.row as i32 - pos_2.row as i32,
                pos_1.col as i32 - pos_2.col as i32,
            );

            for off in [offset, (-offset.0, -offset.1)] {
                let new_antinodes: HashSet<GridPos> = map
                    .scan_direction_until(pos_1, off, |_, _| false)
                    .unwrap_or_default()
                    .iter()
                    .map(|(p, _)| *p)
                    .collect();
                antinodes.extend(new_antinodes);
            }
        }
    }
}

fn is_valid_node(map: &Grid<char>, node: (i32, i32)) -> bool {
    node.0 >= 0 && node.1 >= 0 && node.0 < map.rows() as i32 && node.1 < map.cols() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(34));
    }
}
