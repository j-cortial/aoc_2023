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
    fn iter<'a>(&'a self, start: Loc) -> GardenIterator<'a> {
        GardenIterator::new(&self, start)
    }

    fn reachable_open_plots(&self, start: Loc, steps: usize) -> usize {
        self.iter(start).skip(steps).next().unwrap()
    }

    fn is_well_behaved(&self, start: Loc) -> bool {
        (self.cols == self.rows)
            && (start[0] as usize * 2) == self.rows - 1
            && (start[1] as usize * 2) == self.cols - 1
            && (0..self.rows).all(|row| {
                [0, start[1], self.cols as Coord - 1]
                    .into_iter()
                    .all(|col| self.open_plots.contains(&[row as Coord, col]))
            })
            && (0..self.cols).all(|col| {
                [0, start[0], self.rows as Coord - 1]
                    .into_iter()
                    .all(|row| self.open_plots.contains(&[row, col as Coord]))
            })
    }
}

struct GardenIterator<'a> {
    parent: &'a Garden,
    current: GardenIteratorState,
    next: GardenIteratorState,
}

impl<'a> Iterator for GardenIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.current.set.len();
        while let Some(base) = self.current.front.pop() {
            for delta in MOVES {
                let candidate = add(base, delta);
                if self.parent.open_plots.contains(&candidate) && self.next.set.insert(candidate) {
                    self.next.front.push(candidate);
                }
            }
        }
        swap(&mut self.current, &mut self.next);
        Some(res)
    }
}

impl<'a> GardenIterator<'a> {
    fn new(parent: &'a Garden, start: Loc) -> Self {
        Self {
            parent,
            current: GardenIteratorState::new(start),
            next: GardenIteratorState::default(),
        }
    }
}

#[derive(Debug, Default)]
struct GardenIteratorState {
    set: HashSet<Loc>,
    front: Vec<Loc>,
}

impl GardenIteratorState {
    fn new(start: Loc) -> Self {
        Self {
            set: HashSet::from([start]),
            front: vec![start],
        }
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
    garden.reachable_open_plots(start, 64)
}

fn assert_part2(garden: &Garden, start: Loc) {
    assert!(garden.is_well_behaved(start));
    let size = garden.cols / 2;
    assert_ne!(
        garden.reachable_open_plots(start, 2 * size - 2),
        garden.reachable_open_plots(start, 2 * size)
    );
    assert_eq!(
        garden.reachable_open_plots(start, 2 * size - 1),
        garden.reachable_open_plots(start, 2 * size + 1)
    );
    assert_eq!(
        garden.reachable_open_plots(start, 2 * size),
        garden.reachable_open_plots(start, 2 * size + 2)
    );
    for root in [
        [0, 0],
        [0, garden.cols as Coord - 1],
        [garden.rows as Coord - 1, 0],
        [garden.rows as Coord - 1, garden.cols as Coord - 1],
    ] {
        assert_ne!(
            garden.reachable_open_plots(root, 4 * size - 2),
            garden.reachable_open_plots(root, 4 * size)
        );
        assert_eq!(
            garden.reachable_open_plots(root, 4 * size - 1),
            garden.reachable_open_plots(root, 4 * size + 1)
        );
        assert_eq!(
            garden.reachable_open_plots(root, 4 * size),
            garden.reachable_open_plots(root, 4 * size + 2)
        );
    }
    for root in [
        [start[0], 0],
        [start[0], garden.cols as Coord - 1],
        [0, start[1]],
        [garden.rows as Coord - 1, start[1]],
    ] {
        assert_ne!(
            garden.reachable_open_plots(root, 3 * size - 2),
            garden.reachable_open_plots(root, 3 * size)
        );
        assert_eq!(
            garden.reachable_open_plots(root, 3 * size - 1),
            garden.reachable_open_plots(root, 3 * size + 1)
        );
        assert_eq!(
            garden.reachable_open_plots(root, 3 * size),
            garden.reachable_open_plots(root, 3 * size + 2)
        );
    }
}

fn even_integer_sum(bound: usize) -> usize {
    let count = bound / 2;
    count * (count + 1)
}

fn odd_integer_sum(bound: usize) -> usize {
    let count = (bound + 1) / 2;
    count * count
}

fn solve_part2(garden: &Garden, start: Loc) -> usize {
    assert_part2(garden, start);
    let period = garden.rows;
    let distance = garden.rows / 2;
    assert_eq!(period, 2 * distance + 1);
    assert_eq!(start[0] as usize, distance);
    assert_eq!(start[1] as usize, distance);
    let target_steps = 26_501_365;
    let periods = target_steps / period;
    let remainder = target_steps - (period * periods);
    assert_eq!(remainder, distance);
    let (even_count, odd_count) = {
        let mut iter = garden.iter(start).skip(2 * distance);
        (iter.next().unwrap(), iter.next().unwrap())
    };
    assert_eq!(target_steps % 2, 1);
    let full_count = odd_count
        + 4 * (even_count * odd_integer_sum(periods - 1)
            + odd_count * even_integer_sum(periods - 1));
    let point_count = [
        [start[0], 0],
        [start[0], start[1] * 2],
        [0, start[1]],
        [start[0] * 2, start[1]],
    ]
    .into_iter()
    .map(|root| garden.reachable_open_plots(root, distance + remainder))
    .sum::<usize>();
    let fat_count = [
        [0, 0],
        [0, start[1] * 2],
        [start[0] * 2, 0],
        [start[0] * 2, start[1] * 2],
    ]
    .into_iter()
    .map(|root| garden.reachable_open_plots(root, 2 * distance + remainder))
    .sum::<usize>()
        * (periods - 1);
    let slim_count = [
        [0, 0],
        [0, start[1] * 2],
        [start[0] * 2, 0],
        [start[0] * 2, start[1] * 2],
    ]
    .into_iter()
    .map(|root| garden.reachable_open_plots(root, remainder - 1))
    .sum::<usize>()
        * periods;
    full_count + point_count + fat_count + slim_count
}

fn main() {
    let input = include_str!("../../data/day21.txt");
    let (garden, start) = parse_input(input);
    let answer1 = solve_part1(&garden, start);
    println!("The answer to part 1 is {}", answer1);
    let answer2 = solve_part2(&garden, start);
    println!("The answer to part 2 is {}", answer2);
}
