use std::str::FromStr;

use nom::{
    bytes::complete::{tag, take_while_m_n},
    character::complete::char,
    character::complete::{anychar, newline, one_of, space1},
    combinator::{map, map_res, recognize},
    multi::{many0, many1, separated_list1},
    sequence::{delimited, pair, separated_pair, terminated},
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'U' | '3' => Ok(Direction::Up),
            'L' | '2' => Ok(Direction::Left),
            'D' | '1' => Ok(Direction::Down),
            'R' | '0' => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

type Coord = i64;

#[derive(Debug, Clone)]
struct Move {
    dir: Direction,
    length: u32,
}

fn volume<'a>(moves: impl Iterator<Item = &'a Move>) -> Coord {
    let (vol, per, x) = moves.fold((0, 0, 0), |(vol, per, x), Move { dir, length }| {
        let length = *length as Coord;
        match *dir {
            Direction::Up => (vol + x * length, per + length, x),
            Direction::Left => (vol, per + length, x - length),
            Direction::Down => (vol - (x * length), per + length, x),
            Direction::Right => (vol, per + length, x + length),
        }
    });
    assert_eq!(x, 0);
    vol.abs() + (per / 2) + 1
}

fn from_hex(input: &str) -> Result<u32, std::num::ParseIntError> {
    u32::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn hex_distance(input: &str) -> IResult<&str, u32> {
    map_res(take_while_m_n(5, 5, is_hex_digit), from_hex)(input)
}

fn integer<I: FromStr>(input: &str) -> IResult<&str, I> {
    map_res(
        recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))),
        |s: &str| s.parse(),
    )(input)
}

fn parse_input(input: &str) -> Vec<(Move, Move)> {
    separated_list1(
        newline,
        separated_pair(
            map(
                separated_pair(
                    map_res(anychar, |c| Direction::try_from(c)),
                    space1,
                    integer::<u32>,
                ),
                |(dir, length)| Move { dir, length },
            ),
            space1,
            map(
                delimited(
                    tag("(#"),
                    pair(hex_distance, map_res(anychar, |c| Direction::try_from(c))),
                    char(')'),
                ),
                |(length, dir)| Move { dir, length },
            ),
        ),
    )(input)
    .unwrap()
    .1
}

fn solve_part1(data: &[(Move, Move)]) -> Coord {
    volume(data.iter().map(|(m, _)| m))
}

fn solve_part2(data: &[(Move, Move)]) -> Coord {
    volume(data.iter().map(|(_, m)| m))
}

fn main() {
    let input = include_str!("../../data/day18.txt");
    let data = parse_input(input);
    let answer1 = solve_part1(&data);
    println!("The answer to part 1 is {}", answer1);
    let answer2 = solve_part2(&data);
    println!("The answer to part 2 is {}", answer2);
}

#[cfg(test)]
mod test {
    use crate::*;

    const INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parse_input(INPUT)), 62);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parse_input(INPUT)), 952408144115);
    }
}
