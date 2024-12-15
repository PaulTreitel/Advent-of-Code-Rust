advent_of_code_2022::solution!(12);

pub fn part_one(input: &str) -> Option<i32> {
    let (map, start) = get_elevation_map(input);
    let num_steps = bfs_map(map, &mut vec![(0, start)]);
    Some(num_steps)
}

fn valid_step(a: i32, b: i32) -> bool {
    b == a + 1 || b <= a
}

fn bfs_map(map: Vec<Vec<i32>>, queue: &mut Vec<(i32, (i32, i32))>) -> i32 {
    let mut visited: Vec<(i32, i32)> = Vec::new();
    let rows = map.len();
    let cols = map.first().unwrap().len();
    while !queue.is_empty() {
        let (moves, pos) = queue.remove(0);
        if visited.contains(&pos) {
            continue;
        }
        visited.push(pos);
        let start_elevation = map
            .get(pos.0 as usize)
            .unwrap()
            .get(pos.1 as usize)
            .unwrap();
        for (newrow, newcol) in [
            (pos.0 + 1, pos.1),
            (pos.0 - 1, pos.1),
            (pos.0, pos.1 + 1),
            (pos.0, pos.1 - 1),
        ] {
            if (newrow < 0 || newrow >= rows as i32) || (newcol < 0 || newcol >= cols as i32) {
                continue;
            }
            let new_elevation = map
                .get(newrow as usize)
                .unwrap()
                .get(newcol as usize)
                .unwrap();
            if valid_step(*start_elevation, *new_elevation) {
                if *new_elevation == 26 {
                    return moves + 1;
                }
                if !visited.contains(&(newrow, newcol)) {
                    queue.push((moves + 1, (newrow, newcol)));
                }
            }
        }
    }
    -1
}

pub fn part_two(input: &str) -> Option<i32> {
    let (map, _) = get_elevation_map(input);
    let mut starts: Vec<(i32, (i32, i32))> = Vec::new();
    for row_index in 0..map.len() {
        for col_index in 0..map.first().unwrap().len() {
            let elevation = map.get(row_index).unwrap().get(col_index).unwrap();
            if *elevation == 0 {
                starts.push((0, (row_index as i32, col_index as i32)));
            }
        }
    }
    let steps = bfs_map(map, &mut starts);
    Some(steps)
}

fn get_elevation_map(input: &str) -> (Vec<Vec<i32>>, (i32, i32)) {
    let mut map = Vec::<Vec<i32>>::new();
    let mut start: (i32, i32) = (0, 0);
    for (rowcount, line) in input.lines().enumerate() {
        let mut map_row: Vec<i32> = Vec::new();
        let line = line.chars();
        for (colcount, ch) in line.enumerate() {
            map_row.push(get_height(ch));
            if ch == 'S' {
                start = (rowcount as i32, colcount as i32);
            }
        }
        map.push(map_row);
    }
    (map, start)
}

fn get_height(ch: char) -> i32 {
    if ch == 'S' {
        return 0;
    } else if ch == 'E' {
        return 26;
    }
    ch as i32 - 'a' as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(29));
    }
}
