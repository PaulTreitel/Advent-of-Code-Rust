// Solving https://adventofcode.com/2022/day/19
advent_of_code_2022::solution!(19);

use std::cmp::max;


#[derive(Debug)]
struct Blueprint {
    id: i32,
    ore_bot: i32,
    clay_bot: i32,
    obsidian_bot: (i32, i32),
    geode_bot: (i32, i32),
}

#[derive(Debug, Clone)]
struct State {
    mins_left: i32,
    ore: i32,
    clay: i32,
    obsidian: i32,
    geodes: i32,
    ore_bots: i32,
    clay_bots: i32,
    obsidian_bots: i32,
    geode_bots: i32,
}

pub fn part_one(input: &str) -> Option<i32> {
    let prints = get_blueprints(input);
    let mut quality_sum = 0;
    for blueprint in prints {
        let mut max_geodes = 0;
        let mut states = Vec::new();
        states.push(State {
            mins_left: 24,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
            ore_bots: 1,
            clay_bots: 0,
            obsidian_bots: 0,
            geode_bots: 0,
        });

        while !states.is_empty() {
            let curr = states.pop().unwrap();
            if curr.mins_left == 1 {
                max_geodes = max(max_geodes, curr.geodes + curr.geode_bots);
                continue;
            }
            let purchaseable = get_purchaseable(&blueprint, &curr);
            let mut new = curr.clone();
            time_step(&mut new);
            states.push(new);
            add_purchase_states(&mut states, &blueprint, &curr, purchaseable);
        }
        quality_sum += max_geodes * blueprint.id;
    }
    Some(quality_sum)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut prints = get_blueprints(input);
    while prints.len() > 3 {
        prints.pop();
    }
    let mut geode_product = 1;

    for blueprint in prints {
        let mut max_geodes = 0;
        let mut states = Vec::new();
        states.push(State {
            mins_left: 32,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
            ore_bots: 1,
            clay_bots: 0,
            obsidian_bots: 0,
            geode_bots: 0,
        });

        while !states.is_empty() {
            let curr = states.pop().unwrap();
            if curr.mins_left == 1 {
                max_geodes = max(max_geodes, curr.geodes + curr.geode_bots);
                continue;
            }
            let purchaseable = get_purchaseable(&blueprint, &curr);
            let mut new = curr.clone();
            time_step(&mut new);
            states.push(new);
            add_purchase_states(&mut states, &blueprint, &curr, purchaseable);
        }
        geode_product *= max_geodes;
    }
    Some(geode_product)
}

fn add_purchase_states(
    states: &mut Vec<State>,
    bprint: &Blueprint,
    curr_state: &State,
    purchaseable: [bool; 4]
) -> () {
    let max_ore_cost = max(bprint.ore_bot,
        max(bprint.clay_bot,
            max(bprint.obsidian_bot.0, bprint.geode_bot.0)));
    if *purchaseable.get(0).unwrap() && curr_state.ore_bots < max_ore_cost {
        let could_have_purchased = curr_state.ore >= curr_state.ore_bots + bprint.ore_bot;
        if !could_have_purchased {
            let mut new = curr_state.clone();
            new.ore -= bprint.ore_bot;
            time_step(&mut new);
            new.ore_bots += 1;
            states.push(new);
        }
    }
    if *purchaseable.get(1).unwrap() && curr_state.clay_bots < bprint.obsidian_bot.1 {
        let could_have_purchased = curr_state.ore >= curr_state.ore_bots + bprint.clay_bot;
        if !could_have_purchased {
            let mut new = curr_state.clone();
            new.ore -= bprint.clay_bot;
            time_step(&mut new);
            new.clay_bots += 1;
            states.push(new);
        }
    }
    if *purchaseable.get(2).unwrap() && curr_state.obsidian_bots < bprint.geode_bot.1 {
        let could_have_purchased = curr_state.ore >= curr_state.ore_bots + bprint.obsidian_bot.0
            && curr_state.clay >= curr_state.clay_bots + bprint.obsidian_bot.1;
        if !could_have_purchased {
            let mut new = curr_state.clone();
            new.ore -= bprint.obsidian_bot.0;
            new.clay -= bprint.obsidian_bot.1;
            time_step(&mut new);
            new.obsidian_bots += 1;
            states.push(new);
        }
    }
    if *purchaseable.get(3).unwrap() {
        let mut new = curr_state.clone();
        new.ore -= bprint.geode_bot.0;
        new.obsidian -= bprint.geode_bot.1;
        time_step(&mut new);
        new.geode_bots += 1;
        states.push(new);
    }
}

fn time_step(state: &mut State) {
    state.mins_left -= 1;
    state.ore += state.ore_bots;
    state.clay += state.clay_bots;
    state.obsidian += state.obsidian_bots;
    state.geodes += state.geode_bots;
}

fn get_purchaseable(bprint: &Blueprint, curr_state: &State) -> [bool; 4] {
    [
        curr_state.ore >= bprint.ore_bot,
        curr_state.ore >= bprint.clay_bot,
        curr_state.ore >= bprint.obsidian_bot.0 && curr_state.clay >= bprint.obsidian_bot.1,
        curr_state.ore >= bprint.geode_bot.0 && curr_state.obsidian >= bprint.geode_bot.1,
    ]
}

fn get_blueprints(input: &str) -> Vec<Blueprint> {
    let mut prints = Vec::new();
    for line in input.lines() {
        let mut line = line.split(",");
        let id = line.next().unwrap().parse::<i32>().unwrap();
        let ore_bot_ore = line.next().unwrap().parse::<i32>().unwrap();
        let clay_bot_ore = line.next().unwrap().parse::<i32>().unwrap();
        let obsi_bot_ore = line.next().unwrap().parse::<i32>().unwrap();
        let obsi_bot_clay = line.next().unwrap().parse::<i32>().unwrap();
        let geode_bot_ore = line.next().unwrap().parse::<i32>().unwrap();
        let geode_bot_obsidian = line.next().unwrap().parse::<i32>().unwrap();
        let new = Blueprint {
            id: id,
            ore_bot: ore_bot_ore,
            clay_bot: clay_bot_ore,
            obsidian_bot: (obsi_bot_ore, obsi_bot_clay),
            geode_bot: (geode_bot_ore, geode_bot_obsidian),
        };
        prints.push(new);
    }
    prints
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(33));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(3472));
    }
}
