advent_of_code_2022::solution!(7);

use std::collections::HashMap;

const DIRSIZE_FILTER: i32 = 100000;
const DISKSPACE: i32 = 70000000;
const DISKSPACE_NEEDED: i32 = 30000000;

struct Directory {
    size: i32,
    child_dirs: Vec<String>,
}

pub fn part_one(input: &str) -> Option<i32> {
    let fs_contents = build_filesystem_hashmap(input);
    let dir_sizes = fs_contents.iter().map(|(_, dir)| dir.size);
    let result: i32 = dir_sizes.filter(|x| *x < DIRSIZE_FILTER).sum();
    Some(result)
}

fn build_filesystem_hashmap(input: &str) -> HashMap<String, Directory> {
    let mut fs_contents: HashMap<String, Directory> = HashMap::new();
    let mut curr_path = String::new();
    let input = input.lines();
    for line in input {
        let mut parts = line.split_ascii_whitespace();
        let first = parts.next().unwrap();
        let mut path_addition = String::new();
        if first.eq("$") {
            if parts.next().unwrap().eq("cd") {
                path_addition = handle_cd(&mut fs_contents, &mut curr_path, parts.next().unwrap());
            }
        } else if first.eq("dir") {
            let name = String::new() + parts.next().unwrap() + "/";
            handle_dir(&mut fs_contents, &curr_path, &name);
        } else {
            let size: i32 = first.parse().unwrap();
            add_sizes(&mut fs_contents, &mut curr_path, size);
        }
        curr_path.push_str(&path_addition);
    }
    fs_contents
}

fn add_sizes(fs_contents: &mut HashMap<String, Directory>, path: &mut String, size: i32) {
    let top_dir = fs_contents.get_mut(path).unwrap();
    top_dir.size += size;
    if path == "/" {
        return;
    }
    let mut temp_stack = String::new();
    let mut temp_char;
    while path != "" {
        temp_char = path.pop().unwrap();
        temp_stack.push(temp_char);
        temp_char = ' ';
        while temp_char != '/' && path.len() > 0 {
            temp_char = path.pop().unwrap();
            temp_stack.push(temp_char);
        }
        path.push('/');
        let dir = fs_contents.get_mut(path).unwrap();
        dir.size += size;
        path.pop();
    }
    while temp_stack != "" {
        path.push(temp_stack.pop().unwrap());
    }
}

fn handle_dir(fs_contents: &mut HashMap<String, Directory>, curr_path: &String, name: &String) {
    let name = String::new() + curr_path + name;
    let new_subdir = Directory{ size: 0, child_dirs: Vec::new() };
    fs_contents.get_mut(&curr_path.to_string()).unwrap().child_dirs.push(name.to_string());
    fs_contents.entry(name.to_string()).or_insert(new_subdir);
}

fn handle_cd(fs_contents: &mut HashMap<String, Directory>, curr_path: &mut String, path_change: &str) -> String {
    let mut path_addition = String::new();
    if path_change.eq("..") {
        curr_path.pop();
        while curr_path.pop().unwrap().ne(&'/') {

        }
        curr_path.push('/');
    } else {
        path_addition.push_str(path_change);
        if path_change.ne("/") {
            path_addition.push('/');
        }
        let tmp = String::new();
        let tmp = tmp + &curr_path + &path_addition;
        fs_contents.entry(tmp.clone()).or_insert(Directory { size: 0, child_dirs: Vec::new() });
    }
    path_addition
}

pub fn part_two(input: &str) -> Option<i32> {
    let fs_contents = build_filesystem_hashmap(input);
    let free_space = DISKSPACE - fs_contents.get("/").unwrap().size;
    let target_dir_size = DISKSPACE_NEEDED - free_space;

    let candidate_dirs = fs_contents.iter().filter(|(_, d)| d.size >= target_dir_size);
    let mut candidate_dirs: Vec<i32> = candidate_dirs.map(|d| d.1.size).collect();
    candidate_dirs.sort_unstable();
    Some(*candidate_dirs.get(0).unwrap())
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
        assert_eq!(part_one(&input), Some(95437)); // fill in
    }
    #[test]
    fn test_part_two() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(24933642)); // fill in
    }
}
