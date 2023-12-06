use nom::{
    bytes::complete::tag,
    character::complete::{char, multispace0, multispace1, one_of},
    combinator::{map_res, recognize},
    multi::{many0, many1, separated_list1},
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

#[derive(Debug)]
struct Race {
    time: i32,
    distance: i32,
}

fn integer(input: &str) -> IResult<&str, i32> {
    map_res(
        recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))),
        |s: &str| s.parse(),
    )(input)
}

fn parse_input(input: &str) -> Vec<Race> {
    let (times, distances) = separated_pair(
        preceded(
            tag("Time:"),
            preceded(multispace0, separated_list1(multispace1, integer)),
        ),
        multispace1,
        preceded(
            tag("Distance:"),
            preceded(multispace0, separated_list1(multispace1, integer)),
        ),
    )(input)
    .unwrap()
    .1;
    times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

fn main() {
    let input = include_str!("../../data/day06.txt");
    let races = parse_input(input);
    dbg!(races);
}
