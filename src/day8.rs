use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

use aoc_utils::{
    AsciiUtils, FromGridLike, InvalidCharacter, SliceUtils, example_tests, known_input_tests,
};

pub enum Cell {
    Empty,
    Antenna(u8),
}

impl TryFrom<u8> for Cell {
    type Error = InvalidCharacter;
    fn try_from(c: u8) -> Result<Self, InvalidCharacter> {
        match c {
            b'.' => Ok(Cell::Empty),
            b'0'..=b'9' | b'a'..=b'z' | b'A'..=b'Z' => Ok(Cell::Antenna(c)),
            _ => Err(InvalidCharacter(c)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i16,
    y: i16,
}

impl Position {
    fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }
}

impl std::ops::Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub<Position> for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

pub struct Map {
    antennas: [Vec<Position>; u8::MAX as _],
    width: usize,
    height: usize,
}

impl Map {
    fn contains(&self, pos: Position) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x < self.width as _ && pos.y < self.height as _
    }
}

impl FromGridLike for Map {
    type Cell = Cell;
    fn from_cells(cells: Vec<Self::Cell>, width: usize, height: usize) -> Self {
        let mut antennas = [(); u8::MAX as _].map(|_| Vec::new());
        for y in 0..height {
            for x in 0..width {
                if let Cell::Antenna(c) = cells[y * width + x] {
                    antennas[c as usize].push(Position::new(x as _, y as _));
                }
            }
        }
        Self {
            antennas,
            width,
            height,
        }
    }
}

#[aoc_generator(day8)]
pub fn parse(input: &[u8]) -> Map {
    input.grid_like().unwrap().into_grid()
}

#[aoc(day8, part1)]
fn part1(input: &Map) -> usize {
    let mut antinodes = HashSet::new();
    for antennas in &input.antennas {
        for (&a, &b) in antennas.pairs() {
            let distance = b - a;
            let pos1 = a - distance;
            antinodes.insert(pos1);
            let pos2 = b + distance;
            antinodes.insert(pos2);
        }
    }
    antinodes.iter().filter(|&&pos| input.contains(pos)).count()
}

#[aoc(day8, part2)]
fn part2(input: &Map) -> usize {
    let mut antinodes = HashSet::new();
    for antennas in &input.antennas {
        for (&a, &b) in antennas.pairs() {
            let distance = b - a;
            let mut pos;

            pos = a;
            while input.contains(pos) {
                antinodes.insert(pos);
                pos = pos - distance;
            }
            pos = b;
            while input.contains(pos) {
                antinodes.insert(pos);
                pos = pos + distance;
            }
        }
    }
    antinodes.len()
}

example_tests! {
    b"
    ............
    ........0...
    .....0......
    .......0....
    ....0.......
    ......A.....
    ............
    ............
    ........A...
    .........A..
    ............
    ............
    ",
    part1 => 14,
    part2 => 34,
}

known_input_tests! {
    input: include_bytes!("../input/2024/day8.txt"),
    part1 => 276,
    part2 => 991,
}
