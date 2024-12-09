advent_of_code_2022::solution!(11);

struct Monkey {
    items: Vec<u64>,
    update_worry: fn(u64) -> u64,
    test_num: u64,
    throw_true: usize,
    throw_false: usize,
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut monkeys = get_monkeys(input);
    let mut num_inspections: Vec<u64> = vec![0; monkeys.len()];
    let update_relief = |x: u64| x / 3;
    for _ in 0..20 {
        for monk_idx in 0..monkeys.len() {
            let monk = monkeys.get_mut(monk_idx).unwrap();
            let throws = monkey_turn(monk, update_relief);
            *num_inspections.get_mut(monk_idx).unwrap() += throws.len() as u64;
            for (item, to_monk) in throws {
                monkeys.get_mut(to_monk).unwrap().items.push(item);
            }
        }
    }
    num_inspections.sort();
    num_inspections.reverse();
    let result = num_inspections.first().unwrap() * num_inspections.get(1).unwrap();
    Some(result)
}

fn monkey_turn(m: &mut Monkey, update_relief: fn(u64) -> u64) -> Vec<(u64, usize)> {
    let mut result: Vec<(u64, usize)> = Vec::new();
    while !m.items.is_empty() {
        let mut item = m.items.pop().unwrap();
        item = (m.update_worry)(item);
        item = (update_relief)(item);

        if item % m.test_num == 0 {
            result.push((item, m.throw_true));
        } else {
            result.push((item, m.throw_false));
        }
    }
    result
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut monkeys = get_monkeys(input);
    let mut num_inspections: Vec<u64> = vec![0; monkeys.len()];
    let update_relief = |x: u64| x;
    let max_wrap: u64 = monkeys.iter().map(|m| m.test_num).product();
    for _ in 0..10_000 {
        for monk_idx in 0..monkeys.len() {
            let throws = monkey_turn(monkeys.get_mut(monk_idx).unwrap(), update_relief);
            *num_inspections.get_mut(monk_idx).unwrap() += throws.len() as u64;
            for (item, to_monk) in throws {
                if item > max_wrap {
                    monkeys.get_mut(to_monk).unwrap().items.push(item % max_wrap);
                } else {
                    monkeys.get_mut(to_monk).unwrap().items.push(item);
                }
            }
        }
    }
    num_inspections.sort();
    num_inspections.reverse();
    let result = num_inspections.first().unwrap() * num_inspections.get(1).unwrap();
    Some(result)
}

fn get_monkeys(input: &str) -> Vec<Monkey> {
    if input.len() > 750 {
        vec![
            Monkey{
                items: vec![66, 79],
                update_worry: |x| x * 11,
                test_num: 7,
                throw_true: 6,
                throw_false: 7,
            },
            Monkey{
                items: vec![84, 94, 94, 81, 98, 75],
                update_worry: |x| x * 17,
                test_num: 13,
                throw_true: 5,
                throw_false: 2,
            },
            Monkey{
                items: vec![85, 79, 59, 64, 79, 95, 67],
                update_worry: |x| x + 8,
                test_num: 5,
                throw_true: 4,
                throw_false: 5,
            },
            Monkey{
                items: vec![70],
                update_worry: |x| x + 3,
                test_num: 19,
                throw_true: 6,
                throw_false: 0,
            },
            Monkey{
                items: vec![57, 69, 78, 78],
                update_worry: |x| x + 4,
                test_num: 2,
                throw_true: 0,
                throw_false: 3,
            },
            Monkey{
                items: vec![65, 92, 60, 74, 72],
                update_worry: |x| x + 7,
                test_num: 11,
                throw_true: 3,
                throw_false: 4,
            },
            Monkey{
                items: vec![77, 91, 91],
                update_worry: |x| x * x,
                test_num: 17,
                throw_true: 1,
                throw_false: 7,
            },Monkey{
                items: vec![76, 58, 57, 55, 67, 77, 54, 99],
                update_worry: |x| x + 6,
                test_num: 3,
                throw_true: 2,
                throw_false: 1,
            },
        ]
    } else {
        vec![
            Monkey{
                items: vec![79, 98],
                update_worry: |x| x * 19,
                test_num: 23,
                throw_true: 2,
                throw_false: 3,
            },
            Monkey{
                items: vec![54, 65, 75, 74],
                update_worry: |x| x + 6,
                test_num: 19,
                throw_true: 2,
                throw_false: 0,
            },
            Monkey{
                items: vec![79, 60, 97],
                update_worry: |x| x * x,
                test_num: 13,
                throw_true: 1,
                throw_false: 3,
            },
            Monkey{
                items: vec![74],
                update_worry: |x| x + 3,
                test_num: 17,
                throw_true: 0,
                throw_false: 1,
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(2713310158));
    }
}
