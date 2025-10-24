// Solving https://adventofcode.com/2024/day/17
use std::{iter::zip, u64};

advent_of_code_2024::solution!(17);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Instruction {
    DivisionStoreRegA(u8),
    DivisionStoreRegB(u8),
    DivisionStoreRegC(u8),
    RegBLiteralXOR(u8),
    RegBRegCXOR,
    Modulo(u8),
    JumpNonZero(u8),
    Output(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    ip: usize,
}

impl Instruction {
    pub fn from_numbers(opcode: u8, operand: u8) -> Self {
        match opcode {
            0 => Self::DivisionStoreRegA(operand),
            1 => Self::RegBLiteralXOR(operand),
            2 => Self::Modulo(operand),
            3 => Self::JumpNonZero(operand),
            4 => Self::RegBRegCXOR,
            5 => Self::Output(operand),
            6 => Self::DivisionStoreRegB(operand),
            7 => Self::DivisionStoreRegC(operand),
            _ => unreachable!("Not a valid opcode: {}", opcode)
        }
    }

    pub fn execute(&self, state: &mut State, out: &mut Vec<String>) {
        match self {
            Instruction::DivisionStoreRegA(x) => {
                let denominator = 1 << state.get_combo_operand_value(*x);
                state.reg_a = state.reg_a / denominator;
                state.ip += 1;
            },
            Instruction::DivisionStoreRegB(x) => {
                let denominator = 1 << state.get_combo_operand_value(*x);
                state.reg_b = state.reg_a / denominator;
                state.ip += 1;
            },
            Instruction::DivisionStoreRegC(x) => {
                let denominator = 1 << state.get_combo_operand_value(*x);
                state.reg_c = state.reg_a / denominator;
                state.ip += 1;
            },
            Instruction::RegBLiteralXOR(x) => {
                state.reg_b = state.reg_b ^ *x as u64;
                state.ip += 1;
            },
            Instruction::RegBRegCXOR => {
                state.reg_b = state.reg_b ^ state.reg_c;
                state.ip += 1;
            },
            Instruction::Modulo(x) => {
                state.reg_b = 0b111 & state.get_combo_operand_value(*x);
                state.ip += 1;
            },
            Instruction::JumpNonZero(x) => {
                if state.reg_a != 0 {
                    state.ip = *x as usize / 2;
                } else {
                    state.ip += 1;
                }
            },
            Instruction::Output(x) => {
                let value = 0b111 & state.get_combo_operand_value(*x);
                out.push(value.to_string());
                state.ip += 1;
            },
        }
    }
}

impl State {
    pub fn get_combo_operand_value(&self, operand: u8) -> u64 {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => unreachable!("invalid operand: {}", operand)
        }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut state, program) = parse_input(input);
    let output = run_program(&mut state, &program);
    Some(output.join(","))
    // None
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, program) = parse_input(input);
    let target_output = get_program_nums(input)
        .iter()
        .map(|x| x.to_string())
        .collect();
    Some(dfs_reg_a_possibilities(&program, &target_output))
}

fn parse_input(input: &str) -> (State, Vec<Instruction>) {
    let mut lines = input.lines();
    let reg_a: u64 = lines.next().unwrap().split(" ").nth(2).unwrap().parse().unwrap();
    let reg_b: u64 = lines.next().unwrap().split(" ").nth(2).unwrap().parse().unwrap();
    let reg_c: u64 = lines.next().unwrap().split(" ").nth(2).unwrap().parse().unwrap();
    let program_nums = get_program_nums(input);
    let mut program = vec![];
    for idx in (0..program_nums.len()).step_by(2) {
        let opcode = *program_nums.get(idx).unwrap();
        let operand = *program_nums.get(idx + 1).unwrap();
        program.push(Instruction::from_numbers(opcode, operand));
    }
    let state = State { reg_a, reg_b, reg_c, ip: 0 };
    (state, program)
}

fn run_program(state: &mut State, program: &Vec<Instruction>) -> Vec<String> {
    let mut output = vec![];
    while state.ip < program.len() {
        let instr = program.get(state.ip).unwrap();
        instr.execute(state, &mut output);
    }
    output
}


fn get_program_nums(input: &str) -> Vec<u8> {
    input.lines()
        .next_back()
        .unwrap()
        .split(" ")
        .nth(1)
        .unwrap()
        .split(",")
        .map(|x| x.parse::<u8>().unwrap())
        .collect()
}

fn dfs_reg_a_possibilities(program: &Vec<Instruction>, target_output: &Vec<String>) -> u64 {
    let mut stack = vec![];
    let mut lowest_reg_a = u64::MAX;
    // (starting reg_a value, bitlength of value)
    stack.push((0u64, 0u64));
    while let Some((reg_a_value, bitlen)) = stack.pop() {
        let mut start_state = get_start_state(reg_a_value);
        let output = run_program(&mut start_state, program);
        if output.len() == target_output.len() && outputs_match(&output, target_output) {
            lowest_reg_a = std::cmp::min(lowest_reg_a, reg_a_value);
        }

        for new_bits in (0..8).rev() {
            let new_reg_a = reg_a_value << 3 | new_bits;
            // prevent 0 from just getting re-added every time
            if bitlen > 0 && new_reg_a == 0 {
                continue;
            }
            let mut start_state = get_start_state(new_reg_a);
            let output = run_program(&mut start_state, program);
            if output.len() > target_output.len() {
                continue;
            }

            let target_start_idx = target_output.len() - output.len();
            let target_substr = &target_output[target_start_idx..];
            if !outputs_match(&output, target_substr) {
                continue;
            }
            stack.push((new_reg_a, bitlen + 3));
        }
    }
    lowest_reg_a
}

fn get_start_state(reg_a: u64) -> State {
    State { reg_a, reg_b: 0, reg_c: 0, ip: 0 }
}

fn outputs_match(output: &[String], target_output: &[String]) -> bool {
    for (out, target) in zip(output, target_output) {
        if out != target {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2024::template::read_file_part("examples", DAY, 2);
        let result = part_two(&input);
        assert_eq!(result, Some(117440));
    }
}
