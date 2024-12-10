use aoc_runner_derive::aoc;

use crate::testing::{example_tests, known_input_tests};

fn part1_parse(line: &str) -> Vec<(u64, u64)> {
    let mut result = vec![];

    // we don't have access to a regex implementation, so let's do a quick
    // and dirty state machine parsing
    enum State {
        Initial,
        M,
        Mu,
        Mul,
        FirstNumber(usize),
        SecondNumber(usize),
    }

    let mut state = State::Initial;
    let mut current_number = 0;
    let mut first_number = 0;

    for c in line.chars() {
        match (state, c) {
            (State::Initial, 'm') => state = State::M,
            (State::M, 'u') => state = State::Mu,
            (State::Mu, 'l') => state = State::Mul,
            (State::Mul, '(') => {
                state = State::FirstNumber(0);
                current_number = 0;
            }
            (State::FirstNumber(n), _) if n <= 3 && c.is_ascii_digit() => {
                current_number = current_number * 10 + c.to_digit(10).unwrap() as u64;
                state = State::FirstNumber(n + 1);
            }
            (State::FirstNumber(n), ',') if n > 0 && n <= 3 => {
                first_number = current_number;
                current_number = 0;
                state = State::SecondNumber(0);
            }
            (State::SecondNumber(n), _) if n <= 3 && c.is_ascii_digit() => {
                current_number = current_number * 10 + c.to_digit(10).unwrap() as u64;
                state = State::SecondNumber(n + 1);
            }
            (State::SecondNumber(n), ')') if n > 0 && n <= 3 => {
                let second_number = current_number;
                result.push((first_number, second_number));
                state = State::Initial;
            }
            _ => state = State::Initial,
        }
    }
    result
}

fn part2_parse(line: &str) -> Vec<(u64, u64)> {
    // same as part 1 but with more parsing states
    let mut result = vec![];

    // we don't have access to a regex implementation, so let's do a quick
    // and dirty state machine parsing
    enum State {
        Initial,
        M,
        Mu,
        Mul,
        D,
        Do,
        DoOpen,
        Don,
        DonQuote,
        DonQuoteT,
        DonQuoteTParen,
        FirstNumber(usize),
        SecondNumber(usize),
    }

    let mut state = State::Initial;
    let mut current_number = 0;
    let mut first_number = 0;
    let mut active = true;

    for c in line.chars() {
        match (state, c) {
            (State::Initial, 'm') => state = State::M,
            (State::M, 'u') => state = State::Mu,
            (State::Mu, 'l') => state = State::Mul,
            (State::Mul, '(') => {
                state = State::FirstNumber(0);
                current_number = 0;
            }
            (State::Initial, 'd') => state = State::D,
            (State::D, 'o') => state = State::Do,
            (State::Do, '(') => state = State::DoOpen,
            (State::DoOpen, ')') => {
                active = true;
                state = State::Initial;
            }
            (State::Do, 'n') => state = State::Don,
            (State::Don, '\'') => state = State::DonQuote,
            (State::DonQuote, 't') => state = State::DonQuoteT,
            (State::DonQuoteT, '(') => state = State::DonQuoteTParen,
            (State::DonQuoteTParen, ')') => {
                active = false;
                state = State::Initial;
            }
            (State::FirstNumber(n), _) if n <= 3 && c.is_ascii_digit() => {
                current_number = current_number * 10 + c.to_digit(10).unwrap() as u64;
                state = State::FirstNumber(n + 1);
            }
            (State::FirstNumber(n), ',') if n > 0 && n <= 3 => {
                first_number = current_number;
                current_number = 0;
                state = State::SecondNumber(0);
            }
            (State::SecondNumber(n), _) if n <= 3 && c.is_ascii_digit() => {
                current_number = current_number * 10 + c.to_digit(10).unwrap() as u64;
                state = State::SecondNumber(n + 1);
            }
            (State::SecondNumber(n), ')') if n > 0 && n <= 3 => {
                let second_number = current_number;
                if active {
                    result.push((first_number, second_number));
                }
                state = State::Initial;
            }
            _ => state = State::Initial,
        }
    }
    result
}

#[aoc(day3, part1)]
fn part1(input: &str) -> u64 {
    part1_parse(input).into_iter().map(|(a, b)| a * b).sum()
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u64 {
    part2_parse(input).into_iter().map(|(a, b)| a * b).sum()
}

example_tests! {
    parser: None,
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
    part1 => 161,

    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
    part2 => 48,
}

known_input_tests! {
    parser: None,
    input: include_str!("../input/2024/day3.txt"),
    part1 => 157621318,
    part2 => 79845780,
}
