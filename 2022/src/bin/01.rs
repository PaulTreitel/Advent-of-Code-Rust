advent_of_code_2022::solution!(1);

fn main() {
    part1();
    part2();
}

fn part2() {
    let mut calorie_counts = get_elf_calorie_list();
    calorie_counts.sort();
    calorie_counts.reverse();
    let top_3_elf_calories: u32 = calorie_counts[0..3].iter().sum();
    println!("{}", top_3_elf_calories);
}

fn part1() {
    let calorie_counts = get_elf_calorie_list();
    let max_calories = calorie_counts.iter().max().unwrap();
    println!("{}", max_calories);
}

fn get_elf_calorie_list() -> Vec<u32> {
    let input = advent_of_code_2022::template::read_file("inputs", DAY);
    let input = input.lines();
    let mut calorie_counts: Vec<u32> = vec![0];
    let mut elf_index = 0;
    for line in input {
        if line.len() == 0 {
            elf_index += 1;
        } else {
            if calorie_counts.len() == elf_index {
                calorie_counts.push(0);
            }
            let item_calories: u32 = line.parse().unwrap();
            calorie_counts[elf_index] += item_calories;
        }
    }
    calorie_counts
}
