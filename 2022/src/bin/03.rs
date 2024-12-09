advent_of_code_2022::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut total_priority = 0;
    for line in input.lines() {
        let sack_halves = split_rucksack(line);
        let shared_item = get_shared_char_two_bags(sack_halves).unwrap();
        let priority = char_to_priority(shared_item).unwrap();
        total_priority += priority;
    }
    Some(total_priority)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total_priority: u32 = 0;
    let mut input2 = input.lines();

    for _ in input.lines().enumerate().step_by(3) {
        let sack_one: &str = input2.next().unwrap();
        let sack_two: &str = input2.next().unwrap();
        let sack_three: &str = input2.next().unwrap();
        let shared_item = get_shared_char_three_bags((sack_one, sack_two, sack_three)).unwrap();
        let priority = char_to_priority(shared_item).unwrap();
        total_priority += priority;
    }
    Some(total_priority)
}

fn char_to_priority(ch: char) -> Option<u32> {
    assert!(ch.is_ascii());
    if ch.is_lowercase() {
        Some((ch as u32) - ('a' as u32) + 1)
    } else if ch.is_uppercase() {
        Some((ch as u32) - ('A' as u32) + 27)
    } else {
        None
    }
}

fn split_rucksack(sack: &str) -> (&str, &str) {
    let num_sack_items = sack.chars().count();
    let midpoint = num_sack_items / 2;
    (&sack[0..midpoint], &sack[midpoint..])
}

fn get_shared_char_three_bags(sack: (&str, &str, &str)) -> Option<char> {
    sack.0.chars().find(|&sack0_item| sack.1.contains(sack0_item) && sack.2.contains(sack0_item))
}

fn get_shared_char_two_bags(sack: (&str, &str)) -> Option<char> {
    sack.0.chars().find(|&sack0_item| sack.1.contains(sack0_item))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(70));
    }
}
