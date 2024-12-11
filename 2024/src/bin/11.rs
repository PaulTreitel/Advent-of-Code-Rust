use std::collections::HashMap;

advent_of_code_2024::solution!(11);

const PART_ONE_ROUNDS: i32 = 25;
const PART_TWO_ROUNDS: i32 = 75;

struct Round {
    zero: u64,
    one: u64,
    two: u64,
    three: u64,
    four: u64,
    five: u64,
    six: u64,
    seven: u64,
    eight: u64,
    nine: u64,
    other: HashMap<u64, u64> // stone value -> count
}

impl Round {
    pub fn from(stones: &Vec<u64>) -> Self {
        let mut ct = [0; 10];
        let mut extra = HashMap::new();
        for &stone in stones {
            if stone < 10 {
                ct[stone as usize] += 1;
            } else {
                extra
                    .entry(stone)
                    .and_modify(|x| *x += 1)
                    .or_insert(1);
            }
        }
        Round {
            zero: ct[0],
            one: ct[1],
            two: ct[2],
            three: ct[3],
            four: ct[4],
            five: ct[5],
            six: ct[6],
            seven: ct[7],
            eight: ct[8],
            nine: ct[9],
            other: extra,
        }
    }

    pub fn next_round(&self) -> Self {
        let mut ct = [0; 10];
        let mut extra = HashMap::new();
        ct[1] += self.zero;
        extra.insert(2024, self.one);
        extra.insert(4048, self.two);
        extra.insert(6072, self.three);
        extra.insert(8096, self.four);
        extra.insert(10120, self.five);
        extra.insert(12144, self.six);
        extra.insert(14168, self.seven);
        extra.insert(16192, self.eight);
        extra.insert(18216, self.nine);
        for (&stone, &stone_count) in &self.other {
            let new_stones = get_new_stones(stone);
            for s in new_stones {
                if s < 10 {
                    ct[s as usize] += stone_count;
                } else {
                    extra
                    .entry(s)
                    .and_modify(|x| *x += stone_count)
                    .or_insert(stone_count);
                }
            }
        }
        Round {
            zero: ct[0],
            one: ct[1],
            two: ct[2],
            three: ct[3],
            four: ct[4],
            five: ct[5],
            six: ct[6],
            seven: ct[7],
            eight: ct[8],
            nine: ct[9],
            other: extra,
        }
    }

    pub fn num_rocks(&self) -> u64 {
        let mut sum = self.zero + self.one + self.two + self.three + self.four
            + self.five + self.six + self.seven + self.eight + self.nine;
        sum += self.other.values().sum::<u64>();
        sum
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let stones = parse_input(input);
    let mut curr_round = Round::from(&stones);
    for _ in 0..PART_ONE_ROUNDS {
        curr_round = curr_round.next_round();
    }
    Some(curr_round.num_rocks())
}

pub fn part_two(input: &str) -> Option<u64> {
    let stones = parse_input(input);
    let mut curr_round = Round::from(&stones);
    for _ in 0..PART_TWO_ROUNDS {
        curr_round = curr_round.next_round();
    }
    Some(curr_round.num_rocks())
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .trim()
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

fn get_new_stones(stone: u64) -> Vec<u64> {
    let mut res = vec![];
    let str_stone = stone.to_string();
    if stone == 0 {
        res.push(1);
    } else if str_stone.len() % 2 == 0 {
        let div_point = str_stone.len() / 2;
        let first = &str_stone[..div_point];
        let second = &str_stone[div_point..];
        res.push(first.parse().unwrap());
        res.push(second.parse().unwrap());
    } else {
        res.push(stone * 2024);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, None);
    }
}
