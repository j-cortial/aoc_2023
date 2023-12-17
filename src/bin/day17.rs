use std::collections::{BinaryHeap, HashSet};

#[derive(Debug)]
struct City {
    blocks: Vec<Vec<u8>>,
}

impl City {
    fn row_count(&self) -> usize {
        self.blocks.len()
    }

    fn col_count(&self) -> usize {
        self.blocks[0].len()
    }

    fn contains(&self, loc: Loc) -> bool {
        loc.0 >= 0
            && loc.1 >= 0
            && loc.0 < self.row_count() as i16
            && loc.1 < self.col_count() as i16
    }

    fn block(&self, loc: Loc) -> Option<u8> {
        if self.contains(loc) {
            Some(self.blocks[loc.0 as usize][loc.1 as usize])
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Loc(i16, i16);

impl Loc {
    fn shift(self, dir: Direction) -> Self {
        Self(self.0 + dir.offset().0, self.1 + dir.offset().1)
    }

    fn manhattan_distance(self, other: Self) -> i16 {
        (self.0 - other.0).abs() + (self.1 + other.1).abs()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn offset(self) -> Loc {
        match self {
            Direction::North => Loc(-1, 0),
            Direction::West => Loc(0, -1),
            Direction::South => Loc(1, 0),
            Direction::East => Loc(0, 1),
        }
    }

    fn opposite(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::West => Direction::East,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
        }
    }

    fn all() -> impl Iterator<Item = Direction> {
        [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ]
        .into_iter()
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Status {
    loc: Loc,
    dir: Direction,
    repeats: u8,
    heat_loss: u32,
    heuristic: u32,
}

impl Status {
    fn neighbors<'a>(&'a self, city: &'a City, target: Loc) -> impl Iterator<Item = Status> + 'a {
        Direction::all()
            .filter(|&d| d != self.dir.opposite())
            .filter(|&d| self.repeats < 3 || d != self.dir)
            .filter_map(move |d| {
                let next_loc = self.loc.shift(d);
                city.block(next_loc).map(|loss| Status {
                    loc: next_loc,
                    dir: d,
                    repeats: {
                        if d == self.dir {
                            self.repeats + 1
                        } else {
                            1
                        }
                    },
                    heat_loss: self.heat_loss + loss as u32,
                    heuristic: next_loc.manhattan_distance(target) as u32,
                })
            })
    }
}

impl PartialOrd for Status {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some((other.heat_loss + other.heuristic).cmp(&(self.heat_loss + self.heuristic)))
    }
}

impl Ord for Status {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn solve_part1(city: &City) -> u32 {
    let mut visited = HashSet::new();
    let mut front = BinaryHeap::new();
    let target = Loc(city.row_count() as i16 - 1, city.col_count() as i16 - 1);
    let init = Status {
        loc: Loc(0, 0),
        dir: Direction::East,
        repeats: 0,
        heat_loss: 0,
        heuristic: Loc(0, 0).manhattan_distance(target) as u32,
    };
    visited.insert(init);
    front.push(init);
    while let Some(status) = front.pop() {
        if status.loc == target {
            return status.heat_loss;
        }
        for neighbor in status.neighbors(&city, target) {
            if visited.insert(neighbor) {
                front.push(neighbor);
            }
        }
    }
    0
}

fn parse_input(input: &str) -> City {
    City {
        blocks: input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect(),
    }
}

fn main() {
    let input = include_str!("../../data/day17.txt");
    let city = parse_input(input);
    let answer1 = solve_part1(&city);
    println!("The answer to part 1 is {}", answer1);
}

#[cfg(test)]
mod test {
    use crate::{parse_input, solve_part1};

    #[test]
    fn test_solve_part1() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        let city = parse_input(input);
        assert_eq!(solve_part1(&city), 102);
    }
}
