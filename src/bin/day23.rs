use std::collections::HashMap;

use strum::EnumIter;

type Coord = i16;
type Loc = [Coord; 2];

#[derive(Debug, EnumIter)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    fn step(&self) -> Loc {
        match self {
            Dir::North => [-1, 0],
            Dir::East => [0, 1],
            Dir::South => [1, 0],
            Dir::West => [0, -1],
        }
    }
    fn offset(&self, loc: &Loc) -> Loc {
        let step = self.step();
        [loc[0] + step[0], loc[1] + step[1]]
    }
}

#[derive(Debug)]
enum Tile {
    Flat,
    Slope(Dir),
}

fn parse_input(input: &str) -> HashMap<Loc, Tile> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().filter_map(move |(col, c)| {
                match c {
                    '.' => Some(Tile::Flat),
                    '^' => Some(Tile::Slope(Dir::North)),
                    '>' => Some(Tile::Slope(Dir::East)),
                    'v' => Some(Tile::Slope(Dir::South)),
                    '<' => Some(Tile::Slope(Dir::West)),
                    _ => None,
                }
                .map(|tile| ([row as Coord, col as Coord], tile))
            })
        })
        .collect()
}

fn main() {
    let input = include_str!("../../data/day23.txt");
    let tiles = parse_input(input);
    dbg!(&tiles);
}
