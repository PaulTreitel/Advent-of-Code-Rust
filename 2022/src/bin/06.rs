advent_of_code_2022::solution!(6);

pub fn part_one(input: &str) -> Option<i32> {
    let mut input = input.chars();
    let mut chars = (
        input.next().unwrap(), input.next().unwrap(), input.next().unwrap(), input.next().unwrap()
    );
    if are_unique(chars) {
        return Some(4);
    }
    for index in input.enumerate() {
        // println!("{}", index.1);
        chars = (chars.1, chars.2, chars.3, index.1);
        if are_unique(chars) {
            return Some(index.0 as i32 + 5);
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<i32> {
    let input = input.chars();
    let input: Vec<_> = input.collect();
    for index in 0..(input.len() - 14) {
        // let x = input[index..index+14];
        if is_unique(&input[index..index + 14]) {
            return Some(index as i32 + 14);
        }
    }
    None
}

fn is_unique(chars: &[char]) -> bool {
    for index in 0..chars.len() {
        if chars[index+1..].contains(&chars[index]) {
            return false;
        }
    }
    true
}

fn are_unique(chars: (char, char, char, char)) -> bool {
    let char0_match = chars.0 == chars.1 || chars.0 == chars.2 || chars.0 == chars.3;
    let char1_match = chars.1 == chars.2 || chars.1 == chars.3;
    let char2_match = chars.2 == chars.3;
    // println!("{} {} {} {} -> {}", chars.0, chars.1, chars.2, chars.3, !(char0_match || char1_match || char2_match));
    !(char0_match || char1_match || char2_match)
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
