use aoc_runner_derive::{aoc, aoc_generator};

use crate::{
    testing::example_tests,
    utils::{AsciiUtils, FromAscii},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vector {
    x: i32,
    y: i32,
}

impl FromAscii for i32 {
    type Slice<'a> = &'a [u8];
    type Error = std::num::ParseIntError;
    fn from_ascii(s: Self::Slice<'_>) -> Result<Self, Self::Error> {
        std::str::from_utf8(s).unwrap().parse()
    }
}

struct Machine {
    button_a: Vector,
    button_b: Vector,
    target: Vector,
}

fn parse_button_line(line: &[u8]) -> Vector {
    // "Button A: X+51, Y+31" -> Vector { x: 51, y: 31 }
    // not pretty but: all the numbers are 2 digits
    let x = line[12..14].parse().unwrap();
    let y = line[18..20].parse().unwrap();
    Vector { x, y }
}

fn parse_target_line(line: &[u8]) -> Vector {
    // "Prize: X=8400, Y=15400" -> Vector { x: 8400, y: 15400 }
    // numbers have different lengths so can't use the same trick
    let line = &line[9..];
    let x_length = line.iter().position(|&c| c == b',').unwrap();
    let x = line[..x_length].parse().unwrap();
    let y = line[x_length + 4..].parse().unwrap();
    Vector { x, y }
}

#[aoc_generator(day13)]
fn parse(input: &[u8]) -> Vec<Machine> {
    let mut machines = Vec::new();
    let mut lines = input.ascii_lines();
    loop {
        let Some(button_a_line) = lines.next() else {
            break;
        };
        let button_b_line = lines.next().unwrap();
        let target_line = lines.next().unwrap();
        let _ = lines.next();

        let button_a = parse_button_line(button_a_line);
        let button_b = parse_button_line(button_b_line);
        let target = parse_target_line(target_line);
        machines.push(Machine {
            button_a,
            button_b,
            target,
        });
    }
    machines
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Solution {
    a: i32,
    b: i32,
}

impl Solution {
    fn tokens(self) -> usize {
        let a = usize::try_from(self.a).unwrap();
        let b = usize::try_from(self.b).unwrap();
        3 * a + b
    }
}

fn solve_machine(machine: &Machine) -> Option<Solution> {
    // a*xa + b*xb = xt
    // a*ya + b*yb = yt
    //
    // a = (xt*yb - xb*yt) / (xa*yb - xb*ya)
    // b = (xa*yt - xt*ya) / (xa*yb - xb*ya)

    let xa = machine.button_a.x;
    let ya = machine.button_a.y;
    let xb = machine.button_b.x;
    let yb = machine.button_b.y;
    let xt = machine.target.x;
    let yt = machine.target.y;

    let det = xa * yb - xb * ya;
    assert!(det != 0);

    let is_a_int = (xt * yb - xb * yt) % det == 0;
    let is_b_int = (xa * yt - xt * ya) % det == 0;

    if is_a_int && is_b_int {
        let a = (xt * yb - xb * yt) / det;
        let b = (xa * yt - xt * ya) / det;
        Some(Solution { a, b })
    } else {
        None
    }
}

#[aoc(day13, part1)]
fn part1(input: &[Machine]) -> usize {
    input
        .iter()
        .filter_map(|machine| solve_machine(machine))
        .map(Solution::tokens)
        .sum()
}

#[aoc(day13, part2)]
fn part2(input: &[Machine]) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_machine_test() {
        let machine = Machine {
            button_a: Vector { x: 94, y: 34 },
            button_b: Vector { x: 22, y: 67 },
            target: Vector { x: 8400, y: 5400 },
        };
        let solution = solve_machine(&machine).unwrap();
        assert_eq!(solution.a, 80);
        assert_eq!(solution.b, 40);
    }

    #[test]
    fn solve_machine_no_solution_test() {
        let machine = Machine {
            button_a: Vector { x: 26, y: 66 },
            button_b: Vector { x: 67, y: 21 },
            target: Vector { x: 12748, y: 12176 },
        };
        assert_eq!(solve_machine(&machine), None);
    }
}

example_tests! {
    b"
    Button A: X+94, Y+34
    Button B: X+22, Y+67
    Prize: X=8400, Y=5400

    Button A: X+26, Y+66
    Button B: X+67, Y+21
    Prize: X=12748, Y=12176

    Button A: X+17, Y+86
    Button B: X+84, Y+37
    Prize: X=7870, Y=6450

    Button A: X+69, Y+23
    Button B: X+27, Y+71
    Prize: X=18641, Y=10279
    ",

    part1 => 480,
}
