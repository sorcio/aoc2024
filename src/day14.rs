use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::{example_tests, known_input_tests};

pub struct Robots {
    position_x: Vec<u8>,
    position_y: Vec<u8>,
    velocity_x: Vec<u8>,
    velocity_y: Vec<u8>,
}

impl Robots {
    fn iter(&self) -> impl Iterator<Item = (u8, u8, u8, u8)> + '_ {
        self.position_x
            .iter()
            .zip(&self.position_y)
            .zip(&self.velocity_x)
            .zip(&self.velocity_y)
            .map(|(((&px, &py), &vx), &vy)| (px, py, vx, vy))
    }
}

fn parse_vector<const WIDTH: u8, const HEIGHT: u8>(s: &str) -> (u8, u8) {
    let mut parts = s.split(',');
    let x: i8 = parts.next().unwrap().parse().unwrap();
    let y: i8 = parts.next().unwrap().parse().unwrap();
    (
        (x as i16 + WIDTH as i16).rem_euclid(WIDTH as _) as u8,
        (y as i16 + HEIGHT as i16).rem_euclid(HEIGHT as _) as u8,
    )
}

pub fn parse_impl<const WIDTH: u8, const HEIGHT: u8>(input: &str) -> Robots {
    let mut position_x = Vec::new();
    let mut position_y = Vec::new();
    let mut velocity_x = Vec::new();
    let mut velocity_y = Vec::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let position_part = &parts.next().unwrap()[2..];
        let velocity_part = &parts.next().unwrap()[2..];
        let (px, py) = parse_vector::<WIDTH, HEIGHT>(position_part);
        let (vx, vy) = parse_vector::<WIDTH, HEIGHT>(velocity_part);
        position_x.push(px);
        position_y.push(py);
        velocity_x.push(vx);
        velocity_y.push(vy);
    }

    Robots {
        position_x,
        position_y,
        velocity_x,
        velocity_y,
    }
}

#[aoc_generator(day14)]
pub fn parse(input: &str) -> Robots {
    parse_impl::<101, 103>(input)
}

#[cfg(test)]
fn parse_small_example(input: &str) -> Robots {
    parse_impl::<11, 7>(input)
}

fn part1_impl<const WIDTH: u8, const HEIGHT: u8>(robots: &Robots) -> usize {
    const ITERATIONS: u8 = 100;

    let border_x: u8 = WIDTH / 2;
    let border_y: u8 = HEIGHT / 2;

    let mut quadrant_nw = 0;
    let mut quadrant_ne = 0;
    let mut quadrant_sw = 0;
    let mut quadrant_se = 0;

    for (px, py, vx, vy) in robots.iter() {
        let position_x = ((px as i32 + vx as i32 * ITERATIONS as i32) % WIDTH as i32) as u8;
        let position_y = ((py as i32 + vy as i32 * ITERATIONS as i32) % HEIGHT as i32) as u8;

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
fn part1_small_example(input: &Robots) -> usize {
    part1_impl::<11, 7>(input)
}

#[aoc(day14, part1)]
pub fn part1(input: &Robots) -> usize {
    part1_impl::<101, 103>(input)
}

fn part2_impl<const WIDTH: usize, const HEIGHT: usize>(robots: &Robots) -> usize {
    // check if robots draw a christmas tree figure
    // heuristic: is there a row/col that could look like the picture frame?

    let mut iterations_x = 0;
    let mut cols = [0u8; 104];
    debug_assert!(cols.len() >= WIDTH);
    'outer_x: loop {
        for (px, _, vx, _) in robots.iter() {
            let pos = (px as i16 + vx as i16 * iterations_x as i16).rem_euclid(WIDTH as _) as usize;
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
        for (_, py, _, vy) in robots.iter() {
            let pos =
                (py as i16 + vy as i16 * iterations_y as i16).rem_euclid(HEIGHT as _) as usize;
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
        .find(|n| *n % HEIGHT == 0)
        .unwrap()
        + iterations_y as usize
}

fn part2_impl_autovect<const WIDTH: usize, const HEIGHT: usize>(robots: &Robots) -> usize {
    let mut position_x = robots.position_x.clone();
    let mut position_y = robots.position_y.clone();
    let velocity_x = &robots.velocity_x;
    let velocity_y = &robots.velocity_y;

    // check if robots draw a christmas tree figure
    // heuristic: is there a row/col that could look like the picture frame?

    let mut iterations_x = 0;
    let mut cols = [0u8; 104];
    debug_assert!(cols.len() >= WIDTH);
    'outer_x: loop {
        for (pos, vel) in position_x.iter_mut().zip(velocity_x) {
            *pos = (*pos + vel).rem_euclid(WIDTH as _);
        }
        iterations_x += 1;
        for &pos in &position_x {
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
            *pos = (*pos + vel).rem_euclid(HEIGHT as _);
        }
        iterations_y += 1;
        for &pos in &position_y {
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
        .find(|n| *n % HEIGHT == 0)
        .unwrap()
        + iterations_y
}

#[aoc(day14, part2, slow)]
pub fn part2_slow(input: &Robots) -> usize {
    part2_impl::<101, 103>(input)
}

#[aoc(day14, part2)]
pub fn part2(input: &Robots) -> usize {
    part2_impl_autovect::<101, 103>(input)
}

example_tests! {
    parser: crate::day14::parse_small_example,
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
    part2_slow => 8168,
}
