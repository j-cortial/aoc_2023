use itertools::Itertools;
use std::iter::repeat;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

struct Platform {
    tiles: Vec<Vec<Tile>>,
}

impl Platform {
    fn row_count(&self) -> usize {
        self.tiles.len()
    }

    fn col_count(&self) -> usize {
        self.tiles[0].len()
    }

    fn roll_north(&self) -> Self {
        let transposed: Vec<Vec<_>> = (0..self.col_count())
            .map(|j| roll_to_begin(self.tiles.iter().map(move |row| row[j])))
            .collect();

        Platform {
            tiles: (0..self.row_count())
                .map(|i| transposed.iter().map(move |col| col[i]).collect())
                .collect(),
        }
    }

    fn load_on_north_beam(&self) -> u64 {
        self.tiles
            .iter()
            .rev()
            .enumerate()
            .map(|(i, row)| (i + 1) * row.iter().filter(|&&tile| tile == Tile::Round).count())
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
    platform.roll_north().load_on_north_beam()
}

fn main() {
    let input = include_str!("../../data/day14.txt");
    let platform = parse_input(input);
    let answer1 = solve_part1(&platform);
    println!("The answer for part 1 is {}", answer1);
}
