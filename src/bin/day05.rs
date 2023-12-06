use std::cmp::Ordering;

use nom::{
    bytes::complete::tag,
    character::complete::{char, none_of, one_of},
    combinator::{map, map_res, recognize},
    multi::{many0, many1, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    IResult,
};

type Id = u64;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Entry {
    source: Id,
    target: Id,
    range: Id,
}

struct Mapping {
    entries: Vec<Entry>,
}

impl Mapping {
    fn new(mut entries: Vec<Entry>) -> Self {
        entries.sort_unstable();
        Self { entries }
    }
}

fn integer(input: &str) -> IResult<&str, Id> {
    map_res(
        recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))),
        |s: &str| s.parse(),
    )(input)
}

fn entry(input: &str) -> IResult<&str, Entry> {
    map(separated_list1(tag(" "), integer), |v| Entry {
        target: v[0],
        source: v[1],
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

fn mapping(input: &str) -> IResult<&str, Mapping> {
    preceded(
        pair(many1(none_of(":")), tag(":\n")),
        map(many1(terminated(entry, tag("\n"))), Mapping::new),
    )(input)
}

fn parse_input(input: &str) -> (Vec<Id>, Vec<Mapping>) {
    separated_pair(
        initial_seeds,
        tag("\n"),
        separated_list1(tag("\n"), mapping),
    )(input)
    .unwrap()
    .1
}

impl Entry {
    fn compare(&self, source: Id) -> Ordering {
        if self.source + self.range <= source {
            return Ordering::Less;
        }
        if self.source > source {
            return Ordering::Greater;
        }
        Ordering::Equal
    }
}

impl Mapping {
    fn apply(&self, source: Id) -> Id {
        match self.entries.binary_search_by(|e| e.compare(source)) {
            Ok(index) => {
                let entry = &self.entries[index];
                entry.target + (source - entry.source)
            }
            Err(_) => source,
        }
    }
}

fn solve_part1(seeds: &[Id], mappings: &[Mapping]) -> Id {
    seeds
        .iter()
        .map(|&s| mappings.iter().fold(s, |acc, m| m.apply(acc)))
        .min()
        .unwrap()
}

fn main() {
    let input = include_str!("../../data/day05.txt");
    let (seeds, mappings) = parse_input(input);
    let answer1 = solve_part1(&seeds, &mappings);
    println!("The answer to part 1 is {}", answer1);
}
