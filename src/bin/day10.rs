use std::{collections::HashSet, iter::successors};

#[derive(Debug, Clone, Copy)]
enum Tile {
    Ground,
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

type Loc = (i64, i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn opposite(&self) -> Self {
        match *self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
    fn next(&self, loc: Loc) -> Loc {
        let delta = match *self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        };
        (loc.0 + delta.0, loc.1 + delta.1)
    }
}

impl Tile {
    fn directions(&self) -> Vec<Direction> {
        use Direction::*;
        use Tile::*;
        match *self {
            Ground => vec![],
            NorthSouth => vec![North, South],
            EastWest => vec![East, West],
            NorthEast => vec![North, East],
            NorthWest => vec![North, West],
            SouthWest => vec![South, West],
            SouthEast => vec![South, East],
        }
    }
}

#[derive(Debug)]
struct Grid {
    tiles: Vec<Vec<Tile>>,
}

impl Grid {
    fn new(tiles: Vec<Vec<Tile>>, start: Loc) -> Self {
        let mut res = Self { tiles };
        use Direction::*;
        let directions: Vec<_> = [North, South, East, West]
            .into_iter()
            .filter_map(|d| {
                res.tile(d.next(start))
                    .directions()
                    .contains(&d.opposite())
                    .then_some(d)
            })
            .collect();
        use Tile::*;
        let tile = [
            NorthSouth, EastWest, NorthEast, NorthWest, SouthWest, SouthEast,
        ]
        .into_iter()
        .find(|t| t.directions() == directions)
        .unwrap();
        *res.tile_mut(start) = tile;
        res
    }

    fn tile(&self, loc: Loc) -> &Tile {
        &self.tiles[loc.0 as usize][loc.1 as usize]
    }

    fn tile_mut(&mut self, loc: Loc) -> &mut Tile {
        &mut self.tiles[loc.0 as usize][loc.1 as usize]
    }
}

fn parse_input(input: &str) -> (Grid, Loc) {
    use Tile::*;
    let mut start = Loc::default();
    let tiles = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '.' => Ground,
                    '|' => NorthSouth,
                    '-' => EastWest,
                    'L' => NorthEast,
                    'J' => NorthWest,
                    '7' => SouthWest,
                    'F' => SouthEast,
                    'S' => {
                        start = (i as i64, j as i64);
                        Ground
                    }
                    _ => panic!(),
                })
                .collect()
        })
        .collect();
    (Grid::new(tiles, start), start)
}

fn find_circuit(grid: &Grid, start: Loc) -> HashSet<Loc> {
    successors(
        Some((start, grid.tile(start).directions()[0])),
        |&(loc, dir)| {
            let next_loc = dir.next(loc);
            (next_loc != start).then_some((
                next_loc,
                grid.tile(next_loc)
                    .directions()
                    .into_iter()
                    .find(|&d| d != dir.opposite())
                    .unwrap(),
            ))
        },
    )
    .map(|(loc, _)| loc)
    .collect()
}

fn solve_part1(circuit: &HashSet<Loc>) -> usize {
    circuit.len() / 2
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Status {
    Out,
    In,
    InWall,
    InIfNorth,
    InIfSouth,
}

impl Status {
    fn next(&self, tile: Tile) -> Self {
        use Status::*;
        match *self {
            Out => match tile {
                Tile::Ground => panic!(),
                Tile::NorthSouth => InWall,
                Tile::EastWest => panic!(),
                Tile::NorthEast => InIfSouth,
                Tile::NorthWest => panic!(),
                Tile::SouthWest => panic!(),
                Tile::SouthEast => InIfNorth,
            },
            In | InWall => match tile {
                Tile::Ground => panic!(),
                Tile::NorthSouth => Out,
                Tile::EastWest => panic!(),
                Tile::NorthEast => InIfNorth,
                Tile::NorthWest => panic!(),
                Tile::SouthWest => panic!(),
                Tile::SouthEast => InIfSouth,
            },
            InIfNorth => match tile {
                Tile::Ground => panic!(),
                Tile::NorthSouth => panic!(),
                Tile::EastWest => *self,
                Tile::NorthEast => panic!(),
                Tile::NorthWest => InWall,
                Tile::SouthWest => Out,
                Tile::SouthEast => panic!(),
            },
            InIfSouth => match tile {
                Tile::Ground => panic!(),
                Tile::NorthSouth => panic!(),
                Tile::EastWest => *self,
                Tile::NorthEast => panic!(),
                Tile::NorthWest => Out,
                Tile::SouthWest => InWall,
                Tile::SouthEast => panic!(),
            },
        }
    }
}

fn solve_part2(grid: &Grid, circuit: &HashSet<Loc>) -> usize {
    grid.tiles
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(move |(j, tile)| ((i as i64, j as i64), tile))
                .scan(Status::Out, |status, (loc, tile)| {
                    *status = if circuit.contains(&loc) {
                        status.next(*tile)
                    } else if *status == Status::InWall {
                        Status::In
                    } else {
                        *status
                    };
                    Some(*status)
                })
                .filter(|&status| status == Status::In)
                .count()
        })
        .sum()
}

fn main() {
    let input = include_str!("../../data/day10.txt");
    let (grid, start) = parse_input(input);
    let circuit = find_circuit(&grid, start);
    let answer1 = solve_part1(&circuit);
    println!("The answer to part 1 is {}", answer1);
    let answer2 = solve_part2(&grid, &circuit);
    println!("The answer to part 2 is {}", answer2);
}
