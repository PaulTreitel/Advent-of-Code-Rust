advent_of_code_2022::solution!(10);

const SPECIAL_CYCLES: [usize; 6] = [20, 60, 100, 140, 180, 220];
const CRT_WIDTH: i32 = 40;
// const CRT_HEIGHT: i32 = 6;
const NOOP: &str = "noop";
const ADDX: &str = "addx";

struct Instruction {
    name: String,
    value: Option<i32>,
}

struct Communicator {
    cycle: i32,
    x_reg: i32,
    crt: Vec<Vec<char>>,
}

pub fn part_one(input: &str) -> Option<i32> {
    let instructions = get_instructions(input);
    let (signals, _) = get_signals(instructions);
    let mut total = 0;
    for cycle in SPECIAL_CYCLES {
        if cycle <= signals.len() {
            total += signals.get(cycle - 1).unwrap();
        }
    }
    Some(total)
}

fn get_signals(instructions: Vec<Instruction>) -> (Vec<i32>, Vec<Vec<char>>) {
    let mut signals: Vec<i32> = Vec::new();
    let crt: Vec<Vec<char>> = vec![vec!['.'; 40]; 6];
    let mut comms = Communicator {
        cycle: 0,
        x_reg: 1,
        crt,
    };
    for instr in instructions {
        if instr.name == NOOP {
            signals.push(cycle(&mut comms));
        } else if instr.name == ADDX {
            signals.push(cycle(&mut comms));
            signals.push(cycle(&mut comms));
            comms.x_reg += instr.value.unwrap();
        }
    }
    (signals, comms.crt)
}

fn cycle(comms: &mut Communicator) -> i32 {
    comms.cycle += 1;
    let crt_row = (comms.cycle - 1) / CRT_WIDTH;
    let crt_col = (comms.cycle - 1) % CRT_WIDTH;
    let reg_diff = comms.x_reg.abs_diff(crt_col);
    if reg_diff <= 1 {
        *comms
            .crt
            .get_mut(crt_row as usize)
            .unwrap()
            .get_mut(crt_col as usize)
            .unwrap() = '#';
    }
    comms.cycle * comms.x_reg
}

pub fn part_two(input: &str) -> Option<i32> {
    let instructions = get_instructions(input);
    let (_, crt) = get_signals(instructions);
    for line in crt {
        let x = line
            .iter()
            .fold("".to_string(), |acc, e| acc + &e.to_string());
        println!("{}", x);
    }
    None
}

fn get_instructions(input: &str) -> Vec<Instruction> {
    let mut instrs: Vec<Instruction> = Vec::new();
    for line in input.lines() {
        if line == NOOP {
            instrs.push(Instruction {
                name: NOOP.to_string(),
                value: None,
            });
        } else {
            let mut line = line.split_ascii_whitespace();
            let cmd = line.next().unwrap();
            if cmd == ADDX {
                let val: i32 = line.next().unwrap().parse().unwrap();
                instrs.push(Instruction {
                    name: ADDX.to_string(),
                    value: Some(val),
                });
            }
        }
    }
    instrs
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, None);
    }
}
