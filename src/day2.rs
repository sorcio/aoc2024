use aoc_runner_derive::{aoc, aoc_generator};

use aoc_utils::{example_tests, known_input_tests};
#[aoc_generator(day2)]
pub fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[Vec<i32>]) -> usize {
    input
        .iter()
        .filter(|row| {
            let diffs: Vec<i32> = row.windows(2).map(|x| x[1] - x[0]).collect();
            let max = *diffs.iter().max().unwrap();
            let min = *diffs.iter().min().unwrap();
            (max <= 3 && min > 0) || (max < 0 && min >= -3)
        })
        .count()
}

#[aoc(day2, part2)]
fn part2(input: &[Vec<i32>]) -> usize {
    input
        .iter()
        .filter(|row| {
            // brute force solution: remove each element and check
            (0..row.len()).any(|i| {
                let mut row = (*row).clone();
                row.remove(i);
                let diffs: Vec<i32> = row.windows(2).map(|x| x[1] - x[0]).collect();
                let max = *diffs.iter().max().unwrap();
                let min = *diffs.iter().min().unwrap();
                (max <= 3 && min > 0) || (max < 0 && min >= -3)
            })
        })
        .count()
}

example_tests! {
    "
    7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9
    ",

    part1 => 2,
    part2 => 4,
}

known_input_tests! {
    input: include_str!("../input/2024/day2.txt"),
    part1 => 624,
    part2 => 658,
}
