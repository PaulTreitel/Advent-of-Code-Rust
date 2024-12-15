advent_of_code_2022::solution!(7);

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

const DIRSIZE_FILTER: i32 = 100000;
const DISKSPACE: i32 = 70000000;
const DISKSPACE_NEEDED: i32 = 30000000;

struct Directory {
    size: i32,
    child_dirs: Vec<String>,
}

pub fn part_one(input: &str) -> Option<i32> {
    let fs_contents = build_filesystem_hashmap(input);
    let result: i32 = fs_contents
        .values()
        .map(|dir| dir.size)
        .filter(|x| *x < DIRSIZE_FILTER)
        .sum();
    Some(result)
}

fn build_filesystem_hashmap(input: &str) -> HashMap<String, Directory> {
    let mut fs_contents: HashMap<String, Directory> = HashMap::new();
    let mut curr_path = PathBuf::new();
    for line in input.lines() {
        let mut parts = line.split_ascii_whitespace();
        let first = parts.next().unwrap();
        if first.eq("$") {
            if parts.next().unwrap().eq("cd") {
                handle_cd(&mut fs_contents, &mut curr_path, parts.next().unwrap());
            }
        } else if first.eq("dir") {
            let name = format!("{}{}", curr_path.to_str().unwrap(), parts.next().unwrap());
            handle_dir(&mut fs_contents, &curr_path, &name);
        } else {
            add_sizes(&mut fs_contents, &curr_path, first.parse().unwrap());
        }
    }
    fs_contents
}

fn add_sizes(fs_contents: &mut HashMap<String, Directory>, path: &Path, size: i32) {
    let mut path_str = path.to_str().unwrap();
    fs_contents.get_mut(path_str).unwrap().size += size;
    let mut path = path.to_path_buf();
    while path.parent().is_some() {
        path.pop();
        path_str = path.to_str().unwrap();
        fs_contents.get_mut(path_str).unwrap().size += size;
    }
}

fn handle_dir(fs_contents: &mut HashMap<String, Directory>, curr_path: &Path, name: &String) {
    let new_subdir = Directory {
        size: 0,
        child_dirs: Vec::new(),
    };
    fs_contents
        .get_mut(&curr_path.to_str().unwrap().to_string())
        .unwrap()
        .child_dirs
        .push(name.to_string());
    fs_contents.entry(name.to_string()).or_insert(new_subdir);
}

fn handle_cd(
    fs_contents: &mut HashMap<String, Directory>,
    curr_path: &mut PathBuf,
    path_change: &str,
) {
    if path_change.eq("..") {
        curr_path.pop();
    } else {
        curr_path.push(path_change);
        let new_path = curr_path.to_str().unwrap().to_string();
        fs_contents.entry(new_path).or_insert(Directory {
            size: 0,
            child_dirs: Vec::new(),
        });
    }
}

pub fn part_two(input: &str) -> Option<i32> {
    let fs_contents = build_filesystem_hashmap(input);
    let free_space = DISKSPACE - fs_contents.get("/").unwrap().size;
    let target_dir_size = DISKSPACE_NEEDED - free_space;

    let mut candidate_dirs = fs_contents
        .iter()
        .filter(|(_, d)| d.size >= target_dir_size)
        .map(|d| d.1.size)
        .collect::<Vec<i32>>();
    candidate_dirs.sort();
    Some(*candidate_dirs.first().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(24933642));
    }
}
