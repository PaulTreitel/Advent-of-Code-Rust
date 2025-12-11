
use advent_of_code_2025::utils::graph_algos::GraphWrapper;
use petgraph::Undirected;


// Solving https://adventofcode.com/2025/day/10
advent_of_code_2025::solution!(10);

type IndicatorState = Vec<bool>;

pub fn part_one(input: &str) -> Option<u64> {
    let machines = parse_input(input);
    let mut button_presses = 0;
    for (indicators, buttons, _) in machines {
        let all_states = get_all_indicator_states(indicators.len());
        let mut edges = vec![];
        for state in &all_states {
            for button in &buttons {
                let new_state = part_one_press_button(state, button);
                edges.push((state.clone(), new_state, 1));
            }
        }
        let start = all_states[all_states.len() - 1].clone();
        let graph: GraphWrapper<Vec<bool>, i32, Undirected> = GraphWrapper::from_nodes_edges(all_states, edges);
        let x = graph.bfs_get_path(graph.vals_to_nodes()[&start][0],
            |ni| graph.node_weight(ni).unwrap().iter().eq(&indicators))
            .unwrap();
        button_presses += x.len();
    }
    Some(button_presses as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let machines = parse_input(input);
    let mut button_presses = 0;
    for (_, buttons, joltages) in machines {
        // button_presses += val;
    }
    Some(button_presses as u64)
}

fn parse_input(input: &str) -> Vec<(IndicatorState, Vec<Vec<usize>>, Vec<i32>)> {
    let mut machines = vec![];
    for line in input.lines() {
        let indicators_else: Vec<_> = line.split("]").collect();
        let indicators = indicators_else[0].as_bytes();
        let indicators: IndicatorState = indicators[1..indicators.len()].iter()
            .map(|&x| x as char == '#')
            .collect();
        let else_joltage: Vec<_> = indicators_else[1].split("{").collect();
        let joltages = else_joltage[1].as_bytes();
        let joltages: String = joltages[..joltages.len() - 1].iter()
            .map(|&x| x as char)
            .collect();
        let joltages: Vec<_> = joltages.split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let buttons: Vec<_> = else_joltage[0].split("(")
            .filter(|s| !s.trim().eq(""))
            .map(|s| s.as_bytes()[..s.as_bytes().len() - 2].iter()
                .map(|&x| x as char).collect::<String>()
                .split(",").map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>())
            .collect();
        machines.push((indicators, buttons, joltages));
    }
    machines
}

fn get_all_indicator_states(n: usize) -> Vec<Vec<bool>> {
    let mut indicator_states = vec![vec![true], vec![false]];
    for _ in 1..n {
        let mut new_indicators = vec![];
        for s in &indicator_states {
            let mut s1 = s.clone();
            s1.push(true);
            new_indicators.push(s1);
            let mut s2 = s.clone();
            s2.push(false);
            new_indicators.push(s2);
        }
        indicator_states = new_indicators;
    }
    indicator_states
}

fn part_two_reverse_button(state: &[i32], button: &[usize]) -> Vec<i32> {
    let mut new_state = state.to_vec();
    for button in button {
        new_state[*button] -= 1;
    }
    new_state
}

fn part_one_press_button(state: &[bool], button: &[usize]) -> Vec<bool> {
    let mut new_state = state.to_vec();
    for button in button {
        new_state[*button] = !new_state[*button];
    }
    new_state
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code_2025::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2025::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(33));
    }
}
