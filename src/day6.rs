use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::{
    testing::{example_tests, known_input_tests},
    utils::{AsciiUtils, FromGridLike, InvalidCharacter},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Heading {
    Up,
    Down,
    Left,
    Right,
}

impl Heading {
    fn turn_right(self) -> Self {
        match self {
            Heading::Up => Heading::Right,
            Heading::Down => Heading::Left,
            Heading::Left => Heading::Up,
            Heading::Right => Heading::Down,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InputCell {
    Free,
    Obstacle,
    Start(Heading),
}

impl TryFrom<u8> for InputCell {
    type Error = InvalidCharacter;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'.' => Ok(InputCell::Free),
            b'#' => Ok(InputCell::Obstacle),
            b'^' => Ok(InputCell::Start(Heading::Up)),
            _ => Err(InvalidCharacter(value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Free,
    Obstacle,
}

impl From<InputCell> for Cell {
    fn from(cell: InputCell) -> Self {
        match cell {
            InputCell::Free => Cell::Free,
            InputCell::Obstacle => Cell::Obstacle,
            InputCell::Start(_) => Cell::Free,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
    heading: Heading,
}

#[derive(Debug, Clone)]
struct Grid {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
    position: Pos,
}

impl FromGridLike for Grid {
    type Cell = InputCell;
    fn from_cells(input_cells: Vec<Self::Cell>, width: usize, height: usize) -> Self {
        let mut start = None;
        let cells = input_cells
            .into_iter()
            .enumerate()
            .inspect(|(i, cell)| {
                if let InputCell::Start(heading) = *cell {
                    if start.is_some() {
                        panic!("Multiple starting positions found");
                    }
                    let x = i % width;
                    let y = i / width;
                    start = Some(Pos { x, y, heading });
                }
            })
            .map(|(_, cell)| cell.into())
            .collect();
        Self {
            cells,
            width,
            height,
            position: start.expect("No starting position found"),
        }
    }
}

impl Grid {
    fn get(&self, x: isize, y: isize) -> Option<Cell> {
        if x < 0 || x >= self.width as isize || y < 0 || y >= self.height as isize {
            return None;
        }
        let x = x as usize;
        let y = y as usize;
        self.cells.get(y * self.width + x).copied()
    }

    fn move_straight(&mut self) -> bool {
        let Pos { x, y, heading } = self.position;
        let (dx, dy) = match heading {
            Heading::Up => (0, -1),
            Heading::Down => (0, 1),
            Heading::Left => (-1, 0),
            Heading::Right => (1, 0),
        };
        let new_x = x as isize + dx;
        let new_y = y as isize + dy;
        let Some(cell) = self.get(new_x, new_y) else {
            return false;
        };

        let new_pos = match cell {
            Cell::Obstacle => Pos {
                x,
                y,
                heading: heading.turn_right(),
            },
            Cell::Free => Pos {
                x: new_x as usize,
                y: new_y as usize,
                heading,
            },
        };
        self.position = new_pos;
        return true;
    }
}

#[aoc_generator(day6)]
fn parse(input: &[u8]) -> Grid {
    input.grid_like().unwrap().into_grid()
}

#[aoc(day6, part1)]
fn part1(input: &Grid) -> usize {
    let mut grid = input.clone();
    let mut visited_cells = HashSet::new();
    while grid.move_straight() {
        let Pos { x, y, .. } = grid.position;
        visited_cells.insert((x, y));
    }
    visited_cells.len()
}

#[aoc(day6, part2)]
fn part2(input: &Grid) -> usize {
    let mut grid = input.clone();
    let mut visited_cells = HashSet::new();
    let mut visited_positions = HashSet::new();
    while grid.move_straight() {
        let Pos { x, y, .. } = grid.position;
        visited_cells.insert((x, y));
        visited_positions.insert(grid.position);
    }
    visited_cells.len()
}

example_tests! {
    b"
    ....#.....
    .........#
    ..........
    ..#.......
    .......#..
    ..........
    .#..^.....
    ........#.
    #.........
    ......#...
    ",

    part1 => 41,
    part2 => 6,
}

known_input_tests! {
    input: include_bytes!("../input/2024/day6.txt"),
    part1 => 5067,
}
