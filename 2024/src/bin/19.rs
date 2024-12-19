use std::collections::{HashMap, HashSet};

use regex::Regex;

advent_of_code_2024::solution!(19);

pub fn part_one(input: &str) -> Option<u64> {
    let (towels, designs) = parse_input(input);
    let search_str = "^((".to_owned() + &towels.join(")|(") + "))+$";
    let re = Regex::new(&search_str).unwrap();
    let working_designs = designs.iter()
        .filter(|s| re.is_match(s))
        .count();
    Some(working_designs as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (towels, designs) = parse_input(input);
    let search_str = "^((".to_owned() + &towels.join(")|(") + "))+$";
    let re = Regex::new(&search_str).unwrap();
    let working_designs: Vec<&str> = designs.iter()
        .filter(|s| re.is_match(s))
        .copied()
        .collect();

    let towels: Vec<_> = towels.iter().map(|x| x.to_string()).collect();
    let designs_found = get_design_map(&towels);
    let mut designs_found_cts: HashMap<_, _> = designs_found.iter()
        .map(|x| (x.0.clone(), x.1.len() as u64))
        .collect();

    let ways_to_make_designs: u64 = working_designs.iter()
        .map(|s| count_ways_to_make_design(&towels, &mut designs_found_cts, *s))
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

fn get_design_map<'a>(towels: &'a Vec<String>) -> HashMap<String, HashSet<Vec<&'a String>>> {
    let mut towelmap: HashMap<String, HashSet<Vec<&'a String>>> = HashMap::new();
    for t in &(*towels) {
        let mut tmp = HashSet::new();
        tmp.insert(vec![t]);
        towelmap.insert(t.to_string(), tmp);
    }

    for t1 in towels {
        for t2 in towels {
            for t3 in towels {
                add_combo(&mut towelmap, t1, t2, t3);
            }
        }
    }

    towelmap
}

fn add_combo(
    towelmap: &mut HashMap<String, HashSet<Vec<&String>>>,
    t1: &String,
    t2: &String,
    t3: &String,
) {
    add_two_combo(towelmap, t1, t2);
    add_two_combo(towelmap, t1, t3);
    add_two_combo(towelmap, t2, t3);
    add_three_combo(towelmap, t1, t2, t3);
}

fn add_two_combo(
    towelmap: &mut HashMap<String, HashSet<Vec<&String>>>,
    t1: &String,
    t2: &String,
) {
    let s1 = t1.to_owned() + t2;
    let s2 = t2.to_owned() + t1;
    let s1_is_towel = towelmap.contains_key(&s1);
    let s2_is_towel = towelmap.contains_key(&s2);
    if !(s1_is_towel || s2_is_towel) {
        return;
    }
    let paths_to_t1 = towelmap.get(t1).unwrap();
    let paths_to_t2 = towelmap.get(t2).unwrap();
    let mut paths_to_s1 = HashSet::new();
    let mut paths_to_s2 = HashSet::new();
    for path_t1 in paths_to_t1 {
        for path_t2 in paths_to_t2 {
            if s1_is_towel {
                let path_s1: Vec<_> = path_t1.iter().chain(path_t2).cloned().collect();
                paths_to_s1.insert(path_s1);
            }
            if s2_is_towel {
                let path_s2: Vec<_> = path_t2.iter().chain(path_t1).cloned().collect();
                paths_to_s2.insert(path_s2);
            }
        }
    }
    if s1_is_towel {
        towelmap.get_mut(&s1).unwrap().extend(paths_to_s1);
    }
    if s2_is_towel {
        towelmap.get_mut(&s2).unwrap().extend(paths_to_s2);
    }
}

fn add_three_combo(
    towelmap: &mut HashMap<String, HashSet<Vec<&String>>>,
    t1: &String,
    t2: &String,
    t3: &String
) {
    let strs = vec![
        t1.to_owned() + t2 + t3,
        t1.to_owned() + t3 + t2,
        t2.to_owned() + t1 + t3,
        t2.to_owned() + t3 + t1,
        t3.to_owned() + t1 + t2,
        t3.to_owned() + t2 + t1,
    ];
    let towel_paths = vec![
        towelmap.get(t1).unwrap(),
        towelmap.get(t2).unwrap(),
        towelmap.get(t3).unwrap(),
    ];
    let str_is_towel: Vec<_> = strs.iter().map(|s| towelmap.contains_key(s)).collect();
    let ct_str_is_towel = str_is_towel.iter().filter(|p| **p).count();
    if ct_str_is_towel == 0 {
        return;
    }
    let mut str_paths = vec![HashSet::new(); ct_str_is_towel];
    for path_t1 in towel_paths[0] {
        for path_t2 in towel_paths[1] {
            for path_t3 in towel_paths[2] {
                let new_paths = get_new_paths(
                    &str_is_towel,
                    path_t1,
                    path_t2,
                    path_t3
                );
                for p in new_paths.iter().enumerate() {
                    str_paths.get_mut(p.0).unwrap().insert(p.1.clone());
                }
            }
        }
    }

    let mut idx = 0;
    for x in str_is_towel.iter().enumerate() {
        if *x.1 {
            towelmap.get_mut(&strs[x.0]).unwrap().extend(str_paths[idx].clone());
            idx += 1;
        }
    }
}

fn get_new_paths<'a>(
    filter: &Vec<bool>,
    path1: &Vec<&'a String>,
    path2: &Vec<&'a String>,
    path3: &Vec<&'a String>
) -> Vec<Vec<&'a String>> {
    let mut out = vec![];
    if filter[0] {
        out.push(path1.iter().chain(path2).chain(path3).copied().collect());
    }
    if filter[1] {
        out.push(path1.iter().chain(path3).chain(path2).copied().collect());
    }
    if filter[2] {
        out.push(path2.iter().chain(path1).chain(path3).copied().collect());
    }
    if filter[3] {
        out.push(path2.iter().chain(path3).chain(path1).copied().collect());
    }
    if filter[4] {
        out.push(path3.iter().chain(path1).chain(path2).copied().collect());
    }
    if filter[5] {
        out.push(path3.iter().chain(path2).chain(path1).copied().collect());
    }
    out
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
