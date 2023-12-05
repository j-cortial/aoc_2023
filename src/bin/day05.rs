use nom::{
    bytes::complete::tag,
    character::complete::{char, none_of, one_of},
    combinator::{map, map_res, recognize},
    multi::{many0, many1, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    IResult,
};

type Id = u64;


#[derive(Debug)]
struct Entry {
    source: Id,
    target: Id,
    range: Id,
}

fn integer(input: &str) -> IResult<&str, Id> {
    map_res(
        recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))),
        |s: &str| s.parse(),
    )(input)
}

fn entry(input: &str) -> IResult<&str, Entry> {
    map(separated_list1(tag(" "), integer), |v| Entry {
        source: v[0],
        target: v[1],
        range: v[2],
    })(input)
}

fn initial_seeds(input: &str) -> IResult<&str, Vec<Id>> {
    delimited(
        tag("seeds: "),
        separated_list1(tag(" "), integer),
        tag("\n"),
    )(input)
}

fn mapping(input: &str) -> IResult<&str, Vec<Entry>> {
    preceded(
        pair(many1(none_of(":")), tag(":\n")),
        many1(terminated(entry, tag("\n"))),
    )(input)
}

fn parse_input(input: &str) -> (Vec<Id>, Vec<Vec<Entry>>) {
    separated_pair(
        initial_seeds,
        tag("\n"),
        separated_list1(tag("\n"), mapping),
    )(input)
    .unwrap()
    .1
}

fn main() {
    let input = include_str!("../../data/day05.txt");
    let (seeds, mappings) = parse_input(input);
}
