use std::{collections::VecDeque, num::ParseIntError, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::{example_tests, known_input_tests};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    x: u8,
    y: u8,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl FromStr for Position {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let x = parts.next().unwrap().parse()?;
        let y = parts.next().unwrap().parse()?;
        Ok(Position { x, y })
    }
}

impl Position {
    fn step(self, direction: Direction) -> Option<Self> {
        Some(match direction {
            Direction::Up => Position {
                x: self.x,
                y: self.y.checked_sub(1)?,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y.checked_add(1)?,
            },
            Direction::Left => Position {
                x: self.x.checked_sub(1)?,
                y: self.y,
            },
            Direction::Right => Position {
                x: self.x.checked_add(1)?,
                y: self.y,
            },
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const ALL: [Self; 4] = [Self::Up, Self::Down, Self::Left, Self::Right];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Free,
    Obstacle(u32),
}

struct Grid {
    width: u8,
    height: u8,
    data: Vec<Tile>,
}

impl Grid {
    fn new(width: u8, height: u8) -> Self {
        Self {
            width,
            height,
            data: vec![Tile::Free; width as usize * height as usize],
        }
    }

    fn get(&self, pos: Position) -> Option<Tile> {
        if pos.x >= self.width || pos.y >= self.height {
            None
        } else {
            Some(self.data[pos.y as usize * self.width as usize + pos.x as usize])
        }
    }

    fn set(&mut self, pos: Position, tile: Tile) {
        debug_assert!(pos.x < self.width);
        debug_assert!(pos.y < self.height);
        self.data[pos.y as usize * self.width as usize + pos.x as usize] = tile;
    }

    fn set_obstacles(&mut self, obstacles: impl IntoIterator<Item = Position>) {
        for (i, pos) in obstacles.into_iter().enumerate() {
            self.set(pos, Tile::Obstacle(i.try_into().unwrap()));
        }
    }

    fn step(&self, pos: Position, direction: Direction, max_obstacle: u32) -> Option<Position> {
        let new_pos = pos.step(direction)?;
        match self.get(new_pos) {
            Some(Tile::Free) => Some(new_pos),
            Some(Tile::Obstacle(i)) if i > max_obstacle => Some(new_pos),
            _ => None,
        }
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get(Position { x, y }).unwrap() {
                    Tile::Free => write!(f, ".")?,
                    Tile::Obstacle(_) => write!(f, "#")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

struct VisitedGrid {
    data: Vec<bool>,
    width: u8,
}

impl VisitedGrid {
    fn from_grid(grid: &Grid) -> Self {
        Self {
            data: vec![false; grid.width as usize * grid.height as usize],
            width: grid.width,
        }
    }

    fn insert(&mut self, pos: Position) -> bool {
        let idx = pos.y as usize * self.width as usize + pos.x as usize;
        let visited = self.data[idx];
        self.data[idx] = true;
        !visited
    }
}

#[aoc_generator(day18)]
fn parse(input: &str) -> Vec<Position> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn solve(grid: &Grid, start: Position, end: Position, max_obstacle: u32) -> Option<usize> {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct State {
        pos: Position,
        steps: usize,
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Ord for State {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.steps.cmp(&other.steps)
        }
    }

    impl State {
        fn initial(pos: Position) -> Self {
            Self { pos, steps: 0 }
        }

        fn move_one(&self, new_pos: Position) -> Self {
            Self {
                pos: new_pos,
                steps: self.steps + 1,
            }
        }
    }

    let mut queue: VecDeque<_> = [State::initial(start)].into();
    // let mut visited = std::collections::HashSet::new();
    let mut visited = VisitedGrid::from_grid(grid);

    while let Some(state) = queue.pop_front() {
        if state.pos == end {
            return Some(state.steps);
        }

        for direction in Direction::ALL {
            if let Some(new_pos) = grid.step(state.pos, direction, max_obstacle) {
                if visited.insert(new_pos) {
                    queue.push_back(state.move_one(new_pos));
                }
            }
        }
    }

    None
}

#[cfg(test)]
fn part1_small(input: &[Position]) -> usize {
    let obstacles = &input[..12];
    let mut grid = Grid::new(7, 7);
    grid.set_obstacles(obstacles.iter().copied());
    solve(&grid, Position { x: 0, y: 0 }, Position { x: 6, y: 6 }, 12).unwrap()
}

#[aoc(day18, part1)]
pub fn part1(input: &[Position]) -> usize {
    let obstacles = &input[..1024];
    let mut grid = Grid::new(71, 71);
    grid.set_obstacles(obstacles.iter().copied());
    solve(
        &grid,
        Position { x: 0, y: 0 },
        Position { x: 70, y: 70 },
        1024,
    )
    .unwrap()
}

fn part2_solve(obstacles: &[Position], start: Position, end: Position, skip: usize) -> Position {
    let width = start.x.max(end.x) + 1;
    let height = start.y.max(end.y) + 1;

    let mut grid = Grid::new(width, height);
    grid.set_obstacles(obstacles.iter().copied());

    let mut left = skip;
    let mut right = obstacles.len() - 1;
    while left < right {
        let i = (left + right) / 2;
        if solve(&grid, start, end, i as _).is_some() {
            left = i + 1;
        } else {
            right = i - 1;
        }
    }
    debug_assert!(left == right);
    obstacles[left]
}

#[aoc(day18, part2)]
pub fn part2(input: &[Position]) -> Position {
    let start = Position { x: 0, y: 0 };
    let end = Position { x: 70, y: 70 };
    part2_solve(input, start, end, 1024)
}

#[cfg(test)]
pub fn part2_small(input: &[Position]) -> Position {
    let start = Position { x: 0, y: 0 };
    let end = Position { x: 6, y: 6 };
    part2_solve(input, start, end, 12)
}

example_tests! {
    "5,4\n4,2\n4,5\n3,0\n2,1\n6,3\n2,4\n1,5\n0,6\n3,3\n2,6\n5,1\n1,2\n5,5\n2,5\n6,5\n1,4\n0,4\n6,4\n1,1\n6,1\n1,0\n0,5\n1,6\n2,0",
    part1_small => 22,
    part2_small => crate::day18::Position { x: 6, y: 1 },
}

known_input_tests! {
    input: include_str!("../input/2024/day18.txt"),
    part1 => 506,
    part2 => crate::day18::Position { x: 62, y: 6 },
}
