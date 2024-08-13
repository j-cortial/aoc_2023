use itertools::Itertools;

use nalgebra::{Const, Matrix3, OMatrix, RowVector3, Vector3};

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

type Data = OMatrix<f64, Const<3>, Const<3>>;
type Residual = OMatrix<f64, Const<9>, Const<1>>;
type Gradient = OMatrix<f64, Const<9>, Const<6>>;

struct Problem {
    positions: Data,
    velocities: Data,
}

impl Problem {
    fn new(hailstones: &[Hailstone<3>]) -> Self {
        let positions = Data::from_fn(|i, j| hailstones[j].position[i] as f64);
        let velocities = Data::from_fn(|i, j| hailstones[j].velocity[i] as f64);
        Self {
            positions,
            velocities,
        }
    }

    fn relax(value: &Vector3<Coord>) -> Vector3<f64> {
        Vector3::from_fn(|i, _| value[i] as f64)
    }

    fn round(value: &Vector3<f64>) -> Vector3<Coord> {
        Vector3::from_fn(|i, _| value[i].round() as Coord)
    }

    fn residual(&self, pos: &Vector3<f64>, vel: &Vector3<f64>) -> Residual {
        let mut columns = self
            .positions
            .column_iter()
            .zip(self.velocities.column_iter())
            .map(|(p, v)| (pos - p).cross(&(vel - v)));
        let columns = [
            columns.next().unwrap(),
            columns.next().unwrap(),
            columns.next().unwrap(),
        ];
        Matrix3::from_columns(&columns).reshape_generic(Const::<9>, Const::<1>)
    }

    fn partial_gradient(state: &Vector3<f64>, data: &Data) -> OMatrix<f64, Const<9>, Const<3>> {
        let delta = RowVector3::repeat(1.0).kronecker(&Matrix3::identity());
        let diff =
            (data - RowVector3::repeat(1.0).kronecker(state)).kronecker(&RowVector3::repeat(1.0));
        OMatrix::<f64, Const<9>, Const<3>>::from_row_iterator(
            delta
                .column_iter()
                .zip(diff.column_iter())
                .flat_map(|(d, p)| {
                    p.cross(&d)
                        .into_iter()
                        .copied()
                        .collect::<Vec<_>>()
                        .into_iter()
                }),
        )
    }

    fn gradient(&self, pos: &Vector3<f64>, vel: &Vector3<f64>) -> Gradient {
        let pos_gradient = Self::partial_gradient(&vel, &self.velocities).scale(-1.0);
        let vel_gradient = Self::partial_gradient(&pos, &self.positions);
        Gradient::from_iterator(
            pos_gradient
                .into_iter()
                .chain(vel_gradient.into_iter())
                .copied(),
        )
    }

    fn solve(
        &self,
        init_pos: &Vector3<Coord>,
        init_vel: &Vector3<Coord>,
    ) -> (Vector3<Coord>, Vector3<Coord>) {
        let mut pos = Self::relax(init_pos);
        let mut vel = Self::relax(init_vel);
        let mut res = self.residual(&pos, &vel);
        let mut iter = 1;
        while res.norm() > 0.0 {
            let gradient = self.gradient(&pos, &vel);
            let qr = gradient.qr();
            qr.q_tr_mul(&mut res);
            let r = qr.unpack_r();
            let increment = r
                .fixed_rows::<6>(0)
                .solve_upper_triangular(&res.fixed_rows::<6>(0))
                .unwrap();
            pos -= increment.fixed_rows::<3>(0);
            vel -= increment.fixed_rows::<3>(3);
            iter += 1;
            if iter > 100 {
                break;
            }
            res = self.residual(&pos, &vel);
        }
        (Self::round(&pos), Self::round(&vel))
    }
}

fn solve_part2(hailstones: &[Hailstone<3>]) -> Coord {
    let problem = Problem::new(hailstones);
    let (pos, _) = problem.solve(&Vector3::zeros(), &Vector3::zeros());
    pos[0] + pos[1] + pos[2]
}

fn main() {
    let input = include_str!("../../data/day24.txt");
    let hailstones = parse_input(input);
    let answer1 = solve_part1::<200_000_000_000_000, 400_000_000_000_000>(&hailstones);
    println!("The answer to part 1 is {answer1}");
    let answer2 = solve_part2(&hailstones);
    println!("The answer to part 2 is {answer2}");
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

    #[test]
    fn test_problem_residual() {
        let problem = Problem::new(&parse_input(INPUT));
        assert_eq!(
            problem.residual(
                &Problem::relax(&Vector3::new(24, 13, 10)),
                &Problem::relax(&Vector3::new(-3, 1, 2))
            ),
            Residual::zeros()
        );
    }

    #[test]
    fn test_problem_solve() {
        let problem = Problem::new(&parse_input(INPUT));
        let (pos, vel) = problem.solve(&Vector3::zeros(), &Vector3::zeros());
        assert_eq!(pos, Vector3::new(24, 13, 10));
        assert_eq!(vel, Vector3::new(-3, 1, 2));
    }
}
