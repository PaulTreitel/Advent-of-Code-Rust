advent_of_code::solution!(3);


pub fn part_one(input: &str) -> Option<u32> {
    let parens_split: Vec<&str> = input.split(|c| c == '(' || c == ')')
        .collect();
    let operands = get_all_operands_part_1(&parens_split);
    let sum = operands.iter()
        .map(|x| x.0 * x.1)
        .reduce(|acc, x| acc + x)
        .unwrap();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let parens_split: Vec<&str> = input.split(|c| c == '(' || c == ')')
        .collect();
    let operands = get_all_operands_part_2(&parens_split);
    let sum = operands.iter()
        .map(|x| x.0 * x.1)
        .reduce(|acc, x| acc + x)
        .unwrap();
    Some(sum)
}

fn get_all_operands_part_2(parens_split: &Vec<&str>) -> Vec<(u32, u32)> {
    let mut operands = Vec::new();
    let mut do_state = true;
    for idx in 1..parens_split.len() {
        let prev = parens_split.get(idx - 1).unwrap();
        let curr = parens_split.get(idx).unwrap();

        if curr.len() == 0 && does_previous_match(prev, "do") {
            do_state = true;
        } else if curr.len() == 0 && does_previous_match(prev, "don't") {
            do_state = false;
        } else if does_previous_match(prev, "mul") && do_state {
            let curr: Vec<&str> = curr .split(",").collect();
            let curr_operands = get_single_operands(&curr);
            if let Some(ops) = curr_operands {
                operands.push(ops);
            }
        }
    }
    operands
}

fn get_all_operands_part_1(parens_split: &Vec<&str>) -> Vec<(u32, u32)> {
    let mut operands = Vec::new();
    for idx in 1..parens_split.len() {
        let prev = parens_split.get(idx - 1).unwrap();
        if !does_previous_match(prev, "mul") {
            continue;
        }

        let curr: Vec<&str> = parens_split.get(idx)
            .unwrap()
            .split(",")
            .collect();
        let curr_operands = get_single_operands(&curr);
        if let Some(ops) = curr_operands {
            operands.push(ops);
        }
    }
    operands
}

fn does_previous_match(prev: &str, cmp: &str) -> bool {
    if prev.len() < cmp.len() {
        return false;
    }
    let pos = prev.char_indices()
        .nth_back(cmp.len() - 1)
        .unwrap()
        .0;
    &prev[pos..] == cmp
}

fn get_single_operands(in_parens: &Vec<&str>) -> Option<(u32, u32)> {
    if in_parens.len() != 2 {
        return None;
    }
    let x = in_parens.get(0)
        .unwrap()
        .parse::<u32>();
    let y = in_parens.get(1)
    .unwrap()
    .parse::<u32>();
    if x.is_err() || y.is_err() {
        return None;
    }
    Some((x.unwrap(), y.unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = part_two(input);
        assert_eq!(result, Some(48));
    }
}
