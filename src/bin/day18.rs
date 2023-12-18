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

#[derive(Debug)]
struct Move {
    dir: Direction,
    length: u8,
}

#[derive(Debug, PartialEq)]
struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
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

fn main() {
    let input = include_str!("../../data/day18.txt");
    let data = parse_input(input);
    dbg!(&data);
}
