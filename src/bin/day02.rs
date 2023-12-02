use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, one_of},
    combinator::{map, map_res, recognize, value},
    multi::{many0, many1, separated_list1},
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Color {
    Blue,
    Green,
    Red,
}

fn color(input: &str) -> IResult<&str, Color> {
    alt((
        value(Color::Blue, tag("blue")),
        value(Color::Green, tag("green")),
        value(Color::Red, tag("red")),
    ))(input)
}

fn integer(input: &str) -> IResult<&str, u32> {
    map_res(
        recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))),
        |s: &str| s.parse::<_>(),
    )(input)
}

fn color_count(input: &str) -> IResult<&str, (Color, u32)> {
    map(separated_pair(integer, tag(" "), color), |(a, b)| (b, a))(input)
}

fn draw(input: &str) -> IResult<&str, HashMap<Color, u32>> {
    map(separated_list1(tag(", "), color_count), |v| {
        v.into_iter().collect()
    })(input)
}

fn game_id(input: &str) -> IResult<&str, u32> {
    preceded(tag("Game "), integer)(input)
}

fn game(input: &str) -> IResult<&str, Vec<HashMap<Color, u32>>> {
    preceded(
        terminated(game_id, tag(": ")),
        separated_list1(tag("; "), draw),
    )(input)
}

fn parse_input(input: &str) -> Vec<Vec<HashMap<Color, u32>>> {
    let (rem, res) = separated_list1(tag("\n"), game)(input).unwrap();
    println!("{}", rem);
    return res;
}

fn main() {
    let input = include_str!("../../data/day02.txt");
    let games = parse_input(input);
    assert_eq!(games.len(), 100);
    let content = HashMap::<_, _>::from([(Color::Blue, 14), (Color::Green, 13), (Color::Red, 12)]);
}
