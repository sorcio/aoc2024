use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::{
    testing::example_tests,
    utils::{AsciiUtils, FromGridLike},
};

struct InputGrid {
    cells: Vec<u8>,
    width: usize,
    height: usize,
}

impl FromGridLike for InputGrid {
    type Cell = u8;

    fn from_cells(cells: Vec<Self::Cell>, width: usize, height: usize) -> Self {
        Self {
            cells,
            width,
            height,
        }
    }
}

impl InputGrid {
    fn get(&self, x: usize, y: usize) -> Option<u8> {
        if x >= self.width || y >= self.height {
            return None;
        }
        self.cells.get(y * self.width + x).copied()
    }

    fn neighbors(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize, u8)> + '_ {
        [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)]
            .iter()
            .filter_map(move |&(dx, dy)| {
                let nx = x.checked_add_signed(dx)?;
                let ny = y.checked_add_signed(dy)?;
                self.get(nx, ny).map(|cell| (nx, ny, cell))
            })
    }
}

#[derive(Debug, Default)]
struct Region {
    area: usize,
    perimeter: usize,
}

#[aoc_generator(day12)]
fn parse(input: &[u8]) -> InputGrid {
    input.grid_like().unwrap().into_grid()
}

#[aoc(day12, part1)]
fn part1(input: &InputGrid) -> usize {
    let mut region_queue = VecDeque::new();
    let mut extra_queue = VecDeque::new();
    let mut regions: Vec<Region> = Vec::new();
    let mut regions_map = vec![None; input.cells.len()];

    extra_queue.push_back((0, 0));
    while let Some((x, y)) = extra_queue.pop_front() {
        if regions_map[y * input.width + x].is_some() {
            continue;
        }
        regions.push(Region::default());
        region_queue.push_back((x, y));
        while let Some((x, y)) = region_queue.pop_front() {
            if regions_map[y * input.width + x].is_some() {
                continue;
            }
            regions_map[y * input.width + x] = Some(regions.len());
            let cell = input.get(x, y).unwrap();
            let mut fence_count = 4;
            for (nx, ny, ncell) in input.neighbors(x, y) {
                if ncell == cell {
                    region_queue.push_back((nx, ny));
                    fence_count -= 1;
                } else {
                    extra_queue.push_back((nx, ny));
                }
            }
            let region = regions.last_mut().unwrap();
            region.area += 1;
            region.perimeter += fence_count;
        }
    }

    regions
        .into_iter()
        .map(|region| region.area * region.perimeter)
        .sum()
}

#[aoc(day12, part2)]
fn part2(input: &InputGrid) -> String {
    todo!()
}

example_tests! {
    b"
    RRRRIICCFF
    RRRRIICCCF
    VVRRRCCFFF
    VVRCCCJFFF
    VVVVCJJCFE
    VVIVCCJJEE
    VVIIICJJEE
    MIIIIIJJEE
    MIIISIJEEE
    MMMISSJEEE
    ",
    part1 => 1930,
}
