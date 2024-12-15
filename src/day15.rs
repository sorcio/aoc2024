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

    fn into_double(self) -> DoubleMap {
        let mut double_tiles = Vec::with_capacity(self.tiles.len() * 2);
        for tile in self.tiles {
            match tile {
                Tile::Empty => {
                    double_tiles.push(DoubleTile::Empty);
                    double_tiles.push(DoubleTile::Empty);
                }
                Tile::Wall => {
                    double_tiles.push(DoubleTile::Wall);
                    double_tiles.push(DoubleTile::Wall);
                }
                Tile::Box => {
                    double_tiles.push(DoubleTile::BoxLeft);
                    double_tiles.push(DoubleTile::BoxRight);
                }
            }
        }
        let width = self.width * 2;
        let height = self.height;
        let start = Position {
            x: self.start.x * 2,
            y: self.start.y,
        };
        DoubleMap {
            tiles: double_tiles,
            width,
            height,
            start,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DoubleTile {
    Empty,
    Wall,
    BoxLeft,
    BoxRight,
}

impl DoubleTile {
    fn is_box(self) -> bool {
        matches!(self, DoubleTile::BoxLeft | DoubleTile::BoxRight)
    }

    fn opposite_box(self) -> Self {
        match self {
            DoubleTile::BoxLeft => DoubleTile::BoxRight,
            DoubleTile::BoxRight => DoubleTile::BoxLeft,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct DoubleMap {
    tiles: Vec<DoubleTile>,
    width: usize,
    height: usize,
    start: Position,
}

impl DoubleMap {
    fn get_tile(&self, pos: Position) -> Option<DoubleTile> {
        // skipping bounds check for now because map is bounded by walls
        self.tiles
            .get((pos.y as usize) * self.width + pos.x as usize)
            .copied()
    }

    fn set_tile(&mut self, pos: Position, tile: DoubleTile) {
        self.tiles[(pos.y as usize) * self.width + pos.x as usize] = tile;
    }

    fn move_robot(&mut self, pos: Position, dir: Direction) -> Position {
        let new_pos = pos + dir;
        match self.get_tile(new_pos) {
            Some(DoubleTile::Empty) => new_pos,
            Some(DoubleTile::Wall) => pos,
            Some(box_tile @ (DoubleTile::BoxLeft | DoubleTile::BoxRight))
                if dir == Direction::Up || dir == Direction::Down =>
            {
                let mut queue = vec![(new_pos, box_tile)];
                if box_tile == DoubleTile::BoxLeft {
                    queue.push((new_pos + Direction::Right, DoubleTile::BoxRight));
                } else {
                    queue.push((new_pos + Direction::Left, DoubleTile::BoxLeft));
                }
                let mut clear = true;
                let mut boxes_to_push = Vec::new();
                while let Some((box_pos, tile)) = queue.pop() {
                    let new_box_pos = box_pos + dir;
                    match self.get_tile(new_box_pos).unwrap() {
                        DoubleTile::Empty => {}
                        DoubleTile::Wall => {
                            clear = false;
                            break;
                        }
                        DoubleTile::BoxLeft => {
                            queue.push((new_box_pos, DoubleTile::BoxLeft));
                            queue.push((new_box_pos + Direction::Right, DoubleTile::BoxRight));
                        }
                        DoubleTile::BoxRight => {
                            queue.push((new_box_pos, DoubleTile::BoxRight));
                            queue.push((new_box_pos + Direction::Left, DoubleTile::BoxLeft));
                        }
                    }
                    boxes_to_push.push((box_pos, new_box_pos, tile));
                }
                if clear {
                    for (box_pos, _, _) in &boxes_to_push {
                        self.set_tile(*box_pos, DoubleTile::Empty);
                    }
                    for (_, new_box_pos, tile) in &boxes_to_push {
                        self.set_tile(*new_box_pos, *tile);
                    }
                    new_pos
                } else {
                    pos
                }
            }
            Some(box_tile @ (DoubleTile::BoxLeft | DoubleTile::BoxRight))
                if dir == Direction::Left || dir == Direction::Right =>
            {
                let mut new_box_pos = new_pos + dir;
                // if multiple boxes are stacked, move them all in the same move
                while self.get_tile(new_box_pos).unwrap().is_box() {
                    new_box_pos = new_box_pos + dir;
                }
                match self.get_tile(new_box_pos) {
                    Some(DoubleTile::Empty) => {
                        self.set_tile(new_pos, DoubleTile::Empty);
                        {
                            let mut new_box_pos = new_pos + dir;
                            while self.get_tile(new_box_pos).unwrap().is_box() {
                                self.set_tile(
                                    new_box_pos,
                                    self.get_tile(new_box_pos).unwrap().opposite_box(),
                                );
                                new_box_pos = new_box_pos + dir;
                            }
                        }
                        self.set_tile(new_box_pos, box_tile.opposite_box());
                        new_pos
                    }
                    _ => pos,
                }
            }
            _ => unreachable!(),
        }
    }

    fn part2_checksum(&self) -> usize {
        self.tiles
            .iter()
            .enumerate()
            .map(|(i, t)| match t {
                DoubleTile::BoxLeft => (i % self.width) + 100 * (i / self.width),
                _ => 0,
            })
            .sum()
    }
}

struct DisplayMap<'a, T>(&'a T, Position);

impl std::fmt::Display for DisplayMap<'_, DoubleMap> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.0.height as u8 {
            for x in 0..self.0.width as u8 {
                let pos = Position { x, y };
                if pos == self.1 {
                    write!(f, "@")?;
                } else {
                    let tile = self.0.get_tile(pos).unwrap();
                    let c = match tile {
                        DoubleTile::Empty => '.',
                        DoubleTile::Wall => '#',
                        DoubleTile::BoxLeft => '[',
                        DoubleTile::BoxRight => ']',
                    };
                    write!(f, "{}", c)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
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
        .flat_map(Direction::try_from)
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
    let mut map = input.map.clone().into_double();
    let mut pos = map.start;
    for dir in &input.instructions {
        if cfg!(feature = "extra-debug-prints") {
            println!("{}\n\nMOVE: {dir:?}", DisplayMap(&map, pos));
        }
        pos = map.move_robot(pos, *dir);
    }
    if cfg!(feature = "extra-debug-prints") {
        println!("{}", DisplayMap(&map, pos));
    }
    map.part2_checksum()
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
    part2 => 9021,
}
