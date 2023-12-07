use std::str::FromStr;

use nom::{
    character::complete::{char, multispace1, one_of, space1},
    combinator::{map_res, recognize, map},
    multi::{many0, many1, many_m_n, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};

type Money = u32;

#[derive(Debug)]
struct Hand<'a> {
    cards: &'a str,
    bet: Money,
}

fn integer<I: FromStr>(input: &str) -> IResult<&str, I> {
    map_res(
        recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))),
        |s: &str| s.parse(),
    )(input)
}

fn parse_input(input: &str) -> Vec<Hand> {
    separated_list1(
        multispace1,
        map(separated_pair(
            recognize(many_m_n(5, 5, one_of("AKQJT98765432"))),
            space1,
            integer::<Money>,
        ), |(cards, bet)| Hand{cards, bet}),
    )(input)
    .unwrap()
    .1
}

fn main() {
    let input = include_str!("../../data/day07.txt");
    let hands = parse_input(input);
    dbg!(&hands);
}
