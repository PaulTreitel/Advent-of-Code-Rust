advent_of_code_2022::solution!(4);

pub fn part_one(input: &str) -> Option<i32> {
    let mut num_fully_overlapping = 0;
    for line in input.lines() {
        let mut elf_ranges = line.split(",");
        let elf_0_range: (i32, i32) = get_elf_range(elf_ranges.next().unwrap());
        let elf_1_range: (i32, i32) = get_elf_range(elf_ranges.next().unwrap());
        if elf_range_fully_contains(elf_0_range, elf_1_range) {
            num_fully_overlapping += 1;
        }
    }
    Some(num_fully_overlapping)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut num_overlapping = 0;
    for line in input.lines() {
        let mut elf_ranges = line.split(",");
        let elf_0_range: (i32, i32) = get_elf_range(elf_ranges.next().unwrap());
        let elf_1_range: (i32, i32) = get_elf_range(elf_ranges.next().unwrap());
        if elf_range_overlaps(elf_0_range, elf_1_range) {
            num_overlapping += 1;
        }
    }
    Some(num_overlapping)
}

fn get_elf_range(range: &str) -> (i32, i32) {
    let mut range = range.split("-");
    let start: i32 = range.next().unwrap().parse().unwrap();
    let end: i32 = range.next().unwrap().parse().unwrap();
    (start, end)
}

fn elf_range_fully_contains(elf1: (i32, i32), elf2: (i32, i32)) -> bool {
    (elf1.0 <= elf2.0 && elf1.1 >= elf2.1) || (elf2.0 <= elf1.0 && elf2.1 >= elf1.1)
}

fn elf_range_overlaps(elf1: (i32, i32), elf2: (i32, i32)) -> bool {
    (elf1.1 >= elf2.0 && elf1.0 <= elf2.1) || (elf2.1 >= elf1.0 && elf2.0 <= elf1.1)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(4));
    }
}
