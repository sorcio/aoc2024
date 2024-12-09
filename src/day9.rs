use aoc_runner_derive::{aoc, aoc_generator};

use crate::testing::example_tests;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Block {
    Free,
    File(u16),
}

fn checksum(blocks: &[Block]) -> u32 {
    blocks
        .iter()
        .enumerate()
        .map(|(i, b)| match b {
            Block::Free => 0,
            Block::File(f) => (i as u32) * *f as u32,
        })
        .sum()
}

#[aoc_generator(day9)]
fn parse(input: &[u8]) -> Vec<Block> {
    let length = input.iter().map(|c| (*c - b'0') as usize).sum();
    let mut result = vec![Block::Free; length];
    let mut next = 0;
    for (i, chunk) in input.chunks(2).enumerate() {
        let file_blocks = (chunk[0] - b'0') as usize;
        let free_blocks = chunk.get(1).map(|b| (b - b'0') as usize).unwrap_or(0);
        for j in 0..file_blocks {
            result[next + j] = Block::File(i as u16);
        }
        next += file_blocks + free_blocks;
    }
    result
}

#[aoc(day9, part1)]
fn part1(input: &Vec<Block>) -> u32 {
    let mut blocks = input.clone();

    let mut next_free = blocks.iter().position(|&b| b == Block::Free).unwrap();
    let mut last_block = blocks.len() - 1;
    assert!(blocks[last_block] != Block::Free);
    while next_free < last_block {
        blocks.swap(next_free, last_block);
        match blocks
            .iter()
            .skip(next_free)
            .position(|&b| b == Block::Free)
        {
            Some(free) => next_free += free,
            None => break,
        }
        while last_block > next_free && blocks[last_block] == Block::Free {
            last_block -= 1;
        }
    }
    blocks.truncate(next_free);
    debug_assert!(blocks.iter().all(|b| *b != Block::Free));
    checksum(&blocks)
}

#[aoc(day9, part2)]
fn part2(input: &[Block]) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser() {
        let blocks = parse(b"2333133121414131402");
        assert_eq!(
            blocks,
            // 00...111...2...333.44.5555.6666.777.888899
            [
                Block::File(0),
                Block::File(0),
                Block::Free,
                Block::Free,
                Block::Free,
                Block::File(1),
                Block::File(1),
                Block::File(1),
                Block::Free,
                Block::Free,
                Block::Free,
                Block::File(2),
                Block::Free,
                Block::Free,
                Block::Free,
                Block::File(3),
                Block::File(3),
                Block::File(3),
                Block::Free,
                Block::File(4),
                Block::File(4),
                Block::Free,
                Block::File(5),
                Block::File(5),
                Block::File(5),
                Block::File(5),
                Block::Free,
                Block::File(6),
                Block::File(6),
                Block::File(6),
                Block::File(6),
                Block::Free,
                Block::File(7),
                Block::File(7),
                Block::File(7),
                Block::Free,
                Block::File(8),
                Block::File(8),
                Block::File(8),
                Block::File(8),
                Block::File(9),
                Block::File(9),
            ]
        );
    }
}

example_tests! {
    b"2333133121414131402",
    part1 => 1928,
}
