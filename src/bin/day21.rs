use std::{collections::HashSet, mem::swap};

use itertools::Itertools;

type Coord = i16;
type Loc = [Coord; 2];

const MOVES: [Loc; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];

fn add(base: Loc, delta: Loc) -> Loc {
    [base[0] + delta[0], base[1] + delta[1]]
}

#[derive(Debug)]
struct Garden {
    rows: usize,
    cols: usize,
    open_plots: HashSet<Loc>,
}

impl Garden {
    fn reachable_open_plots(&self, start: Loc, steps: usize) -> HashSet<Loc> {
        let mut current = (HashSet::from([start]), vec![start]);
        let mut next = (HashSet::new(), vec![]);
        for _ in 0..steps {
            while let Some(base) = current.1.pop() {
                for delta in MOVES {
                    let candidate = add(base, delta);
                    if self.open_plots.contains(&candidate) && next.0.insert(candidate) {
                        next.1.push(candidate);
                    }
                }
            }
            swap(&mut current, &mut next);
        }
        current.0
    }
}

fn parse_input(input: &str) -> (Garden, Loc) {
    let (locs, is_open, is_start): (Vec<_>, Vec<_>, Vec<_>) = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().map(move |(col, tile)| {
                let loc = [row as Coord, col as Coord];
                match tile {
                    '.' => (loc, true, false),
                    '#' => (loc, false, false),
                    'S' => (loc, true, true),
                    _ => panic!(),
                }
            })
        })
        .multiunzip();
    (
        Garden {
            rows: input.lines().count(),
            cols: input.lines().next().unwrap().len(),
            open_plots: is_open
                .into_iter()
                .zip(locs.iter())
                .filter_map(|(b, l)| b.then_some(*l))
                .collect(),
        },
        is_start
            .into_iter()
            .zip(locs.iter())
            .filter_map(|(b, l)| b.then_some(*l))
            .next()
            .unwrap(),
    )
}

fn solve_part1(garden: &Garden, start: Loc) -> usize {
    garden.reachable_open_plots(start, 64).len()
}

fn main() {
    let input = include_str!("../../data/day21.txt");
    let (garden, start) = parse_input(input);
    let answer1 = solve_part1(&garden, start);
    println!("The answer to part 1 is {}", answer1)
}
