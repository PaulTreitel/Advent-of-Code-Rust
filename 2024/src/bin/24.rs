use std::collections::HashMap;

use advent_of_code_2024::utils::show;

advent_of_code_2024::solution!(24);

enum Operation {
    AND,
    OR,
    XOR,
}

struct Gate {
    lhs: String,
    rhs: String,
    op: Operation,
    res: String,
}

impl Operation {
    pub fn from_str(s: &str) -> Self {
        match s {
            "AND" => Self::AND,
            "OR" => Self::OR,
            "XOR" => Self::XOR,
            _ => unreachable!()
        }
    }

    pub fn exec(&self, lhs: u8, rhs: u8) -> u8 {
        match self {
            Self::AND => lhs & rhs,
            Self::OR => lhs | rhs,
            Self::XOR => lhs ^ rhs,
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut wires, mut gates) = parse_input(input);
    while !gates.is_empty() {
        for g_idx in (0..gates.len()).rev() {
            let g = gates.get_mut(g_idx).unwrap();
            if wires.contains_key(&g.lhs) && wires.contains_key(&g.rhs) {
                let lhs = *wires.get(&g.lhs).unwrap();
                let rhs = *wires.get(&g.rhs).unwrap();
                let out_val = g.op.exec(lhs, rhs);
                wires.insert(g.res.clone(), out_val);
                gates.remove(g_idx);
            }
        }
    }
    let mut z_wires = vec![];
    for wire in wires {
        if wire.0.starts_with("z") {
            z_wires.push(wire);
        }
    }
    z_wires.sort();
    z_wires.reverse();
    let mut out: u64 = 0;
    for z_wire in z_wires {
        out = out << 1;
        out += z_wire.1 as u64;
    }
    Some(out)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn parse_input(input: &str) -> (HashMap<String, u8>, Vec<Gate>) {
    let mut input = input.split("\n\n");
    let wires = get_wires(input.next().unwrap());
    let gates = get_gates(input.next().unwrap());
    (wires, gates)
}

fn get_wires(input: &str) -> HashMap<String, u8> {
    let mut wires = HashMap::new();
    for line in input.lines() {
        let mut line = line.split(": ");
        wires.insert(
            line.next().unwrap().to_string(),
            line.next().unwrap().parse::<u8>().unwrap()
        );
    }
    wires
}

fn get_gates(input: &str) -> Vec<Gate> {
    let mut gates = vec![];
    for line in input.lines() {
        let mut line = line.split(" ");
        let lhs = line.next().unwrap().to_string();
        let op = Operation::from_str(line.next().unwrap());
        let rhs = line.next().unwrap().to_string();
        line.next();
        let res = line.next().unwrap().to_string();
        let g = Gate { lhs, rhs, op, res };
        gates.push(g);
    }
    gates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example_one() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_example_two() {
        let input = advent_of_code_2024::template::read_file_part("examples", DAY, 2);
        let result = part_one(&input);
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, None);
    }
}
