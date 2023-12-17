use std::collections::{HashMap, HashSet};

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
}

struct Layout {
    tiles: Vec<Vec<Tile>>,
}

impl Layout {
    fn contains(&self, loc: Loc) -> bool {
        loc.0 >= 0
            && loc.1 >= 0
            && loc.0 < self.tiles.len() as i64
            && loc.1 < self.tiles[0].len() as i64
    }

    fn tile(&self, loc: Loc) -> Option<Tile> {
        if self.contains(loc) {
            Some(self.tiles[loc.0 as usize][loc.1 as usize])
        } else {
            None
        }
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
    let mut visited: HashMap<Loc, HashSet<Direction>> = HashMap::new();
    let mut front = vec![(Loc(0, -1), Direction::East)];
    while let Some((loc, dir)) = front.pop() {
        let next_loc = loc.mv(dir);
        if let Some(next_tile) = layout.tile(next_loc) {
            if visited.entry(next_loc).or_default().insert(dir) {
                for next_dir in next_tile.outgoing_rays(dir) {
                    front.push((next_loc, next_dir));
                }
            }
        }
    }
    visited.len()
}

fn main() {
    let input = include_str!("../../data/day16.txt");
    let layout = parse_input(input);
    let answer1 = solve_part1(&layout);
    println!("The answer to part 1 is {}", answer1);
}
