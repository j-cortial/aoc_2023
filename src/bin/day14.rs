use itertools::Itertools;
use std::collections::{hash_map::Entry, HashMap};
use std::iter::{repeat, successors};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Stable,
    Round,
}

fn roll_to_begin(line: impl Iterator<Item = Tile>) -> Vec<Tile> {
    line.group_by(|&tile| tile != Tile::Stable)
        .into_iter()
        .flat_map(|(is_open, iter)| {
            let storage: Vec<Tile> = if is_open {
                let (empty_count, round_count) = iter.fold((0, 0), |acc, tile| match tile {
                    Tile::Empty => (acc.0 + 1, acc.1),
                    Tile::Round => (acc.0, acc.1 + 1),
                    _ => panic!(),
                });
                repeat(Tile::Round)
                    .take(round_count)
                    .chain(repeat(Tile::Empty).take(empty_count))
                    .collect()
            } else {
                iter.collect()
            };
            storage.into_iter()
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Platform {
    tiles: Vec<Vec<Tile>>,
}

impl Platform {
    fn col_count(&self) -> usize {
        self.tiles[0].len()
    }

    fn turn_anticlockwise(&self) -> Self {
        Self {
            tiles: (0..self.col_count())
                .rev()
                .map(|j| self.tiles.iter().map(move |row| row[j]).collect())
                .collect(),
        }
    }

    fn turn_clockwise(&self) -> Self {
        Self {
            tiles: (0..self.col_count())
                .map(|j| self.tiles.iter().rev().map(move |row| row[j]).collect())
                .collect(),
        }
    }

    fn roll_left(&self) -> Self {
        Self {
            tiles: self
                .tiles
                .iter()
                .map(|row| roll_to_begin(row.iter().copied()))
                .collect(),
        }
    }

    fn cycle_once(self) -> (Self, Self) {
        let steps: Vec<_> = successors(Some(self), |curr| Some(curr.roll_left().turn_clockwise()))
            .take(5)
            .collect();
        let mut iter = steps.into_iter();
        (iter.next().unwrap(), iter.last().unwrap())
    }

    fn load_on_left_beam(&self) -> u64 {
        self.tiles
            .iter()
            .flat_map(|row| {
                row.iter()
                    .rev()
                    .enumerate()
                    .filter_map(|(i, &tile)| (tile == Tile::Round).then_some(i + 1))
            })
            .sum::<usize>() as u64
    }
}

fn parse_input(input: &str) -> Platform {
    Platform {
        tiles: input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Tile::Empty,
                        '#' => Tile::Stable,
                        'O' => Tile::Round,
                        _ => panic!(),
                    })
                    .collect()
            })
            .collect(),
    }
}

fn solve_part1(platform: &Platform) -> u64 {
    platform
        .turn_anticlockwise()
        .roll_left()
        .load_on_left_beam()
}

fn solve_part2(platform: &Platform) -> u64 {
    let mut curr = platform.turn_anticlockwise();
    let mut memory = HashMap::new();
    let mut j: usize = 0;
    let i = loop {
        let (init, next) = curr.cycle_once();
        match memory.entry(init) {
            Entry::Occupied(entry) => {
                let (init, i) = entry.remove_entry();
                curr = init;
                break i;
            }
            Entry::Vacant(entry) => entry.insert(j),
        };
        curr = next;
        j += 1;
    };
    let remainder = (1_000_000_000 - j) % (j - i);
    for _ in 0..remainder {
        let (_, next) = curr.cycle_once();
        curr = next;
        j += 1;
    }
    curr.load_on_left_beam()
}

fn main() {
    let input = include_str!("../../data/day14.txt");
    let platform = parse_input(input);
    let answer1 = solve_part1(&platform);
    println!("The answer for part 1 is {}", answer1);
    let answer2 = solve_part2(&platform);
    println!("The answer for part 2 is {}", answer2);
}
