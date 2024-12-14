use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::{example_tests, known_input_tests};

#[derive(Debug, Clone, Copy)]
#[repr(align(2))]
struct Vector {
    x: i8,
    y: i8,
}

impl Vector {
    fn rem_euclid(self, modulo: Vector) -> Self {
        Self {
            x: self.x.rem_euclid(modulo.x),
            y: self.y.rem_euclid(modulo.y),
        }
    }
}

impl std::ops::Mul<i8> for Vector {
    type Output = Self;

    fn mul(self, rhs: i8) -> Self::Output {
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

#[derive(Debug, Clone, Copy)]
#[repr(align(4))]
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
        x: WIDTH as i8,
        y: HEIGHT as i8,
    };
    const ITERATIONS: i8 = 100;

    let border_x: i8 = size.x / 2;
    let border_y: i8 = size.y / 2;

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
    // check if robots draw a christmas tree figure
    // heuristic: is there a row/col that could look like the picture frame?

    let mut iterations_x = 0;
    let mut cols = [0u8; 104];
    debug_assert!(cols.len() >= WIDTH);
    'outer_x: loop {
        for robot in robots {
            let pos = (robot.position.x as i16 + robot.velocity.x as i16 * iterations_x as i16)
                .rem_euclid(WIDTH as _) as usize;
            cols[pos] += 1;
            if cols[pos] >= 32 {
                break 'outer_x;
            }
        }
        iterations_x += 1;
        cols.fill(0);
    }

    let mut iterations_y = 0;
    let mut rows = [0u8; 104];
    debug_assert!(rows.len() >= HEIGHT);
    'outer_y: loop {
        for robot in robots {
            let pos = (robot.position.y as i16 + robot.velocity.y as i16 * iterations_y as i16)
                .rem_euclid(HEIGHT as _) as usize;
            rows[pos] += 1;
            if rows[pos] >= 32 {
                break 'outer_y;
            }
        }
        iterations_y += 1;
        rows.fill(0);
    }

    // iterations = iterations_x + n * WIDTH = iterations_y + m * HEIGHT

    (1..WIDTH)
        .map(|i| (iterations_x as usize + i * WIDTH) - iterations_y as usize)
        .filter(|n| *n % HEIGHT == 0)
        .next()
        .unwrap()
        + iterations_y as usize
}

fn part2_impl_autovect<const WIDTH: usize, const HEIGHT: usize>(robots: &[Robot]) -> usize {
    let mut position_x = [0u8; 500];
    let mut position_y = [0u8; 500];
    let mut velocity_x = [0u8; 500];
    let mut velocity_y = [0u8; 500];

    for (i, robot) in robots.iter().enumerate() {
        position_x[i] = (robot.position.x as i16 + WIDTH as i16).rem_euclid(WIDTH as _) as _;
        position_y[i] = (robot.position.y as i16 + HEIGHT as i16).rem_euclid(HEIGHT as _) as _;
        velocity_x[i] = (robot.velocity.x as i16 + WIDTH as i16).rem_euclid(WIDTH as _) as _;
        velocity_y[i] = (robot.velocity.y as i16 + HEIGHT as i16).rem_euclid(HEIGHT as _) as _;
    }

    // check if robots draw a christmas tree figure
    // heuristic: is there a row/col that could look like the picture frame?

    let mut iterations_x = 0;
    let mut cols = [0u8; 104];
    debug_assert!(cols.len() >= WIDTH);
    'outer_x: loop {
        for (pos, vel) in position_x.iter_mut().zip(velocity_x) {
            *pos = (*pos + vel).rem_euclid(WIDTH as _).try_into().unwrap();
        }
        iterations_x += 1;
        for pos in position_x {
            cols[pos as usize] += 1;
            if cols[pos as usize] >= 32 {
                break 'outer_x;
            }
        }
        cols.fill(0);
    }

    let mut iterations_y = 0;
    let mut rows = [0u8; 104];
    debug_assert!(rows.len() >= HEIGHT);
    'outer_y: loop {
        if rows.iter().any(|&x| x >= 32) {
            break;
        }
        for (pos, vel) in position_y.iter_mut().zip(velocity_y) {
            *pos = (*pos + vel).rem_euclid(HEIGHT as _).try_into().unwrap();
        }
        iterations_y += 1;
        for pos in position_y {
            rows[pos as usize] += 1;
            if rows[pos as usize] >= 32 {
                break 'outer_y;
            }
        }
        rows.fill(0);
    }

    // iterations = iterations_x + n * WIDTH = iterations_y + m * HEIGHT

    (1..WIDTH)
        .map(|i| (iterations_x + i * WIDTH) - iterations_y)
        .filter(|n| *n % HEIGHT == 0)
        .next()
        .unwrap()
        + iterations_y
}

#[aoc(day14, part2, slow)]
pub fn part2_slow(input: &[Robot]) -> usize {
    part2_impl::<101, 103>(input)
}

#[aoc(day14, part2)]
pub fn part2(input: &[Robot]) -> usize {
    part2_impl_autovect::<101, 103>(input)
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
