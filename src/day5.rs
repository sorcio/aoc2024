use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::testing::{example_tests, known_input_tests};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Page(u8);

impl FromStr for Page {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(Page)
    }
}

impl Page {
    fn number(self) -> usize {
        self.0 as _
    }
}

#[derive(Debug, Clone)]
pub struct Puzzle {
    ordering_constraints: Vec<(Page, Page)>,
    candidate_orderings: Vec<Vec<Page>>,
}

#[aoc_generator(day5)]
pub fn parse(input: &str) -> Puzzle {
    let mut lines = input.lines();
    let ordering_constraints = {
        let mut constraints = vec![];
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            let mut parts = line.split('|');
            let first = parts.next().unwrap().parse().unwrap();
            let second = parts.next().unwrap().parse().unwrap();
            constraints.push((first, second));
        }
        constraints
    };
    let candidate_orderings = {
        let mut orderings = vec![];
        for line in lines {
            let ordering = line.split(',').map(|page| page.parse().unwrap()).collect();
            orderings.push(ordering);
        }
        orderings
    };
    Puzzle {
        ordering_constraints,
        candidate_orderings,
    }
}

#[aoc(day5, part1)]
pub fn part1(input: &Puzzle) -> usize {
    input
        .candidate_orderings
        .iter()
        .filter(|ordering| {
            input.ordering_constraints.iter().all(|(first, second)| {
                let first_index = ordering.iter().position(|&page| page == *first);
                let second_index = ordering.iter().position(|&page| page == *second);
                first_index
                    .and_then(|first_index| {
                        second_index.map(|second_index| first_index < second_index)
                    })
                    .unwrap_or(true)
            })
        })
        .map(|ordering| ordering[ordering.len() / 2].number())
        .sum()
}

#[aoc(day5, part2)]
pub fn part2(input: &Puzzle) -> usize {
    input
        .candidate_orderings
        .iter()
        .filter(|ordering| {
            !input.ordering_constraints.iter().all(|(first, second)| {
                let first_index = ordering.iter().position(|&page| page == *first);
                let second_index = ordering.iter().position(|&page| page == *second);
                first_index
                    .and_then(|first_index| {
                        second_index.map(|second_index| first_index < second_index)
                    })
                    .unwrap_or(true)
            })
        })
        .cloned()
        .map(|mut ordering| {
            // let's just find the indexes again because I'm lazy like that
            while let Some((first_index, second_index)) = input
                .ordering_constraints
                .iter()
                .find_map(|(first, second)| {
                    let first_index = ordering.iter().position(|&page| page == *first);
                    let second_index = ordering.iter().position(|&page| page == *second);
                    first_index
                        .and_then(|first_index| {
                            second_index.map(|second_index| (first_index, second_index))
                        })
                        .filter(|(first_index, second_index)| first_index < second_index)
                })
            {
                ordering.swap(first_index, second_index);
            }

            ordering[ordering.len() / 2].number()
        })
        .sum()
}

example_tests! {
    "
    47|53
    97|13
    97|61
    97|47
    75|29
    61|13
    75|53
    29|13
    97|29
    53|29
    61|53
    97|53
    61|29
    47|13
    75|47
    97|75
    47|61
    75|61
    47|29
    75|13
    53|13

    75,47,61,53,29
    97,61,53,29,13
    75,29,13
    75,97,47,61,53
    61,13,29
    97,13,75,29,47
    ",
    part1 => 143,
    part2 => 123,
}

known_input_tests! {
    input: include_str!("../input/2024/day5.txt"),
    part1 => 5064,
    part2 => 5152,
}
