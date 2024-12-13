advent_of_code_2024::solution!(13);

const A_PRESS_COST: i64 = 3;
const B_PRESS_COST: i64 = 1;
const PART_TWO_ADD_FACTOR: i64 = 10_000_000_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct ArcadeMachine {
    a_button: (i64, i64),
    b_button: (i64, i64),
    prize: (i64, i64),
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut token_costs = 0;
    let machines = parse_input(input);
    for m in &machines {
        if let Some(tokens) = cost_to_win_prize(m) {
            token_costs += tokens;
        }
    }
    Some(token_costs)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut token_costs = 0;
    let mut machines = parse_input(input);
    part_two_machine_update(&mut machines);
    for m in &machines {
        if let Some(tokens) = cost_to_win_prize(m) {
            token_costs += tokens;
        }
    }
    Some(token_costs)
}

fn parse_input(input: &str) -> Vec<ArcadeMachine> {
    let lines: Vec<&str> = input.lines().collect();
    let mut machines = Vec::with_capacity(lines.len() / 4);
    for idx in (0..lines.len()).step_by(4) {
        let mut a_button = lines
            .get(idx)
            .unwrap()
            .split(|c| [' ', ','].contains(&c));
        let a_x: i64 = a_button.nth(2).unwrap()[2..].parse().unwrap();
        let a_y: i64 = a_button.nth(1).unwrap()[2..].parse().unwrap();
        let mut b_button = lines
            .get(idx + 1)
            .unwrap()
            .split(|c| [' ', ','].contains(&c));
        let b_x: i64 = b_button.nth(2).unwrap()[2..].parse().unwrap();
        let b_y: i64 = b_button.nth(1).unwrap()[2..].parse().unwrap();
        let mut prize = lines
            .get(idx + 2)
            .unwrap()
            .split(|c| [' ', ','].contains(&c));
        let prize_x: i64 = prize.nth(1).unwrap()[2..].parse().unwrap();
        let prize_y: i64 = prize.nth(1).unwrap()[2..].parse().unwrap();
        let arcade_machine = ArcadeMachine {
            a_button: (a_x, a_y),
            b_button: (b_x, b_y),
            prize: (prize_x, prize_y)
        };
        machines.push(arcade_machine);
    }
    machines
}

fn cost_to_win_prize(m: &ArcadeMachine) -> Option<i64> {
    // Solving the system of equations:
    // prize.x = m * A.x + n * B.x
        // prize.y = m * A.y + n * B.y
    if m.a_button.0 != m.a_button.1 {
        // solve for `n`:
        // n = A.x * (prize.y - prize.x) / (B.y - B.x)
        let numerator = m.prize.1 * m.a_button.0 - m.prize.0 * m.a_button.1;
        let denominator = m.b_button.1 * m.a_button.0 - m.b_button.0 * m.a_button.1;
        if numerator % denominator == 0 {
            let b_presses = numerator / denominator;
            let x_dist_remaining = m.prize.0 - b_presses * m.b_button.0;
            let a_presses = x_dist_remaining / m.a_button.0;
            return Some(A_PRESS_COST * a_presses + B_PRESS_COST * b_presses);
        }
    } else {
        // If A.x == A.y then solving for `n` degenerates so we have to re-solve
        // for `m`
        // m = (prize.y * B.x - prize.x * B.y) / (A.y * B.x - A.x * B.y)
        let numerator = m.prize.1 * m.b_button.0 - m.prize.0 * m.b_button.1;
        let denominator = m.a_button.1 * m.b_button.0 - m.a_button.0 * m.b_button.1;
        if numerator % denominator == 0 {
            let a_presses = numerator / denominator;
            let x_dist_remaining = m.prize.0 - a_presses * m.a_button.0;
            let b_presses = x_dist_remaining / m.b_button.0;
            return Some(A_PRESS_COST * a_presses + B_PRESS_COST * b_presses);
        }
    }
    None
}

fn part_two_machine_update(machines: &mut Vec<ArcadeMachine>) {
    for m in machines {
        m.prize.0 += PART_TWO_ADD_FACTOR;
        m.prize.1 += PART_TWO_ADD_FACTOR;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, None);
    }
}
