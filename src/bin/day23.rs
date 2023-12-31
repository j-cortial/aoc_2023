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

#[derive(Debug)]
struct Fork {
    loc: Loc,
    visited: HashSet<Loc>,
    candidates: Vec<Dir>,
}

impl Fork {
    fn from_entry(entry: Loc) -> Self {
        Self {
            loc: entry,
            visited: HashSet::from([entry]),
            candidates: vec![Dir::South],
        }
    }

    fn pop(&mut self) -> Option<Dir> {
        self.candidates.pop()
    }
}

fn solve(terrain: &Terrain, dry: bool) -> u64 {
    let mut res = 0;
    let mut forks = vec![Fork::from_entry(terrain.entry())];
    while let Some(mut fork) = forks.pop() {
        if let Some(mut dir) = fork.pop() {
            let mut base_loc = fork.loc;
            let mut visited = HashSet::new();
            loop {
                let loc = dir.offset(&base_loc);
                visited.insert(loc);
                if loc == terrain.exit() {
                    let path_length = visited.len()
                        + fork.visited.len()
                        + forks.iter().map(|f| f.visited.len()).sum::<usize>()
                        - 1;
                    res = res.max(path_length as u64);
                    forks.push(fork);
                    break;
                }
                let candidates: Vec<_> = Dir::iter()
                    .filter(|&d| d != dir.opposite())
                    .filter(|d| {
                        let next_loc = d.offset(&loc);
                        !visited.contains(&next_loc)
                            && !fork.visited.contains(&next_loc)
                            && forks.iter().all(|f| !f.visited.contains(&next_loc))
                            && match terrain.tiles.get(&next_loc) {
                                Some(tile) => match *tile {
                                    Tile::Flat => true,
                                    Tile::Slope(slope) => dry || slope != d.opposite(),
                                },
                                None => false,
                            }
                    })
                    .collect();
                match candidates.len() {
                    0 => {
                        if dry {
                            if let Some(index) = fork
                                .candidates
                                .iter()
                                .position(|d| loc == d.offset(&fork.loc))
                            {
                                println!(
                                    "Removing candidate {:?} at fork {:?}",
                                    &fork.candidates[index], &fork
                                );
                                fork.candidates.remove(index);
                            }
                        }
                        forks.push(fork);
                        break;
                    }
                    1 => {
                        dir = candidates[0];
                        base_loc = loc;
                    }
                    _ => {
                        forks.push(fork);
                        forks.push(Fork {
                            loc,
                            visited,
                            candidates,
                        });
                        break;
                    }
                };
            }
        }
    }
    res
}

fn solve_part1(terrain: &Terrain) -> u64 {
    solve(terrain, false)
}

fn solve_part2(terrain: &Terrain) -> u64 {
    solve(terrain, true)
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
