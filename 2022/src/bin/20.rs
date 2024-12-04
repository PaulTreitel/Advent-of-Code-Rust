advent_of_code_2022::solution!(20);

const KEY: i64 = 811589153;

pub fn part_one(input: &str) -> Option<i64> {
    let mut decrypt = get_numbers(input);
    let count = decrypt.len();
    mix_numbers(&mut decrypt, count);
    let zero_idx = decrypt.iter().position(|x| x.0 == 0).unwrap();
    let p1 = decrypt.get((zero_idx + 1000) % count as usize).unwrap().0;
    let p2 = decrypt.get((zero_idx + 2000) % count as usize).unwrap().0;
    let p3 = decrypt.get((zero_idx + 3000) % count as usize).unwrap().0;
    println!("{}, {}, {}", p1, p2, p3);
    Some(p1 + p2 + p3)
    // None
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut decrypt = get_numbers(input);
    decrypt = decrypt.iter()
        .map(|x| (x.0 * KEY, x.1))
        .collect();
    let count = decrypt.len();
    println!("decrypt = {:?}", decrypt);
    for _ in 0..10 {
        mix_numbers(&mut decrypt, count);
        println!("decrypt = {:?}", decrypt);
        println!("\n");
    }
    None
}

fn mix_numbers(decrypt: &mut Vec<(i64, i32)>, count: usize) -> () {
    for i in 0..count {
        let old_idx = get_next_start_index(&decrypt, i) as i64;
        let mut new_idx = old_idx + decrypt.get(old_idx as usize).unwrap().0;
        if new_idx < 0 {
            new_idx -= 1;
        } else if new_idx > count as i64 {
            new_idx += new_idx / count as i64;
        }
        new_idx = new_idx.rem_euclid(count as i64);
        let tmp = decrypt.remove(old_idx as usize);
        decrypt.insert(new_idx as usize, tmp);
        println!("{:?}", decrypt);
    }
}

fn get_next_start_index(vals: &Vec<(i64, i32)>, start_idx: usize) -> usize {
    vals.iter().position(|x| x.1 == start_idx as i32).unwrap()
}

fn get_numbers(input: &str) -> Vec<(i64, i32)> {
    let mut nums = Vec::new();
    let mut counter = 0;
    for line in input.lines() {
        nums.push((line.parse::<i64>().unwrap(), counter));
        counter += 1;
    }
    nums
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
