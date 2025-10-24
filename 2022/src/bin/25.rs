// Solving https://adventofcode.com/2022/day/25
advent_of_code_2022::solution!(25);

use std::collections::HashMap;

const BASE: i64 = 5;

pub fn part_one(input: &str) -> Option<String> {
    let mut snafus = get_snafu_numbers(input);
    let total_fuel = sum_snafus(&mut snafus);
    let place_vals = i64_to_snafu_place_values(total_fuel);
    Some(vals_to_snafu(&place_vals))
}

pub fn part_two(_input: &str) -> Option<i32> {
    None
}

fn vals_to_snafu(place_vals: &HashMap<u32, i8>) -> String {
    let mut chars = Vec::new();
    let mut places: Vec<u32> = place_vals.keys().map(|x| *x).collect();
    places.sort();
    let max_place = places.pop().unwrap();
    for p in (0..max_place+1).rev() {
        if place_vals.contains_key(&p) {
            chars.push(snafu_char_from_digit(*place_vals.get(&p).unwrap()));
        } else {
            chars.push('0');
        }
    }
    String::from_utf8(chars.iter().map(|x| *x as u8).collect()).unwrap()
}

fn i64_to_snafu_place_values(n: i64) -> HashMap<u32, i8> {
    let mut place_values = HashMap::new();
    let mut diff = n;
    while diff.abs() > 2 {
        let new_place_val = get_next_place_value(diff);
        place_values.insert(new_place_val.0, new_place_val.1);
        diff -= new_place_val.1 as i64 * BASE.pow(new_place_val.0);
        // println!("{:?}\n{}", place_values, diff);
    }
    place_values.insert(0, diff as i8);
    place_values
}

fn get_next_place_value(n: i64) -> (u32, i8) {
    let mut next_place = (0, 0);
    for place in 1..27 {
        let exp = BASE.pow(place);
        let next_exp = BASE.pow(place - 1);
        let cond1 = n > 0 && n - 3 * exp > 0;
        let cond2 = n < 0 && n + 3 * exp < 0;
        if cond1 || cond2 {
            continue;
        }
        for val in [-2, -1, 2, 1] {
            if (n - val * exp).abs() < 3 * next_exp {
                next_place = (place, val as i8);
                break;
            }
        }
        if next_place != (0, 0) {
            break;
        }
    }
    next_place
}

fn sum_snafus(snafus: &mut Vec<Vec<i8>>) -> i64 {
    let mut snafu_sum = 0;
    for snafu in snafus {
        let mut val: i64 = 0;
        let len = snafu.len();
        for power in  0..len as u32 {
            val += snafu.pop().unwrap() as i64 * BASE.pow(power);
        }
        snafu_sum += val;
    }
    snafu_sum
}

fn get_snafu_numbers(input: &str) -> Vec<Vec<i8>> {
    let mut numbers = Vec::new();
    for line in input.lines() {
        let mut value = Vec::new();
        for ch in line.chars() {
            value.push(snafu_digit_from_char(ch));
        }
        numbers.push(value);
    }
    numbers
}

fn snafu_char_from_digit(d: i8) -> char {
    match d {
        -2 => '=',
        -1 => '-',
        0 => '0',
        1 => '1',
        2 => '2',
        _ => panic!("number {} was not a valid snafu value", d),
    }
}

fn snafu_digit_from_char(c: char) -> i8 {
    match c {
        '=' => -2,
        '-' => -1,
        '0' => 0,
        '1' => 1,
        '2' => 2,
        _ => panic!("character {} was not a valid snafu digit", c),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some("2=-1=0".to_string()));
    }
}
