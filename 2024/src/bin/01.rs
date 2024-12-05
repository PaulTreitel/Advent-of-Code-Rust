use std::collections::HashMap;

advent_of_code_2024::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut list1, mut list2) = split_convert_lists(input);
    list1.sort();
    list2.sort();
    let distances = list1
        .iter()
        .zip(list2)
        .map(|(x, y)| (*x - y).abs() as u32)
        .reduce(|acc, e| acc + e)
        .unwrap();
    Some(distances)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (list1, list2) = split_convert_lists(input);
    let occurrence_table = create_occurence_table(&list2);
    let mut similarity = 0;
    for num in list1 {
        similarity += num * occurrence_table.get(&num).unwrap_or(&0);
    }
    Some(similarity as u32)
}

fn create_occurence_table(lst: &[i32]) -> HashMap<i32, i32> {
    let mut table = HashMap::new();
    for num in lst {
        table.insert(*num, 1 + table.get(num).or(Some(&0)).unwrap());
    }
    table
}

fn split_convert_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    for line in input.lines() {
        let nums: Vec<&str> = line.split_ascii_whitespace().collect();
        let num1 = nums.first().unwrap().parse::<i32>().unwrap();
        let num2 = nums.get(1).unwrap().parse::<i32>().unwrap();
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
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(31));
    }
}
