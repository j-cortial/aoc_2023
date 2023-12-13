use std::collections::HashSet;
use std::iter::{repeat, successors, zip};
use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Loc(i64, i64);

impl Loc {
    fn distance(self, other: Self) -> i64 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

fn inner_range(a: i64, b: i64) -> Range<i64> {
    a.min(b) + 1..a.max(b)
}

#[derive(Debug)]
struct Universe {
    galaxies: HashSet<Loc>,
    expanded_rows: Vec<i64>,
    expanded_cols: Vec<i64>,
}

impl Universe {
    fn new(space: Vec<Vec<bool>>) -> Self {
        let galaxies = space
            .iter()
            .enumerate()
            .flat_map(move |(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(j, b)| b.then_some(Loc(i as i64, j as i64)))
            })
            .collect();

        let expanded_rows = space
            .iter()
            .enumerate()
            .filter_map(|(i, row)| row.iter().all(|b| (!b)).then_some(i as i64))
            .collect();
        let expanded_cols = (0..space[0].len() as i64)
            .filter(|j| space.iter().all(|row| !row[*j as usize]))
            .collect();

        Universe {
            galaxies,
            expanded_rows,
            expanded_cols,
        }
    }

    fn galaxy_locations(&self) -> impl Iterator<Item = Loc> + Clone + '_ {
        self.galaxies.iter().copied()
    }

    fn galaxy_pairs(&self) -> impl Iterator<Item = (Loc, Loc)> + '_ {
        let left_iters = self.galaxy_locations().map(|item| repeat(item));
        let right_iters = successors(Some(self.galaxy_locations().skip(1)), |it| {
            let mut res = it.clone();
            res.next().is_some().then_some(res)
        });
        zip(left_iters, right_iters).flat_map(|(r, l)| zip(l, r))
    }

    fn distance(&self, a: Loc, b: Loc, age_factor: i64) -> i64 {
        a.distance(b)
            + (age_factor - 1)
                * (inner_range(a.0, b.0)
                    .filter(|i| self.expanded_rows.binary_search(i).is_ok())
                    .count() as i64
                    + inner_range(a.1, b.1)
                        .filter(|j| self.expanded_cols.binary_search(j).is_ok())
                        .count() as i64)
    }
}

fn parse_input(input: &str) -> Universe {
    let space: Vec<_> = input
        .lines()
        .map(move |line| line.chars().map(|c| c == '#').collect())
        .collect();
    Universe::new(space)
}

fn solve(universe: &Universe, age_factor: i64) -> i64 {
    universe
        .galaxy_pairs()
        .map(|p| universe.distance(p.0, p.1, age_factor))
        .sum()
}

fn solve_part1(universe: &Universe) -> i64 {
    solve(universe, 2)
}

fn solve_part2(universe: &Universe) -> i64 {
    solve(universe, 1_000_000)
}

fn main() {
    let input = include_str!("../../data/day11.txt");
    let universe = parse_input(input);
    let answer1 = solve_part1(&universe);
    println!("The answer to part 1 is {}", answer1);
    let answer2 = solve_part2(&universe);
    println!("The answer to part 2 is {}", answer2);
}
