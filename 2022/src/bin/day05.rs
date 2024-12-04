advent_of_code_2022::solution!(5);

pub enum RunType {
    Test,
    Final
}

struct ProcedureStep {
    num_boxes: i32,
    start_stack: usize,
    end_stack: usize
}

pub fn part_one(input: &str, input_type: RunType) {
    let mut result = String::new();
    let mut stacks = get_stacks(input_type);
    let steps = get_steps(&input);
    for step in steps {
        for _ in 0..step.num_boxes {
            let tmp = stacks[step.start_stack].pop().unwrap();
            stacks[step.end_stack].push(tmp);
        }
    }
    for stack in stacks {
        result.push(*stack.get(stack.len() - 1).unwrap());
    }
    println!("{}", result);
}

pub fn part_two(input: &str, input_type: RunType) {
    let mut result = String::new();
    let mut stacks = get_stacks(input_type);
    let steps = get_steps(&input);
    for step in steps {
        let mut temp_stack: Vec<char> = Vec::new();
        for _ in 0..step.num_boxes {
            temp_stack.push(stacks[step.start_stack].pop().unwrap());
        }
        for _ in 0..step.num_boxes {
            stacks[step.end_stack].push(temp_stack.pop().unwrap());
        }
    }
    for stack in stacks {
        result.push(*stack.get(stack.len() - 1).unwrap());
    }
    println!("{}", result);
}

fn get_steps(input: &str) -> Vec<ProcedureStep> {
    let mut steps: Vec<ProcedureStep> = Vec::new();
    for line in input.lines() {
        let line = line.split_whitespace();
        let mut line_numbers =  line.filter_map(|s| s.parse::<i32>().ok());
        let tmp = ProcedureStep {
            num_boxes: line_numbers.next().unwrap(),
            start_stack: line_numbers.next().unwrap() as usize - 1,
            end_stack: line_numbers.next().unwrap() as usize - 1
        };
        steps.push(tmp);
    }
    steps
}

fn get_stacks(input_type: RunType) -> Vec<Vec<char>> {
    match input_type {
        RunType::Test => vec![
            vec!['Z', 'N'],
            vec!['M', 'C', 'D'],
            vec!['P']
        ],
        RunType::Final => vec![
            vec!['Q', 'S', 'W', 'C', 'Z', 'V', 'F', 'T'],
            vec!['Q', 'R', 'B'],
            vec!['B', 'Z', 'T', 'Q', 'P', 'M', 'S'],
            vec!['D', 'V', 'F', 'R', 'Q', 'H'],
            vec!['J', 'G', 'L', 'D', 'B', 'S', 'T', 'P'],
            vec!['W', 'R', 'T', 'Z'],
            vec!['H', 'Q', 'M', 'N', 'S', 'F', 'R', 'J'],
            vec!['R', 'N', 'F', 'H' ,'W'],
            vec!['J', 'Z', 'T', 'Q', 'P', 'R', 'B']
        ]
    }
}

fn main() {
    let input = advent_of_code_2022::template::read_file("inputs", DAY);
    part_one(&input, RunType::Final);
    part_two(&input, RunType::Final);
}
