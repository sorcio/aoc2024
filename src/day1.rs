use aoc_runner_derive::{aoc, aoc_generator};

use aoc_utils::{example_tests, known_input_tests};

#[aoc_generator(day1)]
pub fn parse(input: &str) -> Vec<(u32, u32)> {
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
pub fn part1(input: &[(u32, u32)]) -> u32 {
    let mut vec1: Vec<_> = input.iter().map(|(x, _)| *x).collect();
    let mut vec2: Vec<_> = input.iter().map(|(_, y)| *y).collect();
    vec1.sort();
    vec2.sort();
    vec1.into_iter().zip(vec2).map(|(x, y)| x.abs_diff(y)).sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &[(u32, u32)]) -> u32 {
    let mut vec1: Vec<_> = input.iter().map(|(x, _)| *x).collect();
    let mut vec2: Vec<_> = input.iter().map(|(_, y)| *y).collect();
    vec1.sort();
    vec2.sort();
    vec1.into_iter()
        .map(|x| x * u32::try_from(vec2.iter().filter(|y| x == **y).count()).unwrap())
        .sum()
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
    part2 => 31,
}

known_input_tests! {
    input: include_str!("../input/2024/day1.txt"),
    part1 => 1506483,
    part2 => 23126924,
}
