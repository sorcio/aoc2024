use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::{AsciiUtils, FromGridLike, InvalidCharacter, example_tests};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<u8> for Direction {
    type Error = InvalidCharacter;

    fn try_from(value: u8) -> Result<Self, InvalidCharacter> {
        match value {
            b'^' => Ok(Direction::Up),
            b'v' => Ok(Direction::Down),
            b'<' => Ok(Direction::Left),
            b'>' => Ok(Direction::Right),
            _ => Err(InvalidCharacter(value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: u8,
    y: u8,
}

impl std::ops::Add<Direction> for Position {
    type Output = Position;

    fn add(mut self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Up => {
                self.y -= 1;
            }
            Direction::Down => {
                self.y += 1;
            }
            Direction::Left => {
                self.x -= 1;
            }
            Direction::Right => {
                self.x += 1;
            }
        }
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Box,
}

#[derive(Debug, Clone)]
struct Map {
    tiles: Vec<Tile>,
    // boxes: Vec<Position>,
    width: usize,
    height: usize,
    start: Position,
}

impl FromGridLike for Map {
    type Cell = u8;

    fn from_cells(cells: Vec<Self::Cell>, width: usize, height: usize) -> Self {
        // let mut boxes = Vec::new();
        let mut start = None;
        let tiles = cells
            .into_iter()
            .enumerate()
            .map(|(i, c)| match c {
                b'.' => Tile::Empty,
                // b'O' => {
                //     boxes.push(Position {
                //         x: (i % width) as u8,
                //         y: (i / width) as u8,
                //     });
                //     Tile::Empty
                // }
                b'O' => Tile::Box,
                b'@' => {
                    start = Some(Position {
                        x: (i % width) as u8,
                        y: (i / width) as u8,
                    });
                    Tile::Empty
                }
                b'#' => Tile::Wall,
                _ => unreachable!(),
            })
            .collect();
        Map {
            tiles,
            // boxes,
            width,
            height,
            start: start.unwrap(),
        }
    }
}

impl Map {
    fn from_ascii(ascii: &[u8]) -> Self {
        ascii.grid_like().unwrap().into_grid()
    }

    fn get_tile(&self, pos: Position) -> Option<Tile> {
        // skipping bounds check for now because map is bounded by walls
        self.tiles
            .get((pos.y as usize) * self.width + pos.x as usize)
            .copied()
    }

    fn set_tile(&mut self, pos: Position, tile: Tile) {
        self.tiles[(pos.y as usize) * self.width + pos.x as usize] = tile;
    }

    fn move_robot(&mut self, pos: Position, dir: Direction) -> Position {
        let new_pos = pos + dir;
        match self.get_tile(new_pos) {
            Some(Tile::Empty) => new_pos,
            Some(Tile::Wall) => pos,
            Some(Tile::Box) => {
                let mut new_box_pos = new_pos + dir;
                // if multiple boxes are stacked, move them all in the same move
                while self.get_tile(new_box_pos) == Some(Tile::Box) {
                    new_box_pos = new_box_pos + dir;
                }
                match self.get_tile(new_box_pos) {
                    Some(Tile::Empty) => {
                        self.set_tile(new_pos, Tile::Empty);
                        self.set_tile(new_box_pos, Tile::Box);
                        new_pos
                    }
                    _ => pos,
                }
            }
            _ => unreachable!(),
        }
    }

    fn part1_checksum(&self) -> usize {
        self.tiles
            .iter()
            .enumerate()
            .map(|(i, t)| match t {
                Tile::Box => (i % self.width) + 100 * (i / self.width),
                _ => 0,
            })
            .sum()
    }
}

#[derive(Debug)]
pub struct Puzzle {
    map: Map,
    instructions: Vec<Direction>,
}

#[aoc_generator(day15)]
pub fn parse(input: &[u8]) -> Puzzle {
    let split_point = input.windows(2).position(|x| x == b"\n\n").unwrap();
    let grid_part = &input[..split_point];
    let instructions_part = &input[split_point + 2..];
    let map = Map::from_ascii(grid_part);
    let instructions = instructions_part
        .iter()
        .copied()
        .map(Direction::try_from)
        .flatten()
        .collect();
    Puzzle { map, instructions }
}

#[aoc(day15, part1)]
pub fn part1(input: &Puzzle) -> usize {
    let mut map = input.map.clone();
    let mut pos = map.start;
    for dir in &input.instructions {
        pos = map.move_robot(pos, *dir);
    }
    map.part1_checksum()
}

#[aoc(day15, part2)]
pub fn part2(input: &Puzzle) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use aoc_utils::unindent_bytes;

    use super::*;

    #[test]
    fn checksum() {
        let input = unindent_bytes(
            b"
            ##########
            #.O.O.OOO#
            #........#
            #OO......#
            #OO@.....#
            #O#.....O#
            #O.....OO#
            #O.....OO#
            #OO....OO#
            ##########
            ",
        );
        let map = Map::from_ascii(&input);
        let checksum = map.part1_checksum();
        assert_eq!(checksum, 10092);
    }
}

example_tests! {
    b"
    ##########
    #..O..O.O#
    #......O.#
    #.OO..O.O#
    #..O@..O.#
    #O#..O...#
    #O..O..O.#
    #.OO.O.OO#
    #....O...#
    ##########

    <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
    vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
    ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
    <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
    ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
    ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
    >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
    <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
    ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
    v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
    ",
    part1 => 10092,
}
