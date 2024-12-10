advent_of_code_2022::solution!(21);

use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Operation {
    Value(i64),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

pub fn part_one(input: &str) -> Option<i64> {
    let monkeys = get_monkeys(input);
    let mut vals: HashMap<String, i64> = HashMap::new();
    while !vals.contains_key("root") {
        for m in monkeys.keys() {
            if vals.contains_key(m) {
                continue;
            }
            let operation = monkeys.get(m).unwrap();
            let _ = check_do_operation(&mut vals, operation, m.to_string());
        }
    }
    Some(*vals.get("root").unwrap())
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut monkeys = get_monkeys(input);
    let mut vals: HashMap<String, i64> = HashMap::new();
    monkeys.remove("humn");

    while !vals.contains_key("root") {
        let mut made_op = false;
        for m in monkeys.keys() {
            let operation = monkeys.get(m).unwrap();
            if vals.contains_key(m) {
                continue;
            }
            made_op = made_op || check_do_operation(&mut vals, operation, m.to_string());
        }
        if !made_op {
            break;
        }
    }

    let (current, target) = match monkeys.get("root").unwrap() {
        Operation::Add(m1, m2) => {
            if vals.contains_key(m1) {
                (monkeys.get(m2).unwrap(), *vals.get(m1).unwrap())
            } else {
                (monkeys.get(m1).unwrap(), *vals.get(m2).unwrap())
            }
        },
        _ => unreachable!("root not add!")
    };

    Some(reverse_find_humn_value(&vals, &monkeys, current, target))
}

fn reverse_find_humn_value(
    vals: &HashMap<String, i64>,
    monkeys: &HashMap<String, Operation>,
    current: &Operation,
    target:i64
) -> i64 {
    let mut current = &current.clone();
    let mut target = target;
    loop {
        match current {
            Operation::Add(m1, m2) => {
                if m2 == "humn" {
                    return target - vals.get(m1).unwrap();
                } else if vals.get(m1).is_some() {
                    target -= vals.get(m1).unwrap();
                    current = monkeys.get(m2).unwrap();
                } else if vals.get(m2).is_some() {
                    target -= vals.get(m2).unwrap();
                    current = monkeys.get(m1).unwrap();
                }
            },
            Operation::Sub(m1, m2) => {
                if m1 == "humn" {
                    return *vals.get(m2).unwrap() + target;
                } else if vals.get(m1).is_some() {
                    target = vals.get(m1).unwrap() - target;
                    current = monkeys.get(m2).unwrap();
                } else if vals.get(m2).is_some() {
                    target += vals.get(m2).unwrap();
                    current = monkeys.get(m1).unwrap();
                }
            },
            Operation::Mul(m1, m2) => {
                if vals.get(m1).is_some() {
                    target /= vals.get(m1).unwrap();
                    current = monkeys.get(m2).unwrap();
                } else if vals.get(m2).is_some() {
                    target /= vals.get(m2).unwrap();
                    current = monkeys.get(m1).unwrap();
                }
            },
            Operation::Div(m1, m2) => {
                if vals.get(m1).is_some() {
                    target = vals.get(m1).unwrap() / target;
                    current = monkeys.get(m2).unwrap();
                } else if vals.get(m2).is_some() {
                    target *= vals.get(m2).unwrap();
                    current = monkeys.get(m1).unwrap();
                }
            },
            _ => unreachable!("found entry that's constant!")
        }
    }
}

fn check_do_operation(
    vals: &mut HashMap<String, i64>,
    operation: &Operation,
    monkey: String
) -> bool {
    match operation {
        Operation::Value(v) => {
            vals.insert(monkey, *v);
            return true;
        },
        Operation::Add(m1, m2) => {
            if let (Some(val1), Some(val2)) = (vals.get(m1), vals.get(m2)) {
                vals.insert(monkey, val1 + val2);
                return true;
            }
        },
        Operation::Div(m1, m2) => {
            if let (Some(val1), Some(val2)) = (vals.get(m1), vals.get(m2)) {
                vals.insert(monkey, val1 / val2);
                return true;
            }
        },
        Operation::Mul(m1, m2) => {
            if let (Some(val1), Some(val2)) = (vals.get(m1), vals.get(m2)) {
                vals.insert(monkey, val1 * val2);
                return true;
            }
        },
        Operation::Sub(m1, m2) => {
            if let (Some(val1), Some(val2)) = (vals.get(m1), vals.get(m2)) {
                vals.insert(monkey, val1 - val2);
                return true;
            }
        },
    }
    false
}

fn get_monkeys(input: &str) -> HashMap<String, Operation> {
    let mut monkeys = HashMap::new();
    for line in input.lines() {
        let mut line = line.split(" ");
        let name = line.next().unwrap();
        let name = name[..name.len() - 1].to_string();
        let op1 = line.next().unwrap();
        if op1.parse::<i64>().is_ok() {
            monkeys.insert(name, Operation::Value(op1.parse::<i64>().unwrap()));
        } else {
            let op1 = op1.to_string();
            let op2 = line.next().unwrap().chars().next().unwrap();
            let op3 = line.next().unwrap().to_string();
            match op2 {
                '+' => {
                    monkeys.insert(name, Operation::Add(op1, op3))
                },
                '-' => {
                    monkeys.insert(name, Operation::Sub(op1, op3))
                },
                '*' => {
                    monkeys.insert(name, Operation::Mul(op1, op3))
                },
                '/' => {
                    monkeys.insert(name, Operation::Div(op1, op3))
                },
                _ => unreachable!(),
            };
        }
    }
    monkeys
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(301));
    }
}
