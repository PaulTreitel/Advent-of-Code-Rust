advent_of_code_2022::solution!(20);

const KEY: i64 = 811589153;

pub fn part_one(input: &str) -> Option<i64> {
    let mut decrypt = get_numbers(input);
    let count = decrypt.len();
    mix_numbers(&mut decrypt, count);
    Some(get_grove_coord_sum(&decrypt))
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut decrypt = get_numbers(input);
    decrypt = decrypt.iter()
        .map(|x| (x.0 * KEY, x.1))
        .collect();
    let count = decrypt.len();
    for _ in 0..10 {
        mix_numbers(&mut decrypt, count);
    }
    Some(get_grove_coord_sum(&decrypt))
}

fn get_grove_coord_sum(decrypt: &[(i64, usize)]) -> i64 {
    let zero_idx = decrypt.iter().position(|x| x.0 == 0).unwrap();
    let p1 = decrypt.get((zero_idx + 1000) % decrypt.len()).unwrap().0;
    let p2 = decrypt.get((zero_idx + 2000) % decrypt.len()).unwrap().0;
    let p3 = decrypt.get((zero_idx + 3000) % decrypt.len()).unwrap().0;
    p1 + p2 + p3
}

fn mix_numbers(decrypt: &mut Vec<(i64, usize)>, count: usize) {
    for i in 0..count {
        let old_idx = decrypt.iter().position(|x| x.1 == i).unwrap() as i64;
        let mut new_idx = old_idx + decrypt.get(old_idx as usize).unwrap().0;
        if new_idx < 0 {
            new_idx -= 1;
        } else if new_idx > count as i64 {
            new_idx += new_idx / count as i64;
        }
        new_idx = new_idx.rem_euclid(count as i64);
        let tmp = decrypt.remove(old_idx as usize);
        decrypt.insert(new_idx as usize, tmp);
    }
}

fn get_numbers(input: &str) -> Vec<(i64, usize)> {
    input
        .lines()
        .enumerate()
        .map(|(c, s)| (s.parse::<i64>().unwrap(), c))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(1623178306));
    }
}
