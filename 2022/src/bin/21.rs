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
        let mut remove = Vec::new();
        for m in monkeys.keys() {
            let op = monkeys.get(m).unwrap();
            if vals.contains_key(m) {
                continue;
            }
            if check_do_operation(&mut vals, op, m.to_string()) {
                remove.push(m);
            }
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
            let op = monkeys.get(m).unwrap();
            if vals.contains_key(m) {
                continue;
            }
            made_op = made_op || check_do_operation(&mut vals, op, m.to_string());
        }
        if !made_op {
            break;
        }
    }

    let root = monkeys.get("root").unwrap();
    let mut current;
    let mut target = match root {
        Operation::Add(m1, m2) => {
            if vals.contains_key(m1) {
                current = monkeys.get(m2).unwrap();
                *vals.get(m1).unwrap()
            } else {
                current = monkeys.get(m1).unwrap();
                *vals.get(m2).unwrap()
            }
        },
        _ => {
            unreachable!("root not add!");
        }
    };

    loop {
        match current {
            Operation::Add(m1, m2) => {
                if m2 == "humn" {
                    let val1 = vals.get(m1).unwrap();
                    return Some(target - val1);
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
                    let val2 = vals.get(m2).unwrap();
                    return Some(*val2 + target);
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
            _ => {
                unreachable!("found entry that's constant!");
            }
        }
        // println!("{}", target);
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
            true
        },
        Operation::Add(m1, m2) => {
            let val1 = vals.get(m1);
            let val2 = vals.get(m2);
            if val1.is_some() && val2.is_some() {
                // println!("{}: adding {} from {} and {} from {} to get {}",
                //     monkey, val1.unwrap(), m1, val2.unwrap(), m2, val1.unwrap() + val2.unwrap());
                vals.insert(monkey, val1.unwrap() + val2.unwrap());
                true
            } else {
                false
            }
        },
        Operation::Div(m1, m2) => {
            let val1 = vals.get(m1);
            let val2 = vals.get(m2);
            if val1.is_some() && val2.is_some() {
                // println!("{}: dividing {} from {} and {} from {} to get {}",
                //     monkey, val1.unwrap(), m1, val2.unwrap(), m2, val1.unwrap() / val2.unwrap());
                vals.insert(monkey, val1.unwrap() / val2.unwrap());
                true
            } else {
                false
            }
        },
        Operation::Mul(m1, m2) => {
            let val1 = vals.get(m1);
            let val2 = vals.get(m2);
            if val1.is_some() && val2.is_some() {
                // println!("{}: multiplying {} from {} and {} from {} to get {}",
                //     monkey, val1.unwrap(), m1, val2.unwrap(), m2, val1.unwrap() * val2.unwrap());
                vals.insert(monkey, val1.unwrap() * val2.unwrap());
                true
            } else {
                false
            }
        },
        Operation::Sub(m1, m2) => {
            let val1 = vals.get(m1);
            let val2 = vals.get(m2);
            if val1.is_some() && val2.is_some() {
                // println!("{}: subtracting {} from {} and {} from {} to get {}",
                //     monkey, val1.unwrap(), m1, val2.unwrap(), m2, val1.unwrap() - val2.unwrap());
                vals.insert(monkey, val1.unwrap() - val2.unwrap());
                true
            } else {
                false
            }
        },
    }
}

fn get_monkeys(input: &str) -> HashMap<String, Operation> {
    let mut monkeys = HashMap::new();
    for line in input.lines() {
        let mut line = line.split(" ");
        let mut name = line.next().unwrap().chars();
        name.next_back();
        let name = name.as_str().to_string();
        let op1 = line.next().unwrap();
        if op1.parse::<i64>().is_ok() {
            monkeys.insert(name, Operation::Value(op1.parse::<i64>().unwrap()));
        } else {
            let op2 = line.next().unwrap().chars().next().unwrap();
            let op3 = line.next().unwrap();
            match op2 {
                '+' => {
                    monkeys.insert(name, Operation::Add(op1.to_string(), op3.to_string()))
                },
                '-' => {
                    monkeys.insert(name, Operation::Sub(op1.to_string(), op3.to_string()))
                },
                '*' => {
                    monkeys.insert(name, Operation::Mul(op1.to_string(), op3.to_string()))
                },
                '/' => {
                    monkeys.insert(name, Operation::Div(op1.to_string(), op3.to_string()))
                },
                _ => {
                    panic!("symbol is {}, not math symbol", op2)
                },
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
