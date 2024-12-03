use aoc_runner_derive::{aoc, aoc_generator};

use crate::testing::example_tests;

#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<(u32, u32)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &[(u32, u32)]) -> u32 {
    let mut vec1: Vec<_> = input.iter().map(|(x, _)| *x).collect();
    let mut vec2: Vec<_> = input.iter().map(|(_, y)| *y).collect();
    vec1.sort();
    vec2.sort();
    vec1.into_iter().zip(vec2).map(|(x, y)| x.abs_diff(y)).sum()
}

#[aoc(day1, part2)]
fn part2(_input: &[(u32, u32)]) -> String {
    todo!()
}

example_tests! {
    "
    3   4
    4   3
    2   5
    1   3
    3   9
    3   3
    ",

    part1 => 11,
}
