use advent_of_code_2022::utils::parse;

advent_of_code_2022::solution!(14);

#[derive(Clone)]
enum Space {
    Empty,
    Sand,
    Rock,
}

pub fn part_one(input: &str) -> Option<i32> {
    let paths = get_paths(input);
    let mut grid = construct_matrix(&paths);
    let mut count = 0;
    while let 0 = run_sand(&mut grid) {
        count += 1;
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<i32> {
    let paths = get_paths(input);
    let mut grid = construct_matrix(&paths);
    add_floor(&mut grid);
    let mut count = 0;
    while let 0 = run_sand(&mut grid) {
        count += 1;
    }
    Some(count)
}

fn add_floor(grid: &mut Vec<Vec<Space>>) {
    let mut end_rock_row = grid.len();
    for row_index in (0..grid.len()).rev() {
        for space in grid.get(row_index).unwrap() {
            if let Space::Rock = space {

                end_rock_row = row_index;
                break;
            }
        }
        if end_rock_row != grid.len() {
            break;
        }
    }
    let floor = grid.get_mut(end_rock_row + 2).unwrap();
    for space in floor {
        *space = Space::Rock;
    }
}

fn run_sand(grid: &mut Vec<Vec<Space>>) -> i32 {
    let mut pos = (0, 500);
    loop {

        if let Space::Sand = grid.get(pos.0).unwrap().get(pos.1).unwrap() {
            return 1;
        };
        if pos.0 == grid.len() - 1 || (pos.1 as i32) < 0
            || pos.1 >= grid.first().unwrap().len()
        {
            return 1;
        }
        let row_above = grid.get_mut(pos.0 + 1).unwrap();
        if let Space::Empty = row_above.get_mut(pos.1).unwrap() {
            pos = (pos.0 + 1, pos.1);
            continue;
        }
        if let Space::Empty = row_above.get_mut(pos.1 - 1).unwrap() {
            pos = (pos.0 + 1, pos.1 - 1);
            continue;
        }
        let next = row_above.get_mut(pos.1 + 1).unwrap();
        match next {
            Space::Empty => {
                pos = (pos.0 + 1, pos.1 + 1);
                continue;
            }
            _ => {
                *grid.get_mut(pos.0).unwrap().get_mut(pos.1).unwrap() = Space::Sand;
                return 0;
            },
        };
    }
}

fn get_paths(input: &str) -> Vec<Vec<(i32, i32)>> {
    let paths = parse::into_2d_array(
        input,
        |l| l.split(" -> ").collect(),
        |pt| {
            let tmp: Vec<i32> = pt.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
            (*tmp.get(1).unwrap(), *tmp.first().unwrap())
        }
    );
    paths
}

fn construct_matrix(rock_paths: &Vec<Vec<(i32, i32)>>) -> Vec<Vec<Space>> {
    let mut matrix: Vec<Vec<Space>> = vec![vec![Space::Empty; 700]; 400];
    for path in rock_paths {
        for index in 0..path.len() - 1 {
            let mut start = *path.get(index).unwrap();
            let end = *path.get(index + 1).unwrap();
            let dir = ((end.0 - start.0).signum(), (end.1 - start.1).signum());
            while start != end {
                let tmp = matrix.get_mut(start.0 as usize).unwrap().get_mut(start.1 as usize).unwrap();
                *tmp = Space::Rock;
                start = (start.0 + dir.0, start.1 + dir.1);
            }
            *matrix.get_mut(start.0 as usize).unwrap().get_mut(start.1 as usize).unwrap() = Space::Rock;
        }
    }
    matrix
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(93));
    }
}
