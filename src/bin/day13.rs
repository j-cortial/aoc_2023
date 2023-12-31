use std::iter::zip;

use nom::{
    branch::alt,
    character::complete::{char, multispace1, newline},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

enum Direction {
    Horizontal,
    Vertical,
}

struct Split {
    direction: Direction,
    offset: usize,
}

impl Split {
    fn new(direction: Direction, offset: usize) -> Self {
        Self { direction, offset }
    }
}

struct Pattern {
    rocks: Vec<Vec<bool>>,
}

impl Pattern {
    fn has_horizontal_reflection(&self, offset: usize) -> bool {
        let (upper, lower) = self.rocks.split_at(offset);
        zip(upper.iter().rev(), lower.iter()).all(|(u, l)| u == l)
    }

    fn has_vertical_reflection(&self, offset: usize) -> bool {
        self.rocks.iter().all(|row| {
            let (left, right) = row.split_at(offset);
            zip(left.iter().rev(), right.iter()).all(|(u, l)| u == l)
        })
    }

    fn has_smudgy_horizontal_reflection(&self, offset: usize) -> bool {
        let (upper, lower) = self.rocks.split_at(offset);
        zip(upper.iter().rev(), lower.iter())
            .map(|(u, l)| zip(u, l).filter(|(u, l)| u != l).count())
            .sum::<usize>()
            == 1
    }

    fn has_smudgy_vertical_reflection(&self, offset: usize) -> bool {
        self.rocks.iter().map(|row| {
            let (left, right) = row.split_at(offset);
            zip(left.iter().rev(), right.iter()).filter(|(u, l)| u != l).count()
        }).sum::<usize>() == 1
    }

    fn rows(&self) -> usize {
        self.rocks.len()
    }

    fn cols(&self) -> usize {
        self.rocks[0].len()
    }

    fn find_reflection(&self) -> Option<Split> {
        if let Some(i) = (1..self.rows()).find(|&i| self.has_horizontal_reflection(i)) {
            return Some(Split::new(Direction::Horizontal, i));
        }
        (1..self.cols())
            .find(|&j| self.has_vertical_reflection(j))
            .map(|j| Split::new(Direction::Vertical, j))
    }

    fn find_smudgy_reflection(&self) -> Option<Split> {
        if let Some(i) = (1..self.rows()).find(|&i| self.has_smudgy_horizontal_reflection(i)) {
            return Some(Split::new(Direction::Horizontal, i));
        }
        (1..self.cols())
            .find(|&j| self.has_smudgy_vertical_reflection(j))
            .map(|j| Split::new(Direction::Vertical, j))
    }
}

fn pattern(input: &str) -> IResult<&str, Pattern> {
    map(
        separated_list1(
            newline,
            many1(map(alt((char('.'), char('#'))), |c| c == '#')),
        ),
        |rocks| Pattern { rocks },
    )(input)
}

fn parse_input(input: &str) -> Vec<Pattern> {
    separated_list1(multispace1, pattern)(input).unwrap().1
}

fn solve_part1(patterns: &[Pattern]) -> usize {
    patterns
        .iter()
        .map(|p| {
            p.find_reflection()
                .map(|Split{direction, offset}| match direction {
                    Direction::Horizontal => 100 * offset,
                    Direction::Vertical => offset,
                })
                .unwrap()
        })
        .sum()
}

fn solve_part2(patterns: &[Pattern]) -> usize {
    patterns
        .iter()
        .map(|p| {
            p.find_smudgy_reflection()
                .map(|Split{direction, offset}| match direction {
                    Direction::Horizontal => 100 * offset,
                    Direction::Vertical => offset,
                })
                .unwrap()
        })
        .sum()
}

fn main() {
    let input = include_str!("../../data/day13.txt");
    let patterns = parse_input(input);
    let answer1 = solve_part1(&patterns);
    println!("The answer to part 1 is {}", answer1);
    let answer2 = solve_part2(&patterns);
    println!("The answer to part 2 is {}", answer2);
}

#[cfg(test)]
mod test {
    use crate::pattern;

    #[test]
    fn test_vertical_reflection() {
        let pattern = pattern(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.",
        )
        .unwrap()
        .1;
        assert!(pattern.has_vertical_reflection(5));
    }

    #[test]
    fn test_horizontal_reflection() {
        let pattern = pattern(
            "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        )
        .unwrap()
        .1;
        assert!(pattern.has_horizontal_reflection(4));
    }

    #[test]
    fn test_has_smudgy_horizontal_reflection() {
let pattern = pattern(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.",
        )
        .unwrap()
        .1;
        assert!(pattern.has_smudgy_horizontal_reflection(3));
    }
}
