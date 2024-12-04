advent_of_code_2022::solution!(8);

pub fn part_one(input: &str) -> Option<i32> {
    let (forest, mut visible) = get_tables(input);
    let row_len = forest.get(0).unwrap().len();
    for row_index in 0..forest.len() {
        let vis_row = visible.get_mut(row_index).unwrap();
        visible_on_row(vis_row, forest.get(row_index).unwrap());
    }
    for col_index in 0..row_len {
        visible_on_col(&mut visible, &forest, col_index);
    }
    let mut total = 0;
    for row_index in 0..visible.len() {
        let row = visible.get(row_index).unwrap();
        total += row.iter().map(|x| if *x {1} else {0}).sum::<i32>();
    }
    // for t in forest {
    //     println!("{:?}", t);
    // }
    // for r in visible {
    //     println!("{:?}", r);
    // }
    // println!("{}", total);
    Some(total)
}

fn get_tables(input: &str) -> (Vec<Vec<u32>>, Vec<Vec<bool>>) {
    let mut forest: Vec<Vec<u32>> = Vec::new();
    let mut visible: Vec<Vec<bool>> = Vec::new();
    for line in input.lines() {
        let trees = line.chars();
        let mut new_forest_line = Vec::<u32>::new();
        let mut new_vis_line = Vec::<bool>::new();
        for t in trees {
            new_forest_line.push(t.to_digit(10).unwrap());
            new_vis_line.push(false);
        }
        forest.push(new_forest_line);
        visible.push(new_vis_line);
    }
    (forest, visible)
}

fn visible_on_col(visible: &mut Vec<Vec<bool>>, forest: &Vec<Vec<u32>>, col_index: usize) {
    let mut max_seen = 0;
    for row_index in 0..forest.len() {
        let row = forest.get(row_index).unwrap();
        let tree: &u32 = row.get(col_index).unwrap();
        if *tree > max_seen || (*tree == 0 && row_index == 0) {
            max_seen = *tree;
            *visible.get_mut(row_index).unwrap().get_mut(col_index).unwrap() = true;
        }
    }

    let mut max_seen_bottom = 0;
    for row_index in (0..forest.len()).rev() {
        let row = forest.get(row_index).unwrap();
        let tree: &u32 = row.get(col_index).unwrap();
        if *tree > max_seen_bottom || (*tree == 0 && row_index == forest.len() - 1) {
            max_seen_bottom = *tree;
            *visible.get_mut(row_index).unwrap().get_mut(col_index).unwrap() = true;
        }
    }
}

fn visible_on_row(visibility_row: &mut Vec<bool>, row: &Vec<u32>) {
    let mut max_seen = 0;
    for (index, tree) in row.iter().enumerate() {
        if *tree > max_seen || (*tree == 0 && index == 0) {
            max_seen = *tree;
            *visibility_row.get_mut(index).unwrap() = true;
        }
    }
    let mut max_seen_right = 0;
    for (index, tree) in row.iter().enumerate().rev() {
        if *tree > max_seen_right || (*tree == 0  && index == row.len() - 1) {
            max_seen_right = *tree;
            *visibility_row.get_mut(index).unwrap() = true;
        }
    }
}

pub fn part_two(input: &str) -> Option<i32> {
    let (forest, visible) = get_tables(input);
    let mut scenic_scores: Vec<Vec<i32>> = Vec::new();
    for row in visible {
        let mut s: Vec<i32> = Vec::new();
        for _ in row {
            s.push(0);
        }
        scenic_scores.push(s);
    }

    for row_index in 0..forest.len() {
        for col_index in 0..forest.get(0).unwrap().len() {
            let view_distances = get_viewing_distance(&forest, (row_index, col_index));
            let scenic_score = view_distances.iter().fold(1, |acc, e| acc * e);
            *scenic_scores.get_mut(row_index).unwrap().get_mut(col_index).unwrap() = scenic_score;
        }
    }
    let mut max_scenic = 0;
    for r in scenic_scores {
        let row_max_scenic = r.iter().max().unwrap();
        max_scenic = max_scenic.max(*row_max_scenic);
    }
    Some(max_scenic)
}

fn get_viewing_distance(forest: &Vec<Vec<u32>>, (row, col): (usize, usize)) -> Vec<i32> {
    let start_tree = forest.get(row).unwrap().get(col).unwrap();
    let mut view_dist: Vec<i32> = Vec::new();
    let directions: [i32; 2] = [1, -1];
    for dir in directions {
        let mut tmp_row = row as i32;
        let mut x: usize = 0;
        for _ in 1..forest.get(row).unwrap().len() {
            tmp_row += dir;
            match forest.get(tmp_row as usize) {
                Some(a) => {
                    if a.get(col).unwrap() >= start_tree && tmp_row != row as i32 {
                        x += 1;
                        break;
                    }
                },
                None => {
                    break;
                }
            }
            x += 1;
        }
        view_dist.push(x as i32);
        let mut tmp_col = col as i32;
        let mut y: usize = 0;
        for _ in 1..forest.len() {
            tmp_col += dir;
            match forest.get(row).unwrap().get(tmp_col as usize) {
                Some(a) => {
                    if a >= start_tree && tmp_col != col as i32 {
                        y += 1;
                        break;
                    }
                },
                None => {
                    break;
                }
            }
            y += 1;
        }
        view_dist.push(y as i32);
    }
    view_dist
}

fn main() {
    let input = advent_of_code_2022::template::read_file("inputs", DAY);
    let res = part_one(&input).unwrap();
    println!("{}", res);
    let res = part_two(&input).unwrap();
    println!("{}", res);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(21)); // fill in
    }
    #[test]
    fn test_part_two() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(8)); // fill in
    }
}
