use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::{
    testing::example_tests,
    utils::{NumberExt, Parity},
};
#[aoc_generator(day11)]
fn parse(input: &str) -> Vec<u64> {
    input
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn split_number(n: u64) -> Option<(u64, u64)> {
    debug_assert!(n != 0);
    let digits = n.ilog10() + 1;
    if digits.parity() == Parity::Even {
        let mut left = n;
        let mut right = 0;
        for i in 0..digits / 2 {
            right = left % 10 * 10u64.pow(i) + right;
            left /= 10;
        }
        Some((left, right))
    } else {
        None
    }
}

fn expand_25_times(n: u64) -> Vec<u64> {
    const ITERATIONS: usize = 25;
    let mut input_vec = Vec::with_capacity(32768);
    input_vec.push(n);
    let mut next_vec = Vec::with_capacity(32768);
    let mut stones = &mut input_vec;
    let mut next = &mut next_vec;
    for _ in 0..ITERATIONS {
        next.clear();
        for stone in stones.iter().copied() {
            if stone == 0 {
                next.push(1);
            } else if let Some((left, right)) = split_number(stone) {
                next.push(left);
                next.push(right);
            } else {
                next.push(stone * 2024);
            }
        }
        std::mem::swap(&mut stones, &mut next);
    }
    next_vec
}

struct Expander {
    cache: HashMap<u64, Vec<u64>>,
}

impl Expander {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    fn expand_25_times(&mut self, n: u64) -> &[u64] {
        if self.cache.get(&n).is_none() {
            let expanded = expand_25_times(n);
            self.cache.insert(n, expanded);
        }
        self.cache.get(&n).unwrap()
    }
}

#[aoc(day11, part1)]
fn part1(input: &[u64]) -> usize {
    // input.iter().map(|n| expand_25_times(*n).len()).sum()
    let mut expander = Expander::new();
    input
        .iter()
        .map(|n| expander.expand_25_times(*n).len())
        .sum()

    // const ITERATIONS: usize = 25;
    // let mut input_vec = Vec::with_capacity(input.len() * 32768);
    // input_vec.extend_from_slice(input);
    // let mut next_vec = Vec::with_capacity(input.len() * 32768);
    // let mut stones = &mut input_vec;
    // let mut next = &mut next_vec;
    // for _ in 0..ITERATIONS {
    //     next.clear();
    //     for stone in stones.iter().copied() {
    //         if stone == 0 {
    //             next.push(1);
    //         } else if let Some((left, right)) = split_number(stone) {
    //             next.push(left);
    //             next.push(right);
    //         } else {
    //             next.push(stone * 2024);
    //         }
    //     }
    //     std::mem::swap(&mut stones, &mut next);
    // }
    // stones.len().try_into().unwrap()
}

#[aoc(day11, part2)]
fn part2(input: &[u64]) -> usize {
    let mut expander = Expander::new();
    input
        .iter()
        .map(|n| {
            // expand 75 times == expand 25 times 3 times
            let expanded = expander.expand_25_times(*n).to_vec();
            dbg!(expander.cache.len());
            let expanded_twice: Vec<u64> = expanded
                .into_iter()
                .flat_map(|n| expander.expand_25_times(n).to_vec())
                .collect();
            expanded_twice
                .into_iter()
                .map(|n| expander.expand_25_times(n).len())
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_number_test() {
        assert_eq!(split_number(1), None);
        assert_eq!(split_number(10), Some((1, 0)));
        assert_eq!(split_number(100), None);
        assert_eq!(split_number(1000), Some((10, 0)));
        assert_eq!(split_number(123), None);
        assert_eq!(split_number(17), Some((1, 7)));
        assert_eq!(split_number(123456), Some((123, 456)));
    }
}

example_tests! {
    "125 17",

    part1 => 55312,
}
