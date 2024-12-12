use std::collections::HashSet;

use advent_of_code_2024::utils::{direction::Direction, grid::{Grid, GridPos}};

advent_of_code_2024::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    let mut total_price = 0;
    let mut garden = parse_input(input);
    while !garden.is_empty() {
        let region = get_region(&mut garden);
        let area = region.len() as u32;
        let perim = get_perimeter(&region);
        total_price += area * perim;
    }
    Some(total_price)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total_price = 0;
    let mut garden = parse_input(input);
    while !garden.is_empty() {
        let region = get_region(&mut garden);
        let area = region.len() as u32;
        let num_sides = count_sides(&region);
        total_price += area * num_sides;
    }
    Some(total_price)
}

fn parse_input(input: &str) -> HashSet<(GridPos, char)> {
    let g = input
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let g = Grid::from(g);
    HashSet::from_iter(g.iter_by_rows())
}

fn get_region(garden: &mut HashSet<(GridPos, char)>) -> HashSet<GridPos> {
    let mut region = HashSet::new();
    let mut to_visit = vec![*garden.iter().next().unwrap()];
    while let Some(pt) = to_visit.pop() {
        if region.contains(&pt.0) {
            continue;
        }
        region.insert(pt.0);
        garden.remove(&pt);
        for n in pt.0.get_orthogonal_neighbors() {
            if garden.contains(&(n, pt.1)) {
                to_visit.push((n, pt.1));
            }
        }
    }
    region
}

fn get_perimeter(region: &HashSet<GridPos>) -> u32 {
    let mut perim = 0;
    for &pt in region {
        for n in pt.get_orthogonal_neighbors() {
            if !region.contains(&n) {
                perim += 1;
            }
        }
    }
    perim
}

fn count_sides(region: &HashSet<GridPos>) -> u32 {
    let mut edge_count = 0;
    let ((minrow, mincol), (maxrow, maxcol)) = find_bounding_coords(region);

    for rowidx in minrow..maxrow + 1 {
        let (mut last_up_edge, mut last_down_edge) = (usize::MAX - 1, usize::MAX - 1);
        for colidx in mincol..maxcol + 1 {
            let pos = GridPos::new(rowidx, colidx);
            if !region.contains(&pos) {
                continue;
            }
            if !region.contains(&pos.position_in_dir(Direction::Up)) {
                if colidx == 0 || last_up_edge != colidx - 1 {
                    edge_count += 1;
                }
                last_up_edge = colidx;
            }
            if !region.contains(&pos.position_in_dir(Direction::Down)) {
                if colidx == 0 || last_down_edge != colidx - 1 {
                    edge_count += 1;
                }
                last_down_edge = colidx;
            }
        }
    }

    for colidx in mincol..maxcol + 1 {
        let (mut last_left_edge, mut last_right_edge) = (usize::MAX - 1, usize::MAX - 1);
        for rowidx in minrow..maxrow + 1 {
            let pos = GridPos::new(rowidx, colidx);
            if !region.contains(&pos) {
                continue;
            }
            if !region.contains(&pos.position_in_dir(Direction::Left)) {
                if rowidx == 0 || last_left_edge != rowidx - 1 {
                    edge_count += 1;
                }
                last_left_edge = rowidx;
            }
            if !region.contains(&pos.position_in_dir(Direction::Right)) {
                if rowidx == 0 || last_right_edge != rowidx - 1 {
                    edge_count += 1;
                }
                last_right_edge = rowidx;
            }
        }
    }
    edge_count
}

fn find_bounding_coords(region: &HashSet<GridPos>) -> ((usize, usize), (usize, usize)) {
    let (mut minrow, mut mincol) = (usize::MAX, usize::MAX);
    let (mut maxrow, mut maxcol) = (0, 0);
    for pos in region {
        if pos.row < minrow {
            minrow = pos.row;
        }
        if pos.row > maxrow {
            maxrow = pos.row;
        }
        if pos.col < mincol{
            mincol = pos.col;
        }
        if pos.col > maxcol {
            maxcol = pos.col;
        }
    }
    ((minrow, mincol), (maxrow, maxcol))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(1206));
    }
}
