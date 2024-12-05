use advent_of_code_2024::utils::parse;
use graph_builder::prelude::*;
use std::cmp::Ordering;

advent_of_code_2024::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, pages) = parse_input(input);
    let successful_booklets: Vec<Vec<u32>> = pages
        .iter()
        .filter(|x| pages_in_order(&rules, x))
        .cloned()
        .collect();
    Some(middle_pages(&successful_booklets))
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, pages) = parse_input(input);
    let fixed_booklets: Vec<Vec<u32>> = pages
        .iter()
        .filter(|x| !pages_in_order(&rules, x))
        .map(|x| reorder_pages(&rules, x))
        .collect();
    Some(middle_pages(&fixed_booklets))
}

fn page_order(rules: &DirectedALGraph<u32>, page: u32, other: u32) -> Ordering {
    for successor in rules.out_neighbors(page) {
        if *successor == other {
            return Ordering::Less;
        }
    }
    for successor in rules.out_neighbors(other) {
        if *successor == other {
            return Ordering::Greater;
        }
    }
    Ordering::Equal
}

fn reorder_pages(rules: &DirectedALGraph<u32>, pages: &[u32]) -> Vec<u32> {
    let mut pages = pages.to_owned();
    pages.sort_by(|x, y| page_order(rules, *x, *y));
    pages
}

fn middle_pages(booklets: &[Vec<u32>]) -> u32 {
    booklets.iter().map(|x| x.get(x.len() / 2).unwrap()).sum()
}

fn pages_in_order(rules: &DirectedALGraph<u32>, pages: &Vec<u32>) -> bool {
    pages.eq(&reorder_pages(rules, pages))
}

fn parse_input(input: &str) -> (DirectedALGraph<u32>, Vec<Vec<u32>>) {
    let tmp: Vec<&str> = input.split("\n\n").collect();
    let rules: Vec<(u32, u32)> = parse::into_2d_array(
        tmp.first().unwrap(),
        |x| x.split("|").collect(),
        |x| x.parse::<u32>().unwrap(),
    )
    .iter()
    .map(|x| (*x.first().unwrap(), *x.get(1).unwrap()))
    .collect();
    let print_pages = parse::into_2d_array(
        tmp.get(1).unwrap(),
        |x| x.split(",").collect(),
        |x| x.parse::<u32>().unwrap(),
    );
    (GraphBuilder::new().edges(rules).build(), print_pages)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(123));
    }
}
