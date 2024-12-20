use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::{AsciiUtils, FromGridLike, grid_cell_enum};

grid_cell_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    enum InputTile {
        Empty => b'.',
        Wall => b'#',
        Start => b'S',
        End => b'E',
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Wall,
}

impl From<InputTile> for Tile {
    fn from(input: InputTile) -> Self {
        match input {
            InputTile::Empty | InputTile::Start | InputTile::End => Tile::Empty,
            InputTile::Wall => Tile::Wall,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: u8,
    y: u8,
}

impl std::hash::Hash for Position {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.x as u32 + self.y as u32 * 256).hash(state);
    }
}

impl Position {
    fn from_index(index: usize, width: usize) -> Self {
        Self {
            x: (index % width).try_into().unwrap(),
            y: (index / width).try_into().unwrap(),
        }
    }

    fn step(self, heading: Heading) -> Self {
        // skipping overflow checks because maze is surrounded by walls
        match heading {
            Heading::North => Self {
                x: self.x,
                y: self.y - 1,
            },
            Heading::East => Self {
                x: self.x + 1,
                y: self.y,
            },
            Heading::South => Self {
                x: self.x,
                y: self.y + 1,
            },
            Heading::West => Self {
                x: self.x - 1,
                y: self.y,
            },
        }
    }

    fn manhattan_distance(self, other: Self) -> u8 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Heading {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone)]
struct Maze {
    grid: Vec<Tile>,
    width: usize,
    _height: usize,
    start: Position,
    end: Position,
}

impl FromGridLike for Maze {
    type Cell = InputTile;

    fn from_cells(cells: Vec<Self::Cell>, width: usize, height: usize) -> Self {
        let start = cells
            .iter()
            .position(|&t| t == InputTile::Start)
            .map(|i| Position::from_index(i, width))
            .unwrap();
        let end = cells
            .iter()
            .position(|&t| t == InputTile::End)
            .map(|i| Position::from_index(i, width))
            .unwrap();
        let grid = cells.into_iter().map(Tile::from).collect();
        Self {
            grid,
            width,
            _height: height,
            start,
            end,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Step {
    Free(Position),
    Wall(Position),
}

impl Step {
    fn free(self) -> Option<Position> {
        match self {
            Step::Free(pos) => Some(pos),
            Step::Wall(_) => None,
        }
    }
}

impl Maze {
    fn get_tile(&self, position: Position) -> Tile {
        // skipping bound checks because maze is surrounded by walls
        self.grid[position.y as usize * self.width + position.x as usize]
    }

    fn step(&self, position: Position, heading: Heading) -> Step {
        let new_position = position.step(heading);
        if self.get_tile(new_position) == Tile::Wall {
            Step::Wall(new_position)
        } else {
            Step::Free(new_position)
        }
    }
}

fn solve_without_cheats(maze: &Maze) -> Vec<Position> {
    // mazes are guaranteed to have a unique path from start to end so
    // let's just walk to the next empty tile until we reach the end
    let mut pos = maze.start;
    let mut prev_pos = None;
    let mut steps = vec![maze.start];
    while pos != maze.end {
        let next_pos = [Heading::North, Heading::East, Heading::South, Heading::West]
            .iter()
            .filter_map(|&heading| maze.step(pos, heading).free())
            .find(|next_pos| Some(*next_pos) != prev_pos)
            .expect("Maze should have a solution");
        prev_pos = Some(pos);
        pos = next_pos;
        steps.push(pos);
    }
    steps
}

#[aoc_generator(day20)]
fn parse(input: &[u8]) -> Maze {
    input.grid_like().unwrap().into_grid()
}

fn part1_solve(maze: &Maze, desired_saving: usize) -> usize {
    let steps = solve_without_cheats(maze);

    // TODO: this is quadratic, could be made faster by using a grid
    steps
        .iter()
        .enumerate()
        .rev()
        .map(|(i, pos)| {
            steps[..i]
                .iter()
                .rev()
                .skip(desired_saving)
                .filter(|other_pos| pos.manhattan_distance(**other_pos) == 2)
                .count()
        })
        .sum()
}

#[aoc(day20, part1)]
fn part1(input: &Maze) -> usize {
    part1_solve(input, 100)
}

#[aoc(day20, part2)]
fn part2(input: &Maze) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use aoc_utils::unindent_bytes;

    use super::*;

    const EXAMPLE: &[u8] = b"\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

    #[test]
    fn no_cheats() {
        let maze = parse(EXAMPLE);
        assert_eq!(solve_without_cheats(&maze).len() - 1, 84);
    }

    #[test]
    fn part1_example() {
        let maze = parse(EXAMPLE);

        // 1 saves 64
        // 1 saves 40
        // 1 saves 38
        // 1 saves 36
        // 1 saves 20
        // 3 save 12
        // 2 save 10
        // 4 save 8
        // 2 save 6
        // 14 save 4
        // 14 save 2

        // Our solver gives us the count of cheats that save *at least* N
        assert_eq!(part1_solve(&maze, 64), 1);
        assert_eq!(part1_solve(&maze, 40), 2);
        assert_eq!(part1_solve(&maze, 38), 3);
        assert_eq!(part1_solve(&maze, 36), 4);
        assert_eq!(part1_solve(&maze, 20), 5);
        assert_eq!(part1_solve(&maze, 12), 8);
        assert_eq!(part1_solve(&maze, 10), 10);
        assert_eq!(part1_solve(&maze, 8), 14);
        assert_eq!(part1_solve(&maze, 6), 16);
        assert_eq!(part1_solve(&maze, 4), 30);
        assert_eq!(part1_solve(&maze, 2), 44);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}
