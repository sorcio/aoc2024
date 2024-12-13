use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

use aoc_utils::{AsciiUtils, FromGridLike, InvalidCharacter, example_tests, known_input_tests};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Heading {
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
pub enum InputCell {
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

impl Pos {
    fn xy(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

#[derive(Debug, Clone)]
pub struct Grid {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
    start: Pos,
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
            start: start.expect("No starting position found"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Step {
    Straight,
    Turn,
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

    fn step_or_turn(&mut self, position: Pos) -> Option<(Step, Pos)> {
        let Pos { x, y, heading } = position;
        let (dx, dy) = match heading {
            Heading::Up => (0, -1),
            Heading::Down => (0, 1),
            Heading::Left => (-1, 0),
            Heading::Right => (1, 0),
        };
        let new_x = x as isize + dx;
        let new_y = y as isize + dy;
        let cell = self.get(new_x, new_y)?;

        Some(match cell {
            Cell::Obstacle => (Step::Turn, Pos {
                x,
                y,
                heading: heading.turn_right(),
            }),
            Cell::Free => (Step::Straight, Pos {
                x: new_x as usize,
                y: new_y as usize,
                heading,
            }),
        })
    }
}

#[aoc_generator(day6)]
pub fn parse(input: &[u8]) -> Grid {
    input.grid_like().unwrap().into_grid()
}

#[aoc(day6, part1)]
pub fn part1(input: &Grid) -> usize {
    let mut grid = input.clone();
    let mut visited_cells = HashSet::new();
    let mut position = grid.start;
    while let Some((_, new_pos)) = grid.step_or_turn(position) {
        position = new_pos;
        let Pos { x, y, .. } = position;
        visited_cells.insert((x, y));
    }
    visited_cells.len()
}

#[aoc(day6, part2)]
pub fn part2(input: &Grid) -> usize {
    let mut grid = input.clone();
    let mut visited_states = HashSet::new();
    let mut visited_cells = HashSet::new();
    let mut position = grid.start;
    let mut new_obstacles = HashSet::new();

    visited_states.insert(position);
    loop {
        visited_cells.insert(position.xy());
        match grid.step_or_turn(position) {
            Some((Step::Straight, new_pos)) => {
                // what if we had hit an obstacle instead?
                if !visited_cells.contains(&new_pos.xy()) && !new_obstacles.contains(&new_pos.xy())
                {
                    let new_obstacle_xy = new_pos.xy();

                    let mut sub_visited_states = HashSet::with_capacity(visited_states.capacity());
                    let mut turned_pos = Pos {
                        heading: position.heading.turn_right(),
                        ..position
                    };

                    // keep moving and see if we end up in a visited state
                    loop {
                        sub_visited_states.insert(turned_pos);
                        match grid.step_or_turn(turned_pos) {
                            Some((Step::Straight, new_pos)) if new_pos.xy() == new_obstacle_xy => {
                                turned_pos = Pos {
                                    heading: turned_pos.heading.turn_right(),
                                    ..turned_pos
                                };
                                continue;
                            }
                            Some((_, new_pos)) if sub_visited_states.contains(&new_pos) => {
                                // found a loop!
                                new_obstacles.insert(new_obstacle_xy);
                                break;
                            }
                            Some((_, new_pos)) => {
                                turned_pos = new_pos;
                            }
                            None => {
                                break;
                            }
                        }
                    }
                }

                // in the end, let's update the position and go ahead as usual
                position = new_pos;
                visited_states.insert(position);
            }
            Some((Step::Turn, new_pos)) => {
                // there is already an obstacle, no point in trying to add one
                position = new_pos;
                visited_states.insert(position);
            }
            None => {
                break;
            }
        }
    }
    new_obstacles.len()
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
    part2 => 1793,
}
