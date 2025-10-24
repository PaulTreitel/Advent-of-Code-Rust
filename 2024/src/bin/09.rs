// Solving https://adventofcode.com/2024/day/9
advent_of_code_2024::solution!(9);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Block {
    File { id: u128, len: u128 },
    Empty { len: u128 },
}

const PART_ONE_EMPTY: Block = Block::Empty { len: 1 };

pub fn part_one(input: &str) -> Option<u128> {
    let mut blocks = parse_input(input);
    blocks = expand_blocks(&blocks);
    fill_in_gaps(&mut blocks);
    Some(compute_checksum(&blocks))
}

pub fn part_two(input: &str) -> Option<u128> {
    let mut blocks = parse_input(input);
    blocks = file_compact(&mut blocks);
    blocks = expand_blocks(&blocks);
    Some(compute_checksum(&blocks))
}

fn parse_input(input: &str) -> Vec<Block> {
    let mut blocks = vec![];
    let nums: Vec<u128> = input
        .trim()
        .chars()
        .map(|c| c.to_string().parse::<u128>().unwrap())
        .collect();
    for idx in (0..nums.len()).step_by(2) {
        let len = *nums.get(idx).unwrap();
        blocks.push(Block::File {
            id: (idx / 2) as u128,
            len,
        });
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
        if let Block::File {
            id: file_id,
            len: file_len,
        } = block
        {
            let new_pos = new_blocks.iter().position(|x| match x {
                Block::File { id: _, len: _ } => false,
                Block::Empty { len: empty_len } => empty_len >= file_len,
            });
            if let Some(new_pos) = new_pos {
                reposition_file(&mut new_blocks, new_pos, *file_id, *file_len);
            }
        }
    }
    new_blocks
}

fn reposition_file(blocks: &mut Vec<Block>, new_pos: usize, file_id: u128, file_len: u128) {
    let old_block_position = blocks
        .iter()
        .position(|b| match b {
            Block::File { id, len: _ } => *id == file_id,
            Block::Empty { len: _ } => false,
        })
        .unwrap();
    if new_pos > old_block_position {
        return;
    }
    blocks.remove(old_block_position);
    // Deal with empty space left behind by the block we're removing. Drop it if
    // it's at the end, expand it if there is one already, or add a new one.
    if old_block_position == blocks.len() {
        blocks.pop();
    } else if let Some(Block::Empty { len }) = blocks.get_mut(old_block_position - 1) {
        *len += file_len;
    } else {
        blocks.insert(old_block_position, Block::Empty { len: file_len });
    }
    let old_empty = blocks.get_mut(new_pos).unwrap();
    if let Block::Empty { len: empty_len } = old_empty {
        if *empty_len == file_len {
            blocks.remove(new_pos);
        } else {
            *empty_len -= file_len;
        }
    }
    blocks.insert(
        new_pos,
        Block::File {
            id: file_id,
            len: file_len,
        },
    );
}

fn expand_blocks(blocks: &[Block]) -> Vec<Block> {
    blocks
        .iter()
        .flat_map(|b| match b {
            Block::File { id, len } => vec![Block::File { id: *id, len: 1 }; *len as usize],
            Block::Empty { len } => vec![PART_ONE_EMPTY; *len as usize],
        })
        .collect()
}

fn fill_in_gaps(blocks: &mut Vec<Block>) {
    let mut empty_spaces: Vec<usize> = blocks
        .iter()
        .enumerate()
        .filter(|(_, b)| **b == PART_ONE_EMPTY)
        .map(|(i, _)| i)
        .collect();
    empty_spaces.reverse();
    while let Some(empty_slot) = empty_spaces.pop() {
        if empty_slot >= blocks.len() || *blocks.get(empty_slot).unwrap() != PART_ONE_EMPTY {
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

fn compute_checksum(blocks: &[Block]) -> u128 {
    blocks
        .iter()
        .enumerate()
        .map(|(idx, b)| match b {
            Block::File { id, len: _ } => idx as u128 * id,
            Block::Empty { len: _ } => 0,
        })
        .sum()
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
