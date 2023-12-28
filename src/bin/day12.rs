use std::str::FromStr;

use nom::{
    character::complete::{anychar, char, newline, one_of, space1},
    combinator::{map, map_res, recognize},
    multi::{many0, many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};

#[derive(Debug, Clone, Copy)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl TryFrom<char> for Condition {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Condition::Operational),
            '#' => Ok(Condition::Damaged),
            '?' => Ok(Condition::Unknown),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Record {
    conditions: Vec<Condition>,
    checksum: Vec<u8>,
}

fn integer<I: FromStr>(input: &str) -> IResult<&str, I> {
    map_res(
        recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))),
        |s: &str| s.parse(),
    )(input)
}

fn parse_input(input: &str) -> Vec<Record> {
    separated_list1(
        newline,
        map(
            separated_pair(
                many1(map_res(anychar, |c| c.try_into())),
                space1,
                separated_list1(char(','), integer),
            ),
            |(conditions, checksum)| Record {
                conditions,
                checksum,
            },
        ),
    )(input)
    .unwrap()
    .1
}

fn main() {
    let input = include_str!("../../data/day12.txt");
    let records = parse_input(input);
    dbg!(&records);
}
