use advent_of_code_2025::utils::{direction::ALL_DIRECTIONS, grid::Grid, parse::{into_2d_array, split_by_all_chars}};

// Solving https://adventofcode.com/2025/day/4
advent_of_code_2025::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let room_layout = parse_input(input);
    let mut accessible_rolls = 0;
    for (pos, value) in room_layout.iter_by_rows() {
        if value == '.' {
            continue;
        }
        let mut neighbors = 0;
        for dir in ALL_DIRECTIONS {
            if room_layout.get(&pos.position_in_dir(dir)) == Some(&'@') {
                neighbors += 1;
            }
        }
        if neighbors < 4 {
            accessible_rolls += 1;
        }
    }
    Some(accessible_rolls)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut room_layout = parse_input(input);
    let mut remove_count = 0;
    loop {
        let removed = remove_rolls(&mut room_layout);
        remove_count += removed;
        if removed == 0 {
            break;
        }
    }
    Some(remove_count)
}

fn parse_input(input: &str) -> Grid<char> {
    let room = into_2d_array(input, split_by_all_chars, |&s| s.chars().next().unwrap());
    Grid::from(room)
}

fn remove_rolls(room: &mut Grid<char>) -> u64 {
    let mut remove_count = 0;
    for (pos, value) in room.iter_by_rows() {
        if value == '.' {
            continue;
        }
        let mut neighbors = 0;
        for dir in ALL_DIRECTIONS {
            if room.get(&pos.position_in_dir(dir)) == Some(&'@') {
                neighbors += 1;
            }
        }
        if neighbors < 4 {
            *room.get_mut(&pos).unwrap() = '.';
            remove_count += 1;
        }
    }
    remove_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code_2025::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2025::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(43));
    }
}
