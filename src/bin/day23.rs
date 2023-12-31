use std::collections::{HashMap, HashSet};

use strum::{EnumIter, IntoEnumIterator};

type Coord = i16;
type Loc = [Coord; 2];

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    fn opposite(&self) -> Self {
        match *self {
            Dir::North => Dir::South,
            Dir::East => Dir::West,
            Dir::South => Dir::North,
            Dir::West => Dir::East,
        }
    }

    fn step(&self) -> Loc {
        match *self {
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

#[derive(Debug)]
struct Terrain {
    tiles: HashMap<Loc, Tile>,
    rows: Coord,
    cols: Coord,
}

impl Terrain {
    fn new(tiles: HashMap<Loc, Tile>) -> Self {
        let (rows, cols) = tiles.iter().fold((0, 0), |acc, (loc, _)| {
            (acc.0.max(loc[0] + 1), acc.1.max(loc[1] + 2))
        });
        Self { tiles, rows, cols }
    }

    fn entry(&self) -> Loc {
        [0, 1]
    }

    fn exit(&self) -> Loc {
        [self.rows - 1, self.cols - 2]
    }

    fn valid_dry_moves_from<'a>(&'a self, loc: &'a Loc) -> impl Iterator<Item = (Dir, Loc)> + 'a {
        Dir::iter()
            .map(|dir| (dir, dir.offset(loc)))
            .filter(|(_, next_loc)| self.tiles.contains_key(next_loc))
    }

    fn valid_icy_moves_from<'a>(&'a self, loc: &'a Loc) -> impl Iterator<Item = (Dir, Loc)> + 'a {
        Dir::iter()
            .map(|dir| (dir, dir.offset(loc)))
            .filter(|(dir, next_loc)| match self.tiles.get(next_loc) {
                Some(tile) => match tile {
                    Tile::Flat => true,
                    Tile::Slope(slope) => dir == slope,
                },
                None => false,
            })
    }

    fn dry_nodes(&self) -> HashMap<Loc, Vec<Dir>> {
        self.tiles
            .keys()
            .map(|loc| {
                (
                    *loc,
                    self.valid_dry_moves_from(loc)
                        .map(|(d, _)| d)
                        .collect::<Vec<_>>(),
                )
            })
            .filter(|(_, v)| v.len() != 2)
            .collect()
    }

    fn icy_nodes(&self) -> HashMap<Loc, Vec<Dir>> {
        self.tiles
            .keys()
            .map(|loc| {
                (
                    *loc,
                    self.valid_dry_moves_from(loc)
                        .map(|(d, _)| d)
                        .collect::<Vec<_>>(),
                    self.valid_icy_moves_from(loc)
                        .map(|(d, _)| d)
                        .collect::<Vec<_>>(),
                )
            })
            .filter(|(_, v, _)| v.len() != 2)
            .map(|(l, d, _)| (l, d))
            .collect()
    }

    fn dry_edges(&self) -> HashMap<Loc, Vec<(Dir, Loc, u64)>> {
        let nodes = self.dry_nodes();
        let mut res = HashMap::<Loc, Vec<_>>::new();
        for (node_loc, node_dirs) in nodes.iter() {
            for dir in node_dirs {
                let mut weight = 1;
                let mut prev_dir = *dir;
                let mut curr_loc = prev_dir.offset(node_loc);
                while !nodes.contains_key(&curr_loc) {
                    (prev_dir, curr_loc) = self
                        .valid_dry_moves_from(&curr_loc)
                        .filter(|(d, _)| *d != prev_dir.opposite())
                        .next()
                        .unwrap();
                    weight += 1;
                }
                res.entry(*node_loc)
                    .or_default()
                    .push((*dir, curr_loc, weight));
            }
        }
        res
    }

    fn icy_edges(&self) -> HashMap<Loc, Vec<(Dir, Loc, u64)>> {
        let nodes = self.icy_nodes();
        let mut res = HashMap::<Loc, Vec<_>>::new();
        for (node_loc, node_dirs) in nodes.iter() {
            for dir in node_dirs {
                let mut weight = 1;
                let mut prev_dir = *dir;
                let mut curr_loc = prev_dir.offset(node_loc);
                let ok = loop {
                    if nodes.contains_key(&curr_loc) {
                        break true;
                    }
                    if let Some((curr_dir, next_loc)) = self
                        .valid_icy_moves_from(&curr_loc.clone())
                        .filter(|(d, _)| *d != prev_dir.opposite())
                        .next()
                    {
                        prev_dir = curr_dir;
                        curr_loc = next_loc;
                        weight += 1;
                    } else {
                        break false;
                    }
                };
                if ok {
                    res.entry(*node_loc)
                        .or_default()
                        .push((*dir, curr_loc, weight));
                }
            }
        }
        res
    }
}

fn parse_input(input: &str) -> Terrain {
    Terrain::new(
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
            .collect(),
    )
}

fn solve_impl(
    edges: &HashMap<Loc, Vec<(Dir, Loc, u64)>>,
    goal: Loc,
    mut visited: HashSet<Loc>,
    node: Loc,
    path_length: u64,
    res: &mut u64,
) {
    visited.insert(node);
    for &(_, next_node, weight) in &edges[&node] {
        if !visited.contains(&next_node) {
            if next_node == goal {
                *res = (*res).max(path_length + weight);
            } else {
                solve_impl(
                    edges,
                    goal,
                    visited.clone(),
                    next_node,
                    path_length + weight,
                    res,
                );
            }
        }
    }
}

fn solve_part1(terrain: &Terrain) -> u64 {
    let mut res = 0;
    solve_impl(
        &terrain.icy_edges(),
        terrain.exit(),
        Default::default(),
        terrain.entry(),
        0,
        &mut res,
    );
    res
}

fn solve_part2(terrain: &Terrain) -> u64 {
    let mut res = 0;
    solve_impl(
        &terrain.dry_edges(),
        terrain.exit(),
        Default::default(),
        terrain.entry(),
        0,
        &mut res,
    );
    res
}

fn main() {
    let input = include_str!("../../data/day23.txt");
    let terrain = parse_input(input);
    let answer1 = solve_part1(&terrain);
    println!("The answer to part 1 is {}", answer1);
    let answer2 = solve_part2(&terrain);
    println!("The answer to part 2 is {}", answer2);
}

#[cfg(test)]
mod test {
    use crate::*;
    const INPUT: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn test_solve_part1() {
        let terrain = parse_input(INPUT);
        assert_eq!(solve_part1(&terrain), 94);
    }

    #[test]
    fn test_solve_part2() {
        let terrain = parse_input(INPUT);
        assert_eq!(solve_part2(&terrain), 154);
    }
}
