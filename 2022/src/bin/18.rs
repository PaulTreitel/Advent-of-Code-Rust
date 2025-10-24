// Solving https://adventofcode.com/2022/day/18
advent_of_code_2022::solution!(18);

use std::cmp::max;

const  DELTA_NEIGHBORS: [[i32; 3]; 6] = [[-1, 0, 0], [1, 0, 0], [0, -1, 0], [0, 1, 0], [0, 0, -1], [0, 0, 1]];

pub fn part_one(input: &str) -> Option<i32> {
    let (points, max_val) = get_points(input);
    let mut space: Vec<Vec<Vec<i32>>> = construct_space(max_val);
    add_points_to_space(&mut space, &points);
    let mut surfaces = 0;

    for i in 0..points.len() {
        let mut sub_sum = 0;
        let pt = points.get(i).unwrap();
        let (x, y, z) = (
            pt.get(0).unwrap(),
            pt.get(1).unwrap(),
            pt.get(2).unwrap());

        for [dx, dy, dz] in DELTA_NEIGHBORS {
            if x + dx < 0 || y + dy < 0 || z + dz < 0 {
                continue;
            }
            sub_sum += *space.get((*x + dx) as usize).unwrap()
            .get((*y + dy) as usize).unwrap()
            .get((*z + dz) as usize).unwrap();
        }
        surfaces += 6 - sub_sum;
    }
    Some(surfaces)
}

pub fn part_two(input: &str) -> Option<i32> {
    let (points, max_val) = get_points(input);
    let mut space: Vec<Vec<Vec<i32>>> = construct_space(max_val);
    add_points_to_space(&mut space, &points);
    fill_steam(&mut space);
    let mut surfaces = 0;

    for i in 0..points.len() {
        let mut sub_sum = 0;
        let pt = points.get(i).unwrap();
        let (x, y, z) = (
            pt.get(0).unwrap(),
            pt.get(1).unwrap(),
            pt.get(2).unwrap());

        for [dx, dy, dz] in DELTA_NEIGHBORS {
            if x + dx < 0 || y + dy < 0 || z + dz < 0 {
                continue;
            }
            let neighbor_val = *space.get((*x + dx) as usize).unwrap()
                .get((*y + dy) as usize).unwrap()
                .get((*z + dz) as usize).unwrap();
            if neighbor_val != 2 {
                sub_sum += 1;
            }
        }
        surfaces += 6 - sub_sum;
    }
    Some(surfaces)
}

fn fill_steam(space: &mut Vec<Vec<Vec<i32>>>) -> () {
    let mut stack: Vec<(i32, i32, i32)> = Vec::new();
    stack.push((0, 0, 0));

    while !stack.is_empty() {
        let pos = stack.pop().unwrap();
        *space.get_mut(pos.0 as usize).unwrap()
            .get_mut(pos.1 as usize).unwrap()
            .get_mut(pos.2 as usize).unwrap() = 2;
        for [dx, dy, dz] in DELTA_NEIGHBORS {
            if pos.0 + dx < 0 || pos.1 + dy < 0 || pos.2 + dz < 0 {
                continue;
            }
            if pos.0 + dx >= space.len() as i32
                || pos.1 + dy >= space.len() as i32
                || pos.2 + dz >= space.len() as i32 {
                continue;
            }
            // println!("{}, {}, {} :: {}, {}, {}", pos.0, pos.1, pos.2, dx, dy, dz);
            if *space.get((pos.0 + dx) as usize).unwrap()
            .get((pos.1 + dy) as usize).unwrap()
            .get((pos.2 + dz) as usize).unwrap() == 0 {
                stack.push((pos.0 + dx, pos.1 + dy, pos.2 + dz));
            }
        }
    }
}

fn add_points_to_space(space: &mut Vec<Vec<Vec<i32>>>, points: &Vec<[i32; 3]>) -> () {
    for i in 0..points.len() {
        let pt = points.get(i).unwrap();
        let (x, y, z) = (
            pt.get(0).unwrap(),
            pt.get(1).unwrap(),
            pt.get(2).unwrap());
        *space.get_mut(*x as usize).unwrap()
            .get_mut(*y as usize).unwrap()
            .get_mut(*z as usize).unwrap() = 1;
    }
}

fn construct_space(max_val: i32) -> Vec<Vec<Vec<i32>>> {
    let mut space = Vec::new();
    let mut inner_vec = Vec::new();
    for _ in 0..max_val {
        inner_vec.push(0);
    }
    let mut middle_vec = Vec::new();
    for _ in 0..max_val {
        middle_vec.push(inner_vec.clone());
    }
    for _ in 0..max_val {
        space.push(middle_vec.clone());
    }
    space
}

fn get_points(input: &str) -> (Vec<[i32; 3]>, i32) {
    let mut pts = Vec::new();
    let mut max_val = 0;
    for line in input.lines() {
        let mut line = line.split(",");
        let x = line.next().unwrap().parse::<i32>().unwrap();
        let y = line.next().unwrap().parse::<i32>().unwrap();
        let z = line.next().unwrap().parse::<i32>().unwrap();
        let new = [x, y, z];
        max_val = max(max_val, *new.iter().max_by(|x, y| x.cmp(y)).unwrap());
        pts.push([x, y, z]);
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
