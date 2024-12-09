use advent_of_code_2022::utils::{grid::{Grid, GridPos}, parse};

advent_of_code_2022::solution!(8);

pub fn part_one(input: &str) -> Option<i32> {
    let (forest, mut visible) = get_tables(input);
    for row_idx in 0..forest.rows() {
        let vis_row = visible.get_mut(row_idx).unwrap();
        let forest_row = forest.get_row(row_idx).unwrap();
        visible_on_row(vis_row, &forest_row);
    }
    for col_index in 0..forest.cols() {
        visible_on_col(&forest, &mut visible, col_index);
    }
    let mut total = 0;
    for row_index in 0..visible.len() {
        let row = visible.get(row_index).unwrap();
        total += row.iter().map(|x| if *x {1} else {0}).sum::<i32>();
    }
    Some(total)
}

fn visible_on_col(forest: &Grid<u32>, visible: &mut Vec<Vec<bool>>, col_index: usize) {
    let mut max_seen = 0;
    for row_index in 0..forest.rows() {
        let tree = forest.get(&GridPos::new(row_index, col_index)).unwrap();
        if *tree > max_seen || (*tree == 0 && row_index == 0) {
            max_seen = *tree;
            *visible.get_mut(row_index).unwrap().get_mut(col_index).unwrap() = true;
        }
    }

    let mut max_seen_bottom = 0;
    for row_index in (0..forest.rows()).rev() {
        let tree = forest.get(&GridPos::new(row_index, col_index)).unwrap();
        if *tree > max_seen_bottom || (*tree == 0 && row_index == forest.rows() - 1) {
            max_seen_bottom = *tree;
            *visible.get_mut(row_index).unwrap().get_mut(col_index).unwrap() = true;
        }
    }
}

fn visible_on_row(visibility_row: &mut Vec<bool>, row: &Vec<&u32>) {
    let mut max_seen = 0;
    for (index, &tree) in row.iter().enumerate() {
        if *tree > max_seen || (*tree == 0 && index == 0) {
            max_seen = *tree;
            *visibility_row.get_mut(index).unwrap() = true;
        }
    }
    let mut max_seen_right = 0;
    for (index, &tree) in row.iter().enumerate().rev() {
        if *tree > max_seen_right || (*tree == 0  && index == row.len() - 1) {
            max_seen_right = *tree;
            *visibility_row.get_mut(index).unwrap() = true;
        }
    }
}

pub fn part_two(input: &str) -> Option<i32> {
    let (forest, visible) = get_tables(input);
    let mut scenic_scores = vec![vec![0; visible.first().unwrap().len()]; visible.len()];

    for row_index in 0..forest.rows() {
        for col_index in 0..forest.cols() {
            let view_distances = get_viewing_distance(&forest, (row_index, col_index));
            let scenic_score = view_distances.iter().product();
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

fn get_viewing_distance(forest: &Grid<u32>, (row, col): (usize, usize)) -> Vec<i32> {
    let start_tree = forest.get(&GridPos::new(row, col)).unwrap();
    let mut view_dist: Vec<i32> = Vec::new();
    let directions: [i32; 2] = [1, -1];
    for dir in directions {
        let mut tmp_row = row as i32;
        let mut x: usize = 0;
        for _ in 1..forest.cols() {
            tmp_row += dir;
            match forest.get_row(tmp_row as usize) {
                Some(a) => {
                    if *a.get(col).unwrap() >= start_tree && tmp_row != row as i32 {
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
        for _ in 1..forest.rows() {
            tmp_col += dir;
            match forest.get(&GridPos::new(row, tmp_col as usize)) {
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

fn get_tables(input: &str) -> (Grid<u32>, Vec<Vec<bool>>) {
    let forest = parse::into_2d_array(
        input,
        |s| s.split("").filter(|&s| !s.eq("")).collect(),
        |s| s.parse::<u32>().unwrap()
    );
    let visible: Vec<Vec<bool>> = forest
        .clone()
        .iter()
        .map(|x| x.iter().map(|_| false).collect())
        .collect();
    let forest = Grid::from(forest);
    (forest, visible)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(8));
    }
}
