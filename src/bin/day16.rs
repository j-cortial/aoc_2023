use std::collections::{HashMap, HashSet};

use itertools::chain;

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    Slash,
    Backslash,
    Dash,
    Pipe,
}

impl Tile {
    fn parse(c: char) -> Option<Self> {
        match c {
            '.' => Some(Self::Empty),
            '/' => Some(Self::Slash),
            '\\' => Some(Self::Backslash),
            '-' => Some(Self::Dash),
            '|' => Some(Self::Pipe),
            _ => None,
        }
    }

    fn outgoing_rays(&self, dir: Direction) -> Vec<Direction> {
        match *self {
            Tile::Empty => vec![dir],
            Tile::Slash => vec![match dir {
                Direction::North => Direction::East,
                Direction::West => Direction::South,
                Direction::South => Direction::West,
                Direction::East => Direction::North,
            }],
            Tile::Backslash => vec![match dir {
                Direction::North => Direction::West,
                Direction::West => Direction::North,
                Direction::South => Direction::East,
                Direction::East => Direction::South,
            }],
            Tile::Dash => match dir {
                Direction::North | Direction::South => vec![Direction::West, Direction::East],
                Direction::West | Direction::East => vec![dir],
            },
            Tile::Pipe => match dir {
                Direction::North | Direction::South => vec![dir],
                Direction::West | Direction::East => vec![Direction::North, Direction::South],
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn offset(self) -> Loc {
        match self {
            Direction::North => Loc(-1, 0),
            Direction::West => Loc(0, -1),
            Direction::South => Loc(1, 0),
            Direction::East => Loc(0, 1),
        }
    }

    fn opposite(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::West => Direction::East,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
        }
    }
}

struct Layout {
    tiles: Vec<Vec<Tile>>,
}

impl Layout {
    fn row_count(&self) -> usize {
        self.tiles.len()
    }

    fn col_count(&self) -> usize {
        self.tiles[0].len()
    }

    fn contains(&self, loc: Loc) -> bool {
        loc.0 >= 0
            && loc.1 >= 0
            && loc.0 < self.row_count() as i64
            && loc.1 < self.col_count() as i64
    }

    fn tile(&self, loc: Loc) -> Option<Tile> {
        if self.contains(loc) {
            Some(self.tiles[loc.0 as usize][loc.1 as usize])
        } else {
            None
        }
    }

    fn energized_tiles(&self, dir: Direction, loc: Loc) -> usize {
        let mut visited: HashMap<Loc, HashSet<Direction>> = HashMap::new();
        let mut front = vec![(loc.mv(dir.opposite()), dir)];
        while let Some((loc, dir)) = front.pop() {
            let next_loc = loc.mv(dir);
            if let Some(next_tile) = self.tile(next_loc) {
                if visited.entry(next_loc).or_default().insert(dir) {
                    for next_dir in next_tile.outgoing_rays(dir) {
                        front.push((next_loc, next_dir));
                    }
                }
            }
        }
        visited.len()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Loc(i64, i64);

impl Loc {
    fn mv(self, dir: Direction) -> Self {
        Self(self.0 + dir.offset().0, self.1 + dir.offset().1)
    }
}

fn parse_input(input: &str) -> Layout {
    Layout {
        tiles: input
            .lines()
            .map(|line| line.chars().map(|c| Tile::parse(c).unwrap()).collect())
            .collect(),
    }
}

fn solve_part1(layout: &Layout) -> usize {
    layout.energized_tiles(Direction::East, Loc(0, 0))
}

fn solve_part2(layout: &Layout) -> usize {
    let vertical = (0..layout.col_count()).flat_map(move |j| {
        [
            (Direction::South, 0),
            (Direction::North, layout.row_count() - 1),
        ]
        .into_iter()
        .map(move |(d, i)| (d, Loc(i as i64, j as i64)))
    });
    let horizontal = (0..layout.row_count()).flat_map(move |i| {
        [
            (Direction::East, 0),
            (Direction::West, layout.col_count() - 1),
        ]
        .into_iter()
        .map(move |(d, j)| (d, Loc(i as i64, j as i64)))
    });
    chain(vertical, horizontal)
        .map(|(dir, loc)| layout.energized_tiles(dir, loc))
        .max()
        .unwrap()
}

fn main() {
    let input = include_str!("../../data/day16.txt");
    let layout = parse_input(input);
    let answer1 = solve_part1(&layout);
    println!("The answer to part 1 is {}", answer1);
    let answer2 = solve_part2(&layout);
    println!("The answer to part 2 is {}", answer2);
}
