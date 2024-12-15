advent_of_code_2022::solution!(6);

pub fn part_one(input: &str) -> Option<i32> {
    find_start(input, 4)
}

pub fn part_two(input: &str) -> Option<i32> {
    find_start(input, 14)
}

fn find_start(input: &str, len: usize) -> Option<i32> {
    let input: Vec<_> = input.chars().collect();
    for index in 0..(input.len() - len) {
        if is_unique(&input[index..index + len]) {
            return Some(index as i32 + len as i32);
        }
    }
    None
}

fn is_unique(chars: &[char]) -> bool {
    for index in 0..chars.len() {
        if chars[index + 1..].contains(&chars[index]) {
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
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(19));
    }
}
