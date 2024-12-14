use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::{example_tests, known_input_tests};

#[derive(Debug, Clone, Copy)]
struct Vector {
    x: i32,
    y: i32,
}

impl Vector {
    fn rem_euclid(self, modulo: Vector) -> Self {
        Self {
            x: self.x.rem_euclid(modulo.x),
            y: self.y.rem_euclid(modulo.y),
        }
    }
}

impl std::ops::Mul<i32> for Vector {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Add<Vector> for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl FromStr for Vector {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        Ok(Self {
            x: parts.next().unwrap().parse()?,
            y: parts.next().unwrap().parse()?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Robot {
    position: Vector,
    velocity: Vector,
}

#[aoc_generator(day14)]
pub fn parse(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let position_part = &parts.next().unwrap()[2..];
            let velocity_part = &parts.next().unwrap()[2..];
            let position = position_part.parse().unwrap();
            let velocity = velocity_part.parse().unwrap();
            Robot { position, velocity }
        })
        .collect()
}

fn part1_impl<const WIDTH: usize, const HEIGHT: usize>(robots: &[Robot]) -> usize {
    let size = Vector {
        x: WIDTH as i32,
        y: HEIGHT as i32,
    };
    const ITERATIONS: i32 = 100;

    let border_x: i32 = size.x / 2;
    let border_y: i32 = size.y / 2;

    let mut quadrant_nw = 0;
    let mut quadrant_ne = 0;
    let mut quadrant_sw = 0;
    let mut quadrant_se = 0;

    for robot in robots {
        let displacement = robot.velocity * ITERATIONS;
        let new_position = (robot.position + displacement).rem_euclid(size);

        let Vector {
            x: position_x,
            y: position_y,
        } = new_position;

        // robots that are on the border are not counted
        if position_x < border_x && position_y < border_y {
            quadrant_nw += 1;
        } else if position_x > border_x && position_y < border_y {
            quadrant_ne += 1;
        } else if position_x < border_x && position_y > border_y {
            quadrant_sw += 1;
        } else if position_x > border_x && position_y > border_y {
            quadrant_se += 1;
        }
    }
    debug_assert_ne!(quadrant_ne, 0);
    debug_assert_ne!(quadrant_nw, 0);
    debug_assert_ne!(quadrant_sw, 0);
    debug_assert_ne!(quadrant_se, 0);

    quadrant_nw * quadrant_ne * quadrant_sw * quadrant_se
}

#[cfg(test)]
fn part1_small_example(input: &[Robot]) -> usize {
    part1_impl::<11, 7>(input)
}

#[aoc(day14, part1)]
pub fn part1(input: &[Robot]) -> usize {
    part1_impl::<101, 103>(input)
}

fn part2_impl<const WIDTH: usize, const HEIGHT: usize>(robots: &[Robot]) -> usize {
    let size = Vector {
        x: WIDTH as i32,
        y: HEIGHT as i32,
    };

    let mut iterations = 0;
    let mut rows = [0; 128];
    let mut cols = [0; 128];
    loop {
        // check if robots draw a christmas tree figure
        // heuristic: is there a row/col that could look like the picture frame?
        cols.fill(0);
        for robot in robots {
            let x = (robot.position.x + robot.velocity.x * iterations).rem_euclid(size.x);
            cols[x as usize] += 1;
        }
        if cols.iter().any(|&x| x >= 32) {
            rows.fill(0);
            for robot in robots {
                let y = (robot.position.y + robot.velocity.y * iterations).rem_euclid(size.y);
                rows[y as usize] += 1;
            }
            if rows.iter().any(|&x| x >= 32) {
                break;
            }
        }

        iterations += 1;
    }
    iterations as _
}

#[aoc(day14, part2)]
pub fn part2(input: &[Robot]) -> usize {
    part2_impl::<101, 103>(input)
}

example_tests! {
    "
    p=0,4 v=3,-3
    p=6,3 v=-1,-3
    p=10,3 v=-1,2
    p=2,0 v=2,-1
    p=0,0 v=1,3
    p=3,0 v=-2,-2
    p=7,6 v=-1,-3
    p=3,0 v=-1,-2
    p=9,3 v=2,3
    p=7,3 v=-1,2
    p=2,4 v=2,-3
    p=9,5 v=-3,-3
    ",

    part1_small_example => 12,

    // this puzzle has no part 2 example
}

known_input_tests! {
    input: include_str!("../input/2024/day14.txt"),
    part1 => 226236192,
    part2 => 8168,
}
