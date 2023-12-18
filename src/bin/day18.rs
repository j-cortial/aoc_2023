use std::str::FromStr;

use nom::{
    bytes::complete::{tag, take_while_m_n},
    character::complete::char,
    character::complete::{anychar, newline, one_of, space1},
    combinator::{map, map_res, recognize},
    multi::{many0, many1, separated_list1},
    sequence::{delimited, separated_pair, terminated, tuple},
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
            'U' => Ok(Direction::Up),
            'L' => Ok(Direction::Left),
            'D' => Ok(Direction::Down),
            'R' => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

impl Direction {
    fn offset(self) -> Loc {
        match self {
            Direction::Up => Loc(-1, 0),
            Direction::Left => Loc(0, -1),
            Direction::Down => Loc(1, 0),
            Direction::Right => Loc(0, 1),
        }
    }
}

type Coord = i32;
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Loc(Coord, Coord);

impl Loc {
    fn shift(self, dir: Direction) -> Self {
        Self(self.0 + dir.offset().0, self.1 + dir.offset().1)
    }
}

#[derive(Debug, Clone)]
struct Move {
    dir: Direction,
    length: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
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

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
}

fn hex_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;

    Ok((input, Color { red, green, blue }))
}

fn integer<I: FromStr>(input: &str) -> IResult<&str, I> {
    map_res(
        recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))),
        |s: &str| s.parse(),
    )(input)
}

fn parse_input(input: &str) -> Vec<(Move, Color)> {
    separated_list1(
        newline,
        separated_pair(
            map(
                separated_pair(
                    map_res(anychar, |c| Direction::try_from(c)),
                    space1,
                    integer::<u8>,
                ),
                |(dir, length)| Move { dir, length },
            ),
            space1,
            delimited(char('('), hex_color, char(')')),
        ),
    )(input)
    .unwrap()
    .1
}

fn solve_part1(data: &[(Move, Color)]) -> Coord {
    volume(data.iter().map(|(m, _)| m))
}

fn main() {
    let input = include_str!("../../data/day18.txt");
    let data = parse_input(input);
    let answer1 = solve_part1(&data);
    println!("The answer to part 1 is {}", answer1);
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
        assert_eq!(solve_part1(&parse_input(INPUT)), 62i32);
    }
}
