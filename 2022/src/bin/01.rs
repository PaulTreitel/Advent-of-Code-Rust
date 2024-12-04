advent_of_code_2022::solution!(1);

fn part_two(input: &str) -> Option<u32> {
    let mut calorie_counts = get_elf_calorie_list(input);
    calorie_counts.sort();
    calorie_counts.reverse();
    let top_3_elf_calories: u32 = calorie_counts[0..3].iter().sum();
    Some(top_3_elf_calories)
}

fn part_one(input: &str) -> Option<u32> {
    let calorie_counts = get_elf_calorie_list(input);
    let max_calories = calorie_counts.iter().max().unwrap();
    Some(*max_calories)
}

fn get_elf_calorie_list(input: &str) -> Vec<u32> {
    let input = input.lines();
    let mut calorie_counts: Vec<u32> = vec![0];
    let mut elf_index = 0;
    for line in input {
        if line.len() == 0 {
            elf_index += 1;
        } else {
            if calorie_counts.len() == elf_index {
                calorie_counts.push(0);
            }
            let item_calories: u32 = line.parse().unwrap();
            calorie_counts[elf_index] += item_calories;
        }
    }
    calorie_counts
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
