use std::fmt::Display;

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
enum IntermediateValue {
    Val(u64),
    Mod8(u64),
    Unset,
    ModuloUnset,
    XORUnset(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    ip: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct StatePart2 {
    reg_a: IntermediateValue,
    reg_b: IntermediateValue,
    reg_c: IntermediateValue,
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

impl StatePart2 {
    pub fn get_combo_operand_value(&self, operand: u8) -> IntermediateValue {
        match operand {
            0 => IntermediateValue::Val(0),
            1 => IntermediateValue::Val(1),
            2 => IntermediateValue::Val(2),
            3 => IntermediateValue::Val(3),
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => unreachable!("invalid operand: {}", operand)
        }
    }
}

impl Display for IntermediateValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Val(x) => f.write_fmt(format_args!("Val({})", x)),
            Self::Mod8(x) => f.write_fmt(format_args!("Mod({})", x)),
            Self::Unset => f.write_str("Unset"),
            Self::XORUnset(op) => f.write_fmt(format_args!("XOR Unset({})", op)),
            Self::ModuloUnset => f.write_str("Modulo Unset"),
        }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut state, program) = parse_input(input);
    let mut output = vec![];
    println!("start state: {:?}", state);
    while state.ip < program.len() {
        let instr = program.get(state.ip).unwrap();
        instr.execute(&mut state, &mut output);
        println!("{:?}", state);
    }
    Some(output.join(","))
    // None
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, program) = parse_input(input);
    let mut target_output = get_program_nums(input);
    let mut state = StatePart2 {
        reg_a: IntermediateValue::Val(0),
        reg_b: IntermediateValue::Unset,
        reg_c: IntermediateValue::Unset,
        ip: program.len() - 2
    };
    println!("target: {:?}", target_output);
    println!("state: {:?}", state);
    loop {
        if state.ip == usize::MAX {
            state.ip = program.len() - 1;
        }
        match program.get(state.ip).unwrap() {
            Instruction::DivisionStoreRegA(x) => {
                let exp = state.get_combo_operand_value(*x);
                match exp {
                    IntermediateValue::Val(x) => {
                        let mult = 1 << x;
                        println!("undoing div: reg_a = reg_a ({}) * {}", state.reg_a, mult);
                        match state.reg_a {
                            IntermediateValue::Val(x) => {
                                state.reg_a = IntermediateValue::Val(x * mult);
                                if x * mult == 0 {
                                    println!("setting to {}", 1);
                                    state.reg_a = IntermediateValue::Val(1);
                                }
                            },
                            IntermediateValue::Mod8(add) => {
                                state.reg_a = IntermediateValue::Mod8(add * mult);
                            },
                            _ => unreachable!("reg_a is unset")
                        }
                    },
                    _ => panic!("computing reg_a = reg_a ({}) / 2 ^ {}", state.reg_a, exp)
                }
            },
            Instruction::DivisionStoreRegB(x) => {
                let exp = state.get_combo_operand_value(*x);
                match exp {
                    IntermediateValue::Val(x) => {
                        let mult = 1 << x;
                        println!("undoing div: reg_b = reg_a ({}) * {}", state.reg_b, mult);
                        match state.reg_a {
                            IntermediateValue::Val(x) => {
                                state.reg_b = IntermediateValue::Val(x * mult);
                                if x * mult == 0 {
                                    println!("setting to {}", 1);
                                    state.reg_b = IntermediateValue::Val(1);
                                }
                            },
                            IntermediateValue::Mod8(add) => {
                                state.reg_b = IntermediateValue::Mod8(add * mult);
                            },
                            _ => unreachable!("reg_a is unset")
                        }
                    },
                    _ => panic!("computing reg_b = reg_a ({}) / 2 ^ {}", state.reg_a, exp)
                }
            },
            Instruction::DivisionStoreRegC(x) => {
                let exp = state.get_combo_operand_value(*x);
                match exp {
                    IntermediateValue::Val(x) => {
                        let mult = 1 << x;
                        println!("undoing div: reg_c = reg_a ({}) * {}", state.reg_a, mult);
                        match state.reg_a {
                            IntermediateValue::Val(x) => {
                                state.reg_c = IntermediateValue::Val(x * mult);
                                if x * mult == 0 {
                                    println!("setting to {}", 1);
                                    state.reg_c = IntermediateValue::Val(1);
                                }
                            },
                            IntermediateValue::Mod8(add) => {
                                state.reg_c = IntermediateValue::Mod8(add * mult);
                            },
                            _ => unreachable!("reg_a is unset")
                        }
                    },
                    _ => panic!("computing reg_c = reg_a ({}) / 2 ^ {} from {}", state.reg_a, exp, x)
                }
            },
            Instruction::RegBLiteralXOR(x) => {
                // println!("undoing XOR: reg_b = reg_b ({}) ^ {}", state.reg_b, x);
                // state.reg_b = state.reg_b ^ *x as u64;
                match state.reg_b {
                    IntermediateValue::Val(curr_val) => {
                        println!("undoing XOR: reg_b = reg_b ({}) ^ {}", state.reg_b, x);
                        state.reg_b = IntermediateValue::Val(curr_val ^ *x as u64);
                    },
                    IntermediateValue::Mod8(_) => {
                        panic!("computing reg_b = reg_b ({}) ^ {}", state.reg_b, x);
                    }
                    _ => panic!("Reg_b literal XOR unset")
                }
            },
            Instruction::RegBRegCXOR => {
                match (state.reg_b, state.reg_c) {
                    (IntermediateValue::Val(x), IntermediateValue::Val(y)) => {
                        println!("undoing XOR: reg_b = reg_b ({}) ^ {}", state.reg_b, state.reg_c);
                        state.reg_b = IntermediateValue::Val(x ^ y);
                    }
                    (IntermediateValue::Unset, _) => {
                        println!("Unset XOR: reg_b = reg_b ({}) ^ {}", state.reg_b, state.reg_c);
                        state.reg_b = IntermediateValue::XORUnset(5);
                    },
                    (_, IntermediateValue::Unset) => {
                        println!("Unset XOR: reg_b = reg_b ({}) ^ {}", state.reg_b, state.reg_c);
                        state.reg_b = IntermediateValue::XORUnset(6);
                    },
                    _ => panic!("computing reg_b = reg_b ({}) ^ reg_c ({})", state.reg_b, state.reg_c)
                }
            },
            Instruction::Modulo(x) => {
                match state.get_combo_operand_value(*x) {
                    IntermediateValue::Val(y) => {
                        println!("undoing modulo: reg_B = {} mod 8 (operand {})",
                            y, x
                        );
                        state.reg_b = IntermediateValue::Mod8(y);
                    },
                    IntermediateValue::Mod8(_) => (),
                    IntermediateValue::Unset => {
                        println!("modulo: setting to Unset Modulo");
                        state.reg_b = IntermediateValue::ModuloUnset;
                    }
                    _ => panic!("modulo operand {} is unset", x)
                }
            },
            Instruction::JumpNonZero(_) => {
                if let IntermediateValue::Val(x) = state.reg_a {
                    if x == 0 {
                        panic!("wouldn't jump but needs to! {:?}", state);
                    }
                }
            },
            Instruction::Output(x) => {
                if target_output.len() == 0 {
                    break;
                }
                let target = target_output.pop().unwrap() as i8;
                let actual = match state.get_combo_operand_value(*x) {
                    IntermediateValue::Val(x) => 0b111 & x as i8,
                    IntermediateValue::Mod8(add) => add as i8,
                    _ => 0
                };
                let curr_val = match state.get_combo_operand_value(*x) {
                    IntermediateValue::Val(x) => x,
                    IntermediateValue::Mod8(_) => {
                        panic!("computing curr_val of reg {} ({}), target {}",
                            x, state.get_combo_operand_value(*x), target
                        );
                    },
                    _ => 0,
                };
                if target != actual {
                    let diff = (target - actual).rem_euclid(8) as i64;
                    let new_val = (curr_val as i64 + diff) as u64;
                    if *x == 4 {
                        println!("output: reg_a = reg_a ({}) + diff {}", state.reg_a, diff);
                        if new_val == 0 {
                            state.reg_a = IntermediateValue::Val(8);
                        } else {
                            state.reg_a = IntermediateValue::Val(new_val);
                        }
                    } else if *x == 5 {
                        println!("output: reg_b = reg_b ({}) + diff {}", state.reg_b, diff);
                        state.reg_b = IntermediateValue::Val(new_val);
                    } else {
                        println!("output: reg_c = reg_c ({}) + diff {}", state.reg_c, diff);
                        state.reg_c = IntermediateValue::Val(new_val);
                    }
                } else {
                    println!("output: target and actual match");
                    match state.get_combo_operand_value(*x) {
                        IntermediateValue::ModuloUnset => {
                            if *x == 5 {
                                println!("(mod unset) setting reg_b to {}", target);
                                state.reg_b = IntermediateValue::Val(target as u64);
                            } else if *x == 6 {
                                println!("(mod unset) setting reg_c to {}", target);
                                state.reg_c = IntermediateValue::Val(target as u64);
                            }
                        },
                        IntermediateValue::XORUnset(op) => {
                            if op == 5 {
                                let lhs = target as u64;
                                if let IntermediateValue::Val(rhs) = state.get_combo_operand_value(6) {
                                    println!("(xor unset) setting reg_b to {}", lhs ^ rhs);
                                    state.reg_b = IntermediateValue::Val(lhs ^ rhs);
                                }
                            } else {
                                let rhs = target as u64;
                                if let IntermediateValue::Val(lhs) = state.get_combo_operand_value(op) {
                                    println!("(xor unset) setting reg_b to {}", lhs ^ rhs);
                                    state.reg_b = IntermediateValue::Val(lhs ^ rhs);
                                }
                            }
                        },
                        IntermediateValue::Unset => {
                            println!("(unset) setting reg_b to {}", target);
                            state.reg_b = IntermediateValue::Val(target as u64);
                            // if target == 0 {
                            //     state.reg_b = IntermediateValue::Val(8 as u64);
                            // }
                        },
                        _ => ()
                    }
                }
            },
        }
        state.ip = state.ip.wrapping_sub(1);
    }

    match state.reg_a {
        IntermediateValue::Val(x) => Some(x),
        IntermediateValue::Mod8(add) => Some(add),
        _ => panic!("reg_a unset")
    }
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
    fn test_part_one_for_two() {
        let input = advent_of_code_2024::template::read_file_part("examples", DAY, 2);
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
