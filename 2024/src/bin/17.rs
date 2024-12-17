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
    let mut output = vec![];
    while state.ip < program.len() {
        let instr = program.get(state.ip).unwrap();
        instr.execute(&mut state, &mut output);
    }
    Some(output.join(","))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (state, program) = parse_input(input);
    let target_output = get_program_nums(input);
    None
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
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, None);
    }
}
