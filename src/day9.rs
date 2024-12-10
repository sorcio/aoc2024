use aoc_runner_derive::aoc;

use crate::testing::{example_tests, known_input_tests};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Block {
    Free,
    File(u16),
}

fn checksum(blocks: &[Block]) -> u64 {
    blocks
        .iter()
        .enumerate()
        .map(|(i, b)| match b {
            Block::Free => 0,
            Block::File(f) => (i as u64).checked_mul(*f as u64).unwrap(),
        })
        .sum()
}

fn parse_blocks(input: &[u8]) -> Vec<Block> {
    let input = input.trim_ascii_end();
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

/// a cluster represents a file or a sequence of free blocks
#[derive(Debug, Clone)]
struct Cluster {
    position: usize,
    size: usize,
}

/// File positions table (for part 2)
struct FileTable {
    files: Vec<Cluster>,
    frees: Vec<Cluster>,
}

fn parse_clusters(input: &[u8]) -> FileTable {
    let input = input.trim_ascii_end();
    let mut files = vec![];
    let mut frees = vec![];

    let mut position = 0;
    let mut is_free = false;
    for b in input {
        let size = (*b - b'0') as usize;
        if size > 0 {
            if is_free { &mut frees } else { &mut files }.push(Cluster { position, size });
        }
        position += size;
        is_free = !is_free;
    }
    FileTable { files, frees }
}

fn compress_fragmenting(blocks: &mut [Block]) -> usize {
    let mut next_free = blocks.iter().position(|&b| b == Block::Free).unwrap();
    let mut last_block = blocks.len() - 1;
    while next_free < last_block {
        blocks.swap(next_free, last_block);
        while blocks[next_free] != Block::Free {
            next_free += 1;
        }
        loop {
            last_block -= 1;
            if last_block <= next_free || blocks[last_block] != Block::Free {
                break;
            }
        }
    }
    last_block
}

#[aoc(day9, part1)]
fn part1(input: &[u8]) -> u64 {
    let mut blocks = parse_blocks(input);
    let new_size = compress_fragmenting(&mut blocks) + 1;
    blocks.truncate(new_size);
    checksum(&blocks)
}

#[aoc(day9, part2)]
fn part2(input: &[u8]) -> u64 {
    let mut table = parse_clusters(input);

    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut frees_by_size: [BinaryHeap<Reverse<usize>>; 10] = [(); 10].map(|_| Default::default());
    for free in table.frees.iter() {
        frees_by_size[free.size].push(Reverse(free.position));
    }

    for file in table.files.iter_mut().rev() {
        let mut first_free = usize::MAX;
        let mut size = None;
        for (i, frees) in frees_by_size.iter().enumerate().skip(file.size) {
            if let Some(Reverse(pos)) = frees.peek() {
                if *pos < file.position && *pos < first_free {
                    first_free = *pos;
                    size = Some(i);
                }
            }
        }
        if let Some(free_size) = size {
            let new_position = frees_by_size[free_size].pop().unwrap().0;
            let difference = free_size - file.size;
            file.position = new_position;

            if difference > 0 {
                let new_free_position = new_position + file.size;
                frees_by_size[difference].push(Reverse(new_free_position));
            }
        }
    }

    // compact formula for checksum
    table
        .files
        .iter()
        .enumerate()
        .map(|(id, f)| (id as u64) * (f.size * (f.position * 2 + f.size - 1) / 2) as u64)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser() {
        let blocks = parse_blocks(b"2333133121414131402");
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

    #[test]
    fn checksum_example() {
        let blocks = b"0099811188827773336446555566"
            .iter()
            .map(|c| Block::File((c - b'0') as _))
            .collect::<Vec<_>>();
        assert_eq!(checksum(&blocks), 1928);
    }

    #[test]
    fn compress_easy() {
        let mut blocks = parse_blocks(b"12345");
        let new_size = compress_fragmenting(&mut blocks);
        assert_eq!(new_size, 9);
        assert_eq!(blocks, [
            Block::File(0),
            Block::File(2),
            Block::File(2),
            Block::File(1),
            Block::File(1),
            Block::File(1),
            Block::File(2),
            Block::File(2),
            Block::File(2),
            Block::Free,
            Block::Free,
            Block::Free,
            Block::Free,
            Block::Free,
            Block::Free,
        ]);
    }
}

example_tests! {
    parser: None,
    b"2333133121414131402",
    part1 => 1928,
    part2 => 2858,
}

known_input_tests! {
    parser: None,
    input: include_bytes!("../input/2024/day9.txt"),
    part1 => 6399153661894,
    part2 => 6421724645083,
}
