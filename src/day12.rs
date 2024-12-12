use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::{
    testing::{example_tests, known_input_tests},
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

    fn neighbors_signed(
        &self,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = (isize, isize, isize, isize, Option<u8>)> + '_ {
        [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)]
            .iter()
            .map(move |&(dx, dy)| {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 || ny >= 0 {
                    self.get(nx as usize, ny as usize)
                        .map(|cell| (nx, ny, dx, dy, Some(cell)))
                        .unwrap_or((nx, ny, dx, dy, None))
                } else {
                    (nx, ny, dx, dy, None)
                }
            })
    }
}

#[aoc_generator(day12)]
fn parse(input: &[u8]) -> InputGrid {
    input.grid_like().unwrap().into_grid()
}

#[aoc(day12, part1)]
fn part1(input: &InputGrid) -> usize {
    #[derive(Debug, Default)]
    struct Region {
        area: usize,
        perimeter: usize,
    }

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
fn part2(input: &InputGrid) -> usize {
    #[derive(Debug, Default)]
    struct Region {
        area: usize,
        sides: usize,
        horizontal_fences: Vec<(isize, isize, isize)>,
        vertical_fences: Vec<(isize, isize, isize)>,
    }

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
        let region_id = regions.len() - 1;
        let region = &mut regions.last_mut().unwrap();
        region_queue.push_back((x, y));
        while let Some((x, y)) = region_queue.pop_front() {
            if regions_map[y * input.width + x].is_some() {
                continue;
            }
            regions_map[y * input.width + x] = Some(region_id);
            let cell = input.get(x, y).unwrap();

            for (nx, ny, dx, dy, ncell) in input.neighbors_signed(x, y) {
                if ncell == Some(cell) {
                    region_queue.push_back((nx as _, ny as _));
                } else {
                    if ncell.is_some() {
                        extra_queue.push_back((nx as _, ny as _));
                    }

                    if nx == x as isize {
                        region
                            .horizontal_fences
                            .push((x as isize, y as isize * 2 + dy, dy));
                    } else {
                        region
                            .vertical_fences
                            .push((x as isize * 2 + dx, y as isize, dx));
                    }
                }
            }

            region.area += 1;
        }

        // compact the fences and determine the sides
        region.vertical_fences.sort_unstable();
        let mut vertical_sides = 1;
        let mut last_fence = None;
        for (fence_x, fence_y, fence_direction) in region.vertical_fences.drain(..) {
            if let Some((last_x, last_y, last_direction)) = last_fence {
                if last_x == fence_x && last_y + 1 == fence_y && last_direction == fence_direction {
                } else {
                    vertical_sides += 1;
                }
            }
            last_fence = Some((fence_x, fence_y, fence_direction));
        }

        region
            .horizontal_fences
            .sort_unstable_by_key(|&(x, y, d)| (y, x, d));
        let mut horizontal_sides = 1;
        let mut last_fence = None;
        for (fence_x, fence_y, fence_direction) in region.horizontal_fences.drain(..) {
            if let Some((last_x, last_y, last_direction)) = last_fence {
                if last_y == fence_y && last_x + 1 == fence_x && last_direction == fence_direction {
                } else {
                    horizontal_sides += 1;
                }
            }
            last_fence = Some((fence_x, fence_y, fence_direction));
        }

        region.sides = vertical_sides + horizontal_sides;
    }

    regions
        .into_iter()
        .map(|region| region.area * region.sides)
        .sum()
}

#[cfg(test)]
mod test {
    use unindent::unindent_bytes;

    use super::*;

    #[test]
    fn sides_test() {
        let input = unindent_bytes(
            b"
            AAAA
            BBCD
            BBCC
            EEEC
            ",
        );
        let grid = parse(&input);
        assert_eq!(part2(&grid), 80);
    }

    #[test]
    fn sides_complex_test() {
        let input = unindent_bytes(
            b"
            EEEEE
            EXXXX
            EEEEE
            EXXXX
            EEEEE
            ",
        );
        let grid = parse(&input);
        assert_eq!(part2(&grid), 236);
    }

    #[test]
    fn sides_more_complex_test() {
        let input = unindent_bytes(
            b"
            AAAAAA
            AAABBA
            AAABBA
            ABBAAA
            ABBAAA
            AAAAAA
            ",
        );
        let grid = parse(&input);
        assert_eq!(part2(&grid), 368);
    }
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
    part2 => 1206,
}

known_input_tests! {
    input: include_bytes!("../input/2024/day12.txt"),
    part1 => 1449902,
    part2 => 908042,
}
