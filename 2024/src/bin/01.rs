use std::collections::HashMap;

advent_of_code_2024::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut list1, mut list2) = split_convert_lists(input);
    list1.sort();
    list2.sort();
    let mut distance = 0;
    for i in 0..list1.len() {
        let first = list1.get(i).unwrap();
        let second = list2.get(i).unwrap();
        distance += (*first as i32 - *second as i32).abs();
    }
    Some(distance as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (list1, list2) = split_convert_lists(input);
    let occurrence_table = create_occurence_table(&list2);
    let mut similarity = 0;
    for num in list1 {
        similarity += num * occurrence_table.get(&num).unwrap_or(&0);
    }
    Some(similarity)
}

fn create_occurence_table(lst: &[u32]) -> HashMap<u32, u32> {
    let mut table = HashMap::new();
    for num in lst {
        if table.contains_key(num) {
            *table.get_mut(num).unwrap() += 1;
        } else {
            table.insert(*num, 1);
        }
    }
    table
}

fn split_convert_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    let lines = input.lines();
    for line in lines {
        let nums: Vec<&str> = line.split_ascii_whitespace().collect();
        let num1 = nums.first().unwrap().parse::<u32>().unwrap();
        let num2 = nums.get(1).unwrap().parse::<u32>().unwrap();
        list1.push(num1);
        list2.push(num2);
    }
    (list1, list2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code_2024::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code_2024::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
