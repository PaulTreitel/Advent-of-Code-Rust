advent_of_code_2022::solution!(18);

use std::cmp::max;

type Position = (i32, i32, i32);

const DELTA_NEIGHBORS: [Position; 6] = [
    (-1, 0, 0),
    (1, 0, 0),
    (0, -1, 0),
    (0, 1, 0),
    (0, 0, -1),
    (0, 0, 1),
];

const EMPTY: i32 = 0;
const LAVA: i32 = 1;
const STEAM: i32 = 2;

pub fn part_one(input: &str) -> Option<i32> {
    let (points, max_val) = get_points(input);
    let mut space: Vec<Vec<Vec<i32>>> = construct_space(max_val);
    add_lava_to_space(&mut space, &points);
    let mut surfaces = 0;

    for pos in points {
        let mut sub_sum = 0;

        for offset in DELTA_NEIGHBORS {
            if !valid_coords(pos, offset, space.len() as i32) {
                continue;
            }
            sub_sum += *space
                .get((pos.0 + offset.0) as usize)
                .unwrap()
                .get((pos.1 + offset.1) as usize)
                .unwrap()
                .get((pos.2 + offset.2) as usize)
                .unwrap();
        }
        surfaces += 6 - sub_sum;
    }
    Some(surfaces)
}

pub fn part_two(input: &str) -> Option<i32> {
    let (points, max_val) = get_points(input);
    let mut space: Vec<Vec<Vec<i32>>> = construct_space(max_val);
    add_lava_to_space(&mut space, &points);
    fill_steam(&mut space);
    let mut surfaces = 0;

    for pos in points {
        let mut sub_sum = 0;

        for offset in DELTA_NEIGHBORS {
            if !valid_coords(pos, offset, space.len() as i32) {
                continue;
            }
            let neighbor_val = *space
                .get((pos.0 + offset.0) as usize)
                .unwrap()
                .get((pos.1 + offset.1) as usize)
                .unwrap()
                .get((pos.2 + offset.2) as usize)
                .unwrap();
            if neighbor_val != STEAM {
                sub_sum += 1;
            }
        }
        surfaces += 6 - sub_sum;
    }
    Some(surfaces)
}

fn fill_steam(space: &mut [Vec<Vec<i32>>]) {
    let mut stack: Vec<Position> = Vec::new();
    stack.push((0, 0, 0));

    while let Some(pos) = stack.pop() {
        *space
            .get_mut(pos.0 as usize)
            .unwrap()
            .get_mut(pos.1 as usize)
            .unwrap()
            .get_mut(pos.2 as usize)
            .unwrap() = STEAM;
        for (dx, dy, dz) in DELTA_NEIGHBORS {
            if !valid_coords(pos, (dx, dy, dz), space.len() as i32) {
                continue;
            }
            if *space
                .get((pos.0 + dx) as usize)
                .unwrap()
                .get((pos.1 + dy) as usize)
                .unwrap()
                .get((pos.2 + dz) as usize)
                .unwrap()
                == EMPTY
            {
                stack.push((pos.0 + dx, pos.1 + dy, pos.2 + dz));
            }
        }
    }
}

fn valid_coords(pos: Position, offset: Position, size: i32) -> bool {
    pos.0 + offset.0 >= 0
        && pos.0 + offset.0 < size
        && pos.1 + offset.1 >= 0
        && pos.1 + offset.1 < size
        && pos.2 + offset.2 >= 0
        && pos.2 + offset.2 < size
}

fn add_lava_to_space(space: &mut [Vec<Vec<i32>>], points: &[Position]) {
    for (x, y, z) in points {
        *space
            .get_mut(*x as usize)
            .unwrap()
            .get_mut(*y as usize)
            .unwrap()
            .get_mut(*z as usize)
            .unwrap() = LAVA;
    }
}

fn construct_space(max_val: i32) -> Vec<Vec<Vec<i32>>> {
    vec![vec![vec![EMPTY; max_val as usize]; max_val as usize]; max_val as usize]
}

fn get_points(input: &str) -> (Vec<Position>, i32) {
    let mut pts = Vec::new();
    let mut max_val = 0;
    for line in input.lines() {
        let nums: Vec<i32> = line.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
        max_val = max(max_val, *nums.iter().max_by(|x, y| x.cmp(y)).unwrap());
        let nums = (
            *nums.first().unwrap(),
            *nums.get(1).unwrap(),
            *nums.get(2).unwrap(),
        );
        pts.push(nums);
    }
    (pts, max_val + 2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(58));
    }
}
