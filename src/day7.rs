use aoc_runner_derive::{aoc, aoc_generator};

use aoc_utils::{example_tests, known_input_tests};

pub struct Equation {
    result: u64,
    operands: Vec<u64>,
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

impl Equation {
    fn operator_combinations(&self, ops: &[Operator]) -> impl Iterator<Item = Vec<Operator>> {
        let n = self.operands.len() - 1;
        (0..ops.len().pow(n as _)).map(move |mut i| {
            (0..n)
                .map(move |_| {
                    let operator = match i % ops.len() {
                        0 => Operator::Add,
                        1 => Operator::Multiply,
                        2 => Operator::Concatenate,
                        _ => unreachable!(),
                    };
                    i /= ops.len();
                    operator
                })
                .collect()
        })
    }
}

#[aoc_generator(day7)]
pub fn parse(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let result = parts.next().unwrap().parse().unwrap();
            let operands = parts
                .next()
                .unwrap()
                .split(' ')
                .map(|part| part.parse().unwrap())
                .collect();
            Equation { result, operands }
        })
        .collect()
}

#[aoc(day7, part1)]
fn part1(input: &[Equation]) -> u64 {
    input
        .iter()
        .map(|equation| {
            for operators in equation.operator_combinations(&[Operator::Add, Operator::Multiply]) {
                let mut result = equation.operands[0];
                for (operator, operand) in operators.iter().zip(equation.operands.iter().skip(1)) {
                    match operator {
                        Operator::Add => result += operand,
                        Operator::Multiply => result *= operand,
                        _ => unreachable!(),
                    }
                }
                if result == equation.result {
                    return result;
                }
            }
            0
        })
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &[Equation]) -> u64 {
    input
        .iter()
        .map(|equation| {
            // TODO: instead of iterating over vectors of operators, expand the combinations
            // inline so we can prune early if the result is too large (and avoid allocations)
            for operators in equation.operator_combinations(&[
                Operator::Add,
                Operator::Multiply,
                Operator::Concatenate,
            ]) {
                let mut result = equation.operands[0];
                for (operator, operand) in operators.iter().zip(equation.operands.iter().skip(1)) {
                    match operator {
                        Operator::Add => result += operand,
                        Operator::Multiply => result *= operand,
                        Operator::Concatenate => {
                            // TODO: more efficient way to concatenate decimal numbers?
                            result = format!("{}{}", result, operand).parse().unwrap()
                        }
                    }
                }
                if result == equation.result {
                    return result;
                }
            }
            0
        })
        .sum()
}

example_tests! {
    "
    190: 10 19
    3267: 81 40 27
    83: 17 5
    156: 15 6
    7290: 6 8 6 15
    161011: 16 10 13
    192: 17 8 14
    21037: 9 7 18 13
    292: 11 6 16 20
    ",
    part1 => 3749,
    part2 => 11387,
}

known_input_tests! {
    input: include_str!("../input/2024/day7.txt"),
    part1 => 42283209483350,
    part2 => 1026766857276279,
}
