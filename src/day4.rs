use aoc_runner_derive::{aoc, aoc_generator};

use crate::{
    testing::{example_tests, known_input_tests},
    utils::{AsciiUtils, FromGridLike, InvalidCharacter},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Letter {
    X,
    M,
    A,
    S,
}

const XMAS: [Letter; 4] = [Letter::X, Letter::M, Letter::A, Letter::S];

impl TryFrom<u8> for Letter {
    type Error = InvalidCharacter;
    fn try_from(c: u8) -> Result<Self, InvalidCharacter> {
        match c {
            b'X' => Ok(Letter::X),
            b'M' => Ok(Letter::M),
            b'A' => Ok(Letter::A),
            b'S' => Ok(Letter::S),
            _ => Err(InvalidCharacter(c)),
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    cells: Vec<Letter>,
    width: usize,
    height: usize,
}

impl FromGridLike for Grid {
    type Cell = Letter;
    fn from_cells(cells: Vec<Self::Cell>, width: usize, height: usize) -> Self {
        Self {
            cells,
            width,
            height,
        }
    }
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> Option<Letter> {
        self.cells.get(y * self.width + x).copied()
    }

    fn find_xmas_in_all_directions(&self, x: usize, y: usize) -> usize {
        const DIRECTIONS: [(isize, isize); 8] = [
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
        ];

        let mut count = 0;
        'dir: for (dx, dy) in DIRECTIONS {
            let mut pos = (x as isize, y as isize);
            for letter in XMAS {
                if pos.0 < 0
                    || pos.0 >= self.width as isize
                    || pos.1 < 0
                    || pos.1 >= self.height as isize
                {
                    continue 'dir;
                }
                if self.get(pos.0 as usize, pos.1 as usize) == Some(letter) {
                    pos.0 += dx;
                    pos.1 += dy;
                } else {
                    continue 'dir;
                }
            }
            count += 1;
        }
        count
    }

    fn is_cross_mas(&self, x: usize, y: usize) -> bool {
        if x < 1 || y < 1 || x >= self.width - 1 || y >= self.height - 1 {
            return false;
        }

        if self.get(x, y) != Some(Letter::A) {
            return false;
        }
        let down_right = (self.get(x - 1, y - 1) == Some(Letter::M)
            && self.get(x + 1, y + 1) == Some(Letter::S))
            || (self.get(x - 1, y - 1) == Some(Letter::S)
                && self.get(x + 1, y + 1) == Some(Letter::M));

        let down_left = (self.get(x + 1, y - 1) == Some(Letter::M)
            && self.get(x - 1, y + 1) == Some(Letter::S))
            || (self.get(x + 1, y - 1) == Some(Letter::S)
                && self.get(x - 1, y + 1) == Some(Letter::M));

        down_right && down_left
    }
}

#[aoc_generator(day4)]
fn parse(input: &[u8]) -> Grid {
    input.grid_like().unwrap().into_grid()
}

#[aoc(day4, part1)]
fn part1(input: &Grid) -> usize {
    let mut total = 0;
    for y in 0..input.height {
        for x in 0..input.width {
            total += input.find_xmas_in_all_directions(x, y);
        }
    }
    total
}

#[aoc(day4, part2)]
fn part2(input: &Grid) -> usize {
    let mut total = 0;
    for y in 0..input.height {
        for x in 0..input.width {
            if input.is_cross_mas(x, y) {
                total += 1;
            }
        }
    }
    total
}

example_tests! {
    b"
    MMMSXXMASM
    MSAMXMSMSA
    AMXSXMAAMM
    MSAMASMSMX
    XMASAMXAMM
    XXAMMXXAMA
    SMSMSASXSS
    SAXAMASAAA
    MAMMMXMMMM
    MXMXAXMASX
    ",
    part1 => 18,
    part2 => 9,
}

known_input_tests! {
    input: include_bytes!("../input/2024/day4.txt"),
    part1 => 2662,
    part2 => 2034,
}
