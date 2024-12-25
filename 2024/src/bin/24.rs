use std::{collections::{HashMap, HashSet}, str::FromStr};

use advent_of_code_2024::utils::show;
use num::Integer;

advent_of_code_2024::solution!(24);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Operation {
    AND,
    OR,
    XOR,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    let (mut wires, gates) = parse_input(input);
    execute_gates(&mut wires, &gates);
    Some(get_wire_num(&wires, 'z'))
}

pub fn part_two(input: &str) -> Option<String> {
    let (wires, gates) = parse_input(input);
    let mut wires2 = wires.clone();
    let x_num = get_wire_num(&wires, 'x');
    let y_num = get_wire_num(&wires, 'y');
    let target = x_num + y_num;
    execute_gates(&mut wires2, &gates);
    let curr_out = get_wire_num(&wires2, 'z');
    let diff_bits = target ^ curr_out;
    println!("target, current, diff\n{:#b}\n{:#b}\n{:#048b}", target, curr_out, diff_bits);
    println!("{} different bits", u64::count_ones(diff_bits));
    println!("there are {} wires", wires2.len());
    let z_bit_names = get_set_z_names(diff_bits);
    println!("bit names: {:?}", z_bit_names);
    let possible_swaps = get_wires_that_set_bits(&z_bit_names, &gates);
    println!("names: {}", possible_swaps.len());
    show::pretty_print_hmap(&possible_swaps, true, false);
    None
}

fn parse_input(input: &str) -> (HashMap<String, u8>, Vec<Gate>) {
    let mut input = input.split("\n\n");
    let wires = get_wires(input.next().unwrap());
    let gates = get_gates(input.next().unwrap());
    (wires, gates)
}

fn get_wires_that_set_bits(bit_names: &Vec<String>, gates: &Vec<Gate>
) -> HashMap<String, HashSet<String>> {
    let mut wire_names: HashMap<String, HashSet<String>> = HashMap::new();
    let mut added = 0;
    while added != 0 || wire_names.len() == 0 {
        // println!("new cycle, current wire names");
        // show::pretty_print_hmap(&wire_names, false, false);
        println!();
        added = 0;
        for g in gates {
            if bit_names.contains(&g.res) || wire_names.contains_key(&g.res) {
                if !g.lhs.starts_with("x")
                    && !g.lhs.starts_with("y")
                {
                    let affects_z = g.res.starts_with('z');
                    // if !wire_names.contains_key(&g.lhs) {

                        // added.inc();
                    // }
                    if affects_z {
                        if !wire_names.contains_key(&g.lhs)
                            || !wire_names.get(&g.lhs).unwrap().contains(&g.res)
                        {
                            // println!("lhs adding res {} to {} entry",
                            //     g.res, g.lhs);
                            wire_names.entry(g.lhs.clone())
                                .and_modify(|v: &mut HashSet<String>| {
                                    // println!("a");
                                    v.insert(g.res.clone());
                                })
                                .or_insert({
                                    // println!("b");
                                    HashSet::from_iter(vec![g.res.clone()])
                                });
                            added.inc();
                        }
                    } else {
                        let bits_affected = wire_names.get(&g.res).unwrap().clone();
                        if !wire_names.contains_key(&g.lhs)
                            || !bits_affected.is_subset(wire_names.get(&g.lhs).unwrap())
                        {
                            // println!("lhs adding fetch {:?} to {} entry",
                            //     wire_names.get(&g.res), g.lhs);
                            wire_names.entry(g.lhs.clone())
                                .and_modify(|v| {
                                    // println!("c");
                                    v.extend(bits_affected.clone())
                                })
                                .or_insert({
                                    // println!("d");
                                    HashSet::from_iter(bits_affected)
                                });
                            added.inc();
                        }
                    }
                }
                if !g.rhs.starts_with("x")
                    && !g.rhs.starts_with("y")
                    // && !wire_names.contains_key(&g.rhs)
                {

                    // if !wire_names.contains_key(&g.rhs) {


                    // }
                    let affects_z = g.res.starts_with('z');
                    if affects_z {
                        if !wire_names.contains_key(&g.rhs)
                            || !wire_names.get(&g.rhs).unwrap().contains(&g.res)
                        {
                            // println!("rhs adding res {}  to {} entry",
                            //     g.res, g.rhs);
                            wire_names.entry(g.rhs.clone())
                                .and_modify(|v: &mut HashSet<String>| {
                                    // println!("e");
                                    v.insert(g.res.clone());
                                })
                                .or_insert({
                                    // println!("f");
                                    HashSet::from_iter(vec![g.res.clone()])
                                });
                            added.inc();
                        }
                    } else {
                        let bits_affected = wire_names.get(&g.res).unwrap().clone();
                        if !wire_names.contains_key(&g.rhs)
                            || !bits_affected.is_subset(wire_names.get(&g.rhs).unwrap())
                        {
                            // println!("rhs adding fetch {:?} to {} entry",
                            //     wire_names.get(&g.res), g.rhs);
                            wire_names.entry(g.rhs.clone())
                                .and_modify(|v| {
                                    // println!("g");
                                    v.extend(bits_affected.clone())
                                })
                                .or_insert({
                                    // println!("e");
                                    HashSet::from_iter(bits_affected)
                                });
                            added.inc();
                        }
                    }
                }
            }
        }
    }
    wire_names
}

fn get_set_z_names(bits: u64) -> Vec<String> {
    let mut names = vec![];
    for i in 0..63 {
        if (1 << i) & bits != 0 {
            let mut name = String::from_str("z").unwrap();
            name.push_str(&i.to_string());
            names.push(name);
        }
    }
    names
}

fn get_wire_num(wires: &HashMap<String, u8>, prefix: char) -> u64 {
    let mut wires: Vec<_> = wires.iter()
        .filter(|(s, _) | s.starts_with(prefix))
        .collect();
    println!("{} has {} bits", prefix, wires.len());
    wires.sort();
    wires.reverse();
    let mut out: u64 = 0;
    for wire in wires {
        out = out << 1;
        out += *wire.1 as u64;
    }
    out
}

fn execute_gates(wires: &mut HashMap<String, u8>, gates: &Vec<Gate>) {
    let mut gates2 = gates.clone();
    while !gates2.is_empty() {
        for g_idx in (0..gates2.len()).rev() {
            let g = gates2.get_mut(g_idx).unwrap();
            if wires.contains_key(&g.lhs) && wires.contains_key(&g.rhs) {
                let lhs = *wires.get(&g.lhs).unwrap();
                let rhs = *wires.get(&g.rhs).unwrap();
                let out_val = g.op.exec(lhs, rhs);
                wires.insert(g.res.clone(), out_val);
                gates2.remove(g_idx);
            }
        }
    }
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
        assert_eq!(result, Some("1".to_string()));
    }
}
