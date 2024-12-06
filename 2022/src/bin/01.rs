advent_of_code_2022::solution!(1);

fn part_two(input: &str) -> Option<u32> {
    let mut calorie_counts = parse_input(input);
    calorie_counts.sort();
    calorie_counts.reverse();
    let top_3_elf_calories: u32 = calorie_counts[0..3].iter().sum();
    Some(top_3_elf_calories)
}

fn part_one(input: &str) -> Option<u32> {
    let calorie_counts = parse_input(input);
    let max_calories = calorie_counts.iter().max().unwrap();
    Some(*max_calories)
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|x| x
            .lines()
            .map(|y| y.parse::<u32>().unwrap())
            .sum()
        )
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code_2022::template::read_file("examples", DAY));
        assert_eq!(result, Some(24000));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code_2022::template::read_file("examples", DAY));
        assert_eq!(result, Some(45000));
    }
}
