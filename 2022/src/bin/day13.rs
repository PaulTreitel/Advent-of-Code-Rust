advent_of_code_2022::solution!(13);

use std::{cmp::Ordering};


enum Packet {
    List(Vec<Packet>),
    Number(i32),
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut index_sum = 0;
    let packet_pairs = get_packets(input);
    for index in 0..packet_pairs.len() {
        let (left, right) = packet_pairs.get(index).unwrap();
        if packets_in_order(&left, &right) != Ordering::Greater {
            // println!("packets at {} in order", index + 1);
            index_sum += index + 1;
        }
    }
    Some(index_sum as i32)
}

fn packet_order_num_cmp(left: i32, right: i32) -> i8 {
    if left < right {
        -1
    } else if left > right {
        1
    } else {
        0
    }
}

fn packets_in_order<'a, 'b>(left: &'a &Packet, right: &'b &Packet) -> Ordering {
    let res = packets_in_order_recursive(left, right);
    match  res {
        -1 => Ordering::Less,
        0 => Ordering::Equal,
        _ => Ordering::Greater,
    }
}

fn packets_in_order_recursive(left: &Packet, right: &Packet) -> i8 {
    match (left, right) {
        (Packet::Number(lnum), Packet::Number(rnum)) => {
            packet_order_num_cmp(*lnum, *rnum)
        },
        (Packet::List(_), Packet::Number(rnum)) => {
            let rlist = Packet::List(vec![Packet::Number(*rnum)]);
            packets_in_order_recursive(left, &rlist)
        },
        (Packet::Number(lnum), Packet::List(_)) => {
            let llist = Packet::List(vec![Packet::Number(*lnum)]);
            packets_in_order_recursive(&llist, right)
        },
        (Packet::List(llist), Packet::List(rlist)) => {
            for (l_packet, r_packet) in llist.iter().zip(rlist.iter()) {
                let cmp = packets_in_order_recursive(l_packet, r_packet);
                if cmp != 0 {
                    return cmp;
                }
            }
            if llist.len() > rlist.len() {
                1
            } else if llist.len() == rlist.len() {
                0
            } else {
                -1
            }
        },
    }
}

pub fn part_two(input: &str) -> Option<i32> {
    let packet_pairs = get_packets(input);
    let divider_2 = Packet::List(vec![Packet::List(vec![Packet::Number(2)])]);
    let divider_6 = Packet::List(vec![Packet::List(vec![Packet::Number(6)])]);
    let mut packets: Vec<&Packet> = Vec::new();
    for (p1, p2) in &packet_pairs {
        packets.push(p1);
        packets.push(p2);
    }
    packets.push(&divider_2);
    packets.push(&divider_6);
    packets.sort_by(packets_in_order);
    let mut res = 1;
    for (index, p) in packets.iter().enumerate() {
        match *p {
            Packet::List(a) => match a.get(0) {
                Some(Packet::List(b)) => match b.get(0) {
                    Some(Packet::Number(c)) => {
                        if (*c == 2 || *c == 6) && a.len() == 1 && b.len() == 1 {
                            res *= (index + 1) as i32;
                        }
                    }
                    _ => (),
                }
                _ => (),
            },
            _ => (),
        };
    }
    Some(res)
}

fn get_packets(input: &str) -> Vec<(Packet, Packet)> {
    let mut result = Vec::<(Packet, Packet)>::new();
    let mut input2 = input.clone().lines();
    for _ in input.lines().enumerate().step_by(3) {
        let p1 = input2.next().unwrap();
        let p2 = input2.next().unwrap();
        input2.next();
        result.push((parse_packet(p1), parse_packet(p2)));
    }
    result
}

// taken from https://www.reddit.com/r/adventofcode/comments/zkmyh4/comment/j01mqo7/
// as I had no clue how to parse this day's inputs given the rigidity of Rust's Vectors
fn parse_packet(s: &str) -> Packet {
    if &s[0..1] == "[" {
        let mut stack: i32 = 0;
        Packet::List(
            s[1..s.len() - 1]
                .split(|c| {
                    if c == '[' {
                        stack += 1
                    } else if c == ']' {
                        stack -= 1
                    }
                    c == ',' && stack == 0
                })
                .filter_map(|s| (!s.is_empty()).then(|| parse_packet(s)))
                .collect(),
        )
    } else {
        Packet::Number(s.parse().unwrap())
    }
}

fn main() {
    let input = advent_of_code_2022::template::read_file("inputs", DAY);
    let res = part_one(&input).unwrap();
    println!("{}", res);
    let res = part_two(&input).unwrap();
    println!("{}", res);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(13)); // fill in
    }
    #[test]
    fn test_part_two() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(140)); // fill in
    }
}
