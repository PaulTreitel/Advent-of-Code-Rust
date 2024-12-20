use std::collections::HashMap;

use regex::Regex;

advent_of_code_2024::solution!(19);

pub fn part_one(input: &str) -> Option<u64> {
    let (towels, designs) = parse_input(input);
    let working_designs = get_working_designs(&towels, &designs);
    Some(working_designs.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (towels, designs) = parse_input(input);
    let working_designs = get_working_designs(&towels, &designs);

    let mut towels: Vec<_> = towels.iter().map(|x| x.to_string()).collect();
    towels.sort_by(|s1, s2| s1.len().cmp(&s2.len()));
    let mut designs_found = get_design_map(&towels);

    let ways_to_make_designs: u64 = working_designs.iter()
        .map(|s| count_ways_to_make_design(&towels, &mut designs_found, *s))
        .sum();
    Some(ways_to_make_designs)
}

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut lines = input.lines();
    let towels: Vec<_> = lines.next().unwrap().split(", ").collect();
    lines.next();
    let designs: Vec<_> = lines.collect();
    (towels, designs)
}

fn get_working_designs<'a>(towels: &Vec<&str>, designs: &Vec<&'a str>) -> Vec<&'a str> {
    let search_str = "^((".to_owned() + &towels.join(")|(") + "))+$";
    let re = Regex::new(&search_str).unwrap();
    designs.iter()
        .filter(|s| re.is_match(s))
        .copied()
        .collect()
}

fn get_design_map(towels: &Vec<String>) -> HashMap<String, u64> {
    let mut towelmap: HashMap<String, u64> = HashMap::new();
    let mut towels_processed = vec![];
    for t in towels {
        let _ = count_ways_to_make_design(
            &towels_processed,
            &mut towelmap,
            t
        );
        *towelmap.get_mut(t).unwrap() += 1;
        towels_processed.push(t.clone());
    }
    towelmap
}

fn count_ways_to_make_design(
    base_towels: &Vec<String>,
    designs_found: &mut HashMap<String, u64>,
    design: &str
) -> u64 {
    if designs_found.contains_key(design) {
        return *designs_found.get(design).unwrap();
    }
    let mut total_ways = 0;
    for t in base_towels {
        if is_prefix(t, design) {
            let subdesign_ct = count_ways_to_make_design(
                base_towels,
                designs_found,
                &design[t.len()..]
            );
            total_ways += subdesign_ct;
        }
    }
    designs_found.insert(design.to_string(), total_ways);
    total_ways
}

fn is_prefix(pre: &str, haystack: &str) -> bool {
    if pre.len() > haystack.len() {
        return false;
    }
    let mut pre = pre.chars();
    let mut haystack = haystack.chars();
    while let Some(pre_ch) = pre.next() {
        if !(haystack.next().unwrap() == pre_ch) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(16));
    }
}
