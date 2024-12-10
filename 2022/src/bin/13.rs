advent_of_code_2022::solution!(13);

use std::cmp::Ordering;


enum Packet {
    List(Vec<Packet>),
    Number(i32),
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut index_sum = 0;
    let packet_pairs = get_packets(input);
    for index in 0..packet_pairs.len() {
        let (left, right) = packet_pairs.get(index).unwrap();
        if packets_in_order(left, right) != Ordering::Greater {
            index_sum += index + 1;
        }
    }
    Some(index_sum as i32)
}

fn packets_in_order(left: &Packet, right: &Packet) -> Ordering {
    match (left, right) {
        (Packet::Number(lnum), Packet::Number(rnum)) => {
            lnum.cmp(rnum)
        },
        (Packet::List(_), Packet::Number(rnum)) => {
            let rlist = Packet::List(vec![Packet::Number(*rnum)]);
            packets_in_order(left, &rlist)
        },
        (Packet::Number(lnum), Packet::List(_)) => {
            let llist = Packet::List(vec![Packet::Number(*lnum)]);
            packets_in_order(&llist, right)
        },
        (Packet::List(llist), Packet::List(rlist)) => {
            for (l_packet, r_packet) in llist.iter().zip(rlist.iter()) {
                let cmp = packets_in_order(l_packet, r_packet);
                if cmp != Ordering::Equal {
                    return cmp;
                }
            }
            llist.len().cmp(&rlist.len())
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
    packets.sort_by(|l: &&Packet, r: &&Packet| packets_in_order(l, r));
    let mut res = 1;
    for (index, p) in packets.iter().enumerate() {
        if let Packet::List(a) =  *p {
            if let Some(Packet::List(b)) =  a.first() {
                if let Some(Packet::Number(c)) =  b.first() {
                    if (*c == 2 || *c == 6) && a.len() == 1 && b.len() == 1 {
                        res *= (index + 1) as i32;
                    }
                }
            }
        };
    }
    Some(res)
}

fn get_packets(input: &str) -> Vec<(Packet, Packet)> {
    let mut result = Vec::<(Packet, Packet)>::new();
    let mut input2 = input.lines();
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
                .filter(|s| !s.is_empty())
                .map(parse_packet)
                .collect(),
        )
    } else {
        Packet::Number(s.parse().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(140));
    }
}
