pub fn split_vertical_lists<T: Clone, F: Copy + Fn(&&str) -> T>(
    input: &str,
    separator: impl Fn(&str) -> Vec<&str>,
    converter: F,
) -> Vec<Vec<T>> {
    let mut new_lists = Vec::new();
    for line in input.lines() {
        add_to_split_list(&mut new_lists, separator(line), converter);
    }
    new_lists
}

fn add_to_split_list<T: Clone, F: Copy + Fn(&&str) -> T>(
    lists: &mut Vec<Vec<T>>,
    line: Vec<&str>,
    converter: F,
) {
    for (idx, t) in line.iter().map(converter).enumerate() {
        if lists.len() <= idx {
            lists.push(vec![t]);
        } else {
            lists.get_mut(idx).unwrap().push(t);
        }
    }
}

pub fn split_two_vertical_lists<T: Clone, F: Copy + Fn(&&str) -> T>(
    input: &str,
    separator: impl Fn(&str) -> Vec<&str>,
    converter: F,
) -> (Vec<T>, Vec<T>) {
    let split_lists = split_vertical_lists(input, separator, converter);
    (
        split_lists.first().unwrap().to_vec(),
        split_lists.get(1).unwrap().to_vec(),
    )
}

pub fn into_2d_array<T: Clone, F: Copy + Fn(&&str) -> T>(
    input: &str,
    separator: impl Fn(&str) -> Vec<&str>,
    converter: F,
) -> Vec<Vec<T>> {
    input
        .lines()
        .map(|x| separator(x).iter().map(converter).collect())
        .collect()
}

pub fn split_by_all_chars(s: &str) -> Vec<&str> {
    s.split("").filter(|&s| !s.eq("")).collect()
}
