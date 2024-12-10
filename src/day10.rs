use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::{
    testing::{example_tests, known_input_tests},
    utils::{AsciiUtils, FromGridLike},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
pub struct Map {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl Map {
    fn get(&self, pos: Pos) -> Option<u8> {
        if pos.x < self.width && pos.y < self.height {
            Some(self.data[pos.y * self.width + pos.x])
        } else {
            None
        }
    }

    fn cells(&self) -> impl Iterator<Item = (Pos, u8)> {
        (0..self.height).flat_map(move |y| {
            (0..self.width).map(move |x| (Pos { x, y }, self.get(Pos { x, y }).unwrap()))
        })
    }

    fn neighbors(&self, pos: Pos) -> impl Iterator<Item = (Pos, u8)> {
        [(0, 1), (1, 0), (0, -1), (-1, 0)]
            .iter()
            .filter_map(move |(dx, dy)| {
                Some(Pos {
                    x: pos.x.checked_add_signed(*dx)?,
                    y: pos.y.checked_add_signed(*dy)?,
                })
            })
            .filter_map(|pos| Some((pos, self.get(pos)?)))
    }
}

const TRAIL_START: u8 = b'0';
const TRAIL_END: u8 = b'9';

impl FromGridLike for Map {
    type Cell = u8;
    fn from_cells(data: Vec<Self::Cell>, width: usize, height: usize) -> Self {
        Map {
            width,
            height,
            data,
        }
    }
}

#[aoc_generator(day10)]
pub fn parse(input: &[u8]) -> Map {
    input.grid_like().unwrap().into_grid()
}

#[aoc(day10, part1)]
pub fn part1(input: &Map) -> usize {
    let mut total_score = 0;
    for (start, _) in input.cells().filter(|&(_, c)| c == TRAIL_START) {
        let mut visited = vec![false; input.width * input.height];
        let mut queue: VecDeque<_> = [(start, TRAIL_START)].into();
        let mut ends = Vec::new();
        while let Some((pos, value)) = queue.pop_front() {
            if visited[pos.y * input.width + pos.x] {
                continue;
            }
            visited[pos.y * input.width + pos.x] = true;
            if value == TRAIL_END {
                ends.push(pos);
                continue;
            }
            for (neighbor, neighbor_value) in input.neighbors(pos) {
                if neighbor_value == value + 1 && !visited[neighbor.y * input.width + neighbor.x] {
                    queue.push_back((neighbor, neighbor_value));
                }
            }
        }
        total_score += ends.len();
    }
    total_score
}

fn recursive_trails(map: &Map, pos: Pos, value: u8) -> usize {
    if value == TRAIL_END {
        return 1;
    }
    let mut trails = 0;
    for (neighbor, neighbor_value) in map.neighbors(pos) {
        if neighbor_value == value + 1 {
            trails += recursive_trails(map, neighbor, neighbor_value);
        }
    }
    trails
}

#[aoc(day10, part2)]
pub fn part2(input: &Map) -> usize {
    let mut total_score = 0;
    for (start, _) in input.cells().filter(|&(_, c)| c == TRAIL_START) {
        let trails = recursive_trails(input, start, TRAIL_START);
        total_score += trails;
    }
    total_score
}

#[cfg(test)]
mod tests {
    use super::*;
    use unindent::unindent_bytes;

    #[test]
    fn part1_basic() {
        let input = unindent_bytes(
            b"
            ...0...
            ...1...
            ...2...
            6543456
            7.....7
            8.....8
            9.....9
            ",
        );
        let map = parse(&input);
        assert_eq!(part1(&map), 2);
    }

    #[test]
    fn part1_basic_4() {
        let input = unindent_bytes(
            b"
            ..90..9
            ...1.98
            ...2..7
            6543456
            765.987
            876....
            987....
            ",
        );
        let map = parse(&input);
        assert_eq!(part1(&map), 4);
    }
}

example_tests! {
    b"
    89010123
    78121874
    87430965
    96549874
    45678903
    32019012
    01329801
    10456732
    ",

    part1 => 36,
    part2 => 81,
}

known_input_tests! {
    input: include_bytes!("../input/2024/day10.txt"),
    part1 => 841,
    part2 => 1875,
}
