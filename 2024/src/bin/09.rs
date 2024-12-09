use std::fmt::Display;

advent_of_code_2024::solution!(9);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Block {
    File{ id: u128, len: u128 },
    Empty{ len: u128 },
}

const PART_ONE_EMPTY: Block = Block::Empty { len: 1 };
const PART_ONE_LEN: u128 = 1;
const OLD_PRINT: bool = true;

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if OLD_PRINT {
            match self {
                Block::File { id, len: _ } => f.write_str(&id.to_string()),
                Block::Empty { len: _ } => f.write_str("."),
            }
        } else {
            match self {
                Block::File { id, len } => f.write_fmt(format_args!("{{{}, {}}}", id, len)),
                Block::Empty { len } => f.write_fmt(format_args!("{{{}}}", len)),
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u128> {
    let mut blocks = parse_input_part_one(input);
    fill_in_gaps(&mut blocks);
    Some(compute_checksum(&blocks))
}

pub fn part_two(input: &str) -> Option<u128> {
    let mut blocks = parse_input_part_two(input);
    // pretty_print_blocks(&blocks);
    println!();
    let blocks = file_compact(&mut blocks);
    let blocks = reexpand_blocks(&blocks);
    // pretty_print_blocks(&blocks);
    Some(compute_checksum(&blocks))
}

fn parse_input_part_one(input: &str) -> Vec<Block> {
    let mut blocks = vec![];
    let nums: Vec<u128> = input
        .trim()
        .chars()
        .map(|c| c.to_string().parse::<u128>().unwrap())
        .collect();
    for idx in (0..nums.len()).step_by(2) {
        let file = Block::File { id: (idx / 2) as u128, len: PART_ONE_LEN };
        for _ in 0..*nums.get(idx).unwrap() {
            blocks.push(file);
        }
        if idx == nums.len() - 1 {
            break;
        }
        for _ in 0..*nums.get(idx + 1).unwrap() {
            blocks.push(PART_ONE_EMPTY);
        }
    }
    blocks
}

fn parse_input_part_two(input: &str) -> Vec<Block> {
    let mut blocks = vec![];
    let nums: Vec<u128> = input
        .trim()
        .chars()
        .map(|c| c.to_string().parse::<u128>().unwrap())
        .collect();
    for idx in (0..nums.len()).step_by(2) {
        let len = *nums.get(idx).unwrap();
        blocks.push(Block::File { id: (idx / 2) as u128, len });
        if idx == nums.len() - 1 {
            break;
        }
        let len = *nums.get(idx + 1).unwrap();
        blocks.push(Block::Empty { len });
    }
    blocks
}

fn file_compact(blocks: &mut Vec<Block>) -> Vec<Block> {
    let mut new_blocks = blocks.clone();
    blocks.reverse();
    for block in blocks {
        if let Block::File { id: file_id, len: file_len } = block {
            let new_pos = new_blocks.iter().position(|x| {
                match x {
                    Block::File { id: _, len: _ } => false,
                    Block::Empty { len: empty_len } => empty_len >= file_len,
                }
            });
            if let Some(new_pos) = new_pos {
                let old_block_position = new_blocks.iter().position
                    (|b|
                        match b {
                            Block::File { id, len: _ } => id == file_id,
                            Block::Empty { len: _ } => false,
                        }
                    )
                    .unwrap();
                if new_pos > old_block_position {
                    continue;
                }
                // println!("removing {} at {}", new_blocks.get(old_block_position).unwrap(), old_block_position);
                new_blocks.remove(old_block_position);
                if old_block_position == new_blocks.len() {
                    // println!("deleting trailing empty space");
                    new_blocks.pop();
                } else if let Some(Block::Empty { len }) = new_blocks.get_mut(old_block_position - 1) {
                    // println!("adding {} spaces to empty at {}", file_len, old_block_position - 1);
                    *len += *file_len;
                } else {
                    // println!("adding new empty at {} with {}", old_block_position, file_len);
                    new_blocks.insert(old_block_position, Block::Empty { len: *file_len });
                }
                let old_empty = new_blocks.get_mut(new_pos).unwrap();
                if let Block::Empty { len: empty_len } = old_empty {
                    if empty_len == file_len {
                        // println!("eliminating empty {} at {}", new_blocks.get(new_pos).unwrap(), new_pos);
                        new_blocks.remove(new_pos);
                    } else {
                        // println!("taking {} from empty at {}", file_len, new_pos);
                        *empty_len -= *file_len;
                    }
                }
                // println!("reinserting {} at {}", *block, new_pos);
                new_blocks.insert(new_pos, *block);
                // println!("new status:");
                // pretty_print_blocks(&new_blocks);
                // println!();
            }
        }
    }
    new_blocks
}

fn reexpand_blocks(blocks: &Vec<Block>) -> Vec<Block> {
    blocks.iter()
        .map(|b| {
                match b {
                    Block::File { id, len } => vec![Block::File { id: *id, len: 1 }; *len as usize],
                    Block::Empty { len } => vec![PART_ONE_EMPTY; *len as usize],
                }
            }
        )
        .flatten()
        .collect()
}

fn fill_in_gaps(blocks: &mut Vec<Block>) {
    let mut empty_spaces: Vec<usize> = blocks.iter()
        .enumerate()
        .filter(|(_, b)| **b == PART_ONE_EMPTY)
        .map(|(i, _)| i)
        .collect();
    empty_spaces.reverse();
    while let Some(empty_slot) = empty_spaces.pop() {
        if empty_slot >= blocks.len()
            || *blocks.get(empty_slot).unwrap() != PART_ONE_EMPTY
        {
            return;
        }

        let mut file_block = blocks.pop().unwrap();
        while let PART_ONE_EMPTY = file_block {
            file_block = blocks.pop().unwrap();
        }
        if empty_slot >= blocks.len() {
            blocks.push(file_block);
            return;
        }
        *blocks.get_mut(empty_slot).unwrap() = file_block;
    }
}

fn compute_checksum(blocks: &Vec<Block>) -> u128 {
    blocks.iter()
        .enumerate()
        .map(|(idx, b)| {
            match b {
                Block::File { id , len: _} => idx as u128 * id,
                Block::Empty { len: _ } => 0,
            }
        })
        .sum()
}

fn pretty_print_blocks(blocks: &Vec<Block>) {
    for b in blocks {
        print!("{} ", b);
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(2858));
    }
}
