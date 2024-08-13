use itertools::Itertools;

use nalgebra::{Const, OMatrix, Vector3};

type Coord = i64;

type Loc<const D: usize> = OMatrix<Coord, Const<D>, Const<1>>;

fn projection(loc2: &Loc<3>) -> Loc<2> {
    loc2.fixed_rows::<2>(0).into()
}

fn cross_product(left: &Loc<2>, right: &Loc<2>) -> Coord {
    left[0] * right[1] - left[1] * right[0]
}

#[derive(Debug)]
struct Hailstone<const D: usize> {
    position: Loc<D>,
    velocity: Loc<D>,
}

impl Hailstone<3> {
    fn projection(&self) -> Hailstone<2> {
        Hailstone {
            position: projection(&self.position),
            velocity: projection(&self.velocity),
        }
    }
}

fn future_intersection(a: &Hailstone<2>, b: &Hailstone<2>) -> Option<[f64; 2]> {
    let det = cross_product(&b.velocity, &a.velocity);
    if det == 0 {
        return None;
    }
    let relative_position = b.position - a.position;
    let a_time = cross_product(&b.velocity, &relative_position);
    if a_time.signum() != det.signum() {
        return None;
    }
    let b_time = cross_product(&a.velocity, &relative_position);
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
    Vector3::new(parse(), parse(), parse())
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
