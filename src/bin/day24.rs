use std::ops::{Add, Deref, Sub};

use itertools::Itertools;

type Coord = i64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Loc<const D: usize> {
    data: [Coord; D],
}

impl<const D: usize> Loc<D> {
    fn new(data: [Coord; D]) -> Self {
        Self { data }
    }
}

impl<const D: usize> Deref for Loc<D> {
    type Target = [Coord; D];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl Add for Loc<2> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new([self[0] + rhs[0], self[1] + rhs[1]])
    }
}

impl Sub for Loc<2> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new([self[0] - rhs[0], self[1] - rhs[1]])
    }
}

impl Loc<2> {
    fn cross_product(self, rhs: Self) -> Coord {
        self[0] * rhs[1] - self[1] * rhs[0]
    }
}

impl Loc<3> {
    fn projection(self) -> Loc<2> {
        Loc::new([self[0], self[1]])
    }
}

#[derive(Debug)]
struct Hailstone<const D: usize> {
    position: Loc<D>,
    velocity: Loc<D>,
}

impl Hailstone<3> {
    fn projection(&self) -> Hailstone<2> {
        Hailstone {
            position: self.position.projection(),
            velocity: self.velocity.projection(),
        }
    }
}

fn future_intersection(a: &Hailstone<2>, b: &Hailstone<2>) -> Option<[f64; 2]> {
    let det = b.velocity.cross_product(a.velocity);
    if det == 0 {
        return None;
    }
    let relative_position = b.position - a.position;
    let a_time = b.velocity.cross_product(relative_position);
    if a_time.signum() != det.signum() {
        return None;
    }
    let b_time = a.velocity.cross_product(relative_position);
    if b_time.signum() != det.signum() {
        return None;
    }
    Some([
        a.position[0] as f64 + a.velocity[0] as f64 * (a_time as f64 / det as f64),
        a.position[1] as f64 + a.velocity[1] as f64 * (a_time as f64 / det as f64),
    ])
}

fn in_range<const LOWER_BOUND: Coord, const UPPER_BOUND: Coord>(coord: f64) -> bool {
    LOWER_BOUND as f64 <= coord && coord <= UPPER_BOUND as f64
}

fn parse_loc3(input: &str) -> Loc<3> {
    let mut iter = input.split(", ");
    let mut parse = || iter.next().map(|s| s.trim().parse().unwrap()).unwrap();
    Loc::new([parse(), parse(), parse()])
}

fn parse_input(input: &str) -> Vec<Hailstone<3>> {
    input
        .lines()
        .map(|line| {
            line.split_once(" @ ")
                .map(|(p, v)| Hailstone {
                    position: parse_loc3(p.trim()),
                    velocity: parse_loc3(v.trim()),
                })
                .unwrap()
        })
        .collect()
}

fn solve_part1<const LOWER_BOUND: Coord, const UPPER_BOUND: Coord>(
    hailstones: &[Hailstone<3>],
) -> usize {
    let hailstones: Vec<_> = hailstones.iter().map(|h| h.projection()).collect();
    hailstones
        .iter()
        .combinations(2)
        .filter_map(|pair| {
            future_intersection(pair[0], pair[1]).filter(|loc| {
                in_range::<LOWER_BOUND, UPPER_BOUND>(loc[0])
                    && in_range::<LOWER_BOUND, UPPER_BOUND>(loc[1])
            })
        })
        .count()
}

fn main() {
    let input = include_str!("../../data/day24.txt");
    let hailstones = parse_input(input);
    let answer1 = solve_part1::<200_000_000_000_000, 400_000_000_000_000>(&hailstones);
    println!("The answer to part 1 is {answer1}");
}

#[cfg(test)]
mod test {
    use crate::*;

    static INPUT: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1::<7, 27>(&parse_input(INPUT)), 2);
    }
}
