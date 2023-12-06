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

#[derive(Debug, Clone, Copy)]
struct Interval {
    begin: Id,
    range: Id,
}

impl Interval {
    fn from_slice(data: &[Id]) -> Self {
        Interval {
            begin: data[0],
            range: data[1],
        }
    }
}

impl Interval {
    fn try_merge(&self, other: &Interval) -> Option<Interval> {
        if other.begin >= self.begin && other.begin <= self.begin + self.range {
            return Some(Interval {
                begin: self.begin,
                range: self.range.max((other.begin - self.begin) + other.range),
            });
        }
        None
    }
}

fn into_intervals(seed_data: &[Id]) -> Vec<Interval> {
    seed_data
        .chunks_exact(2)
        .map(Interval::from_slice)
        .collect()
}

#[derive(Debug)]
struct Collection {
    intervals: Vec<Interval>,
}

impl Collection {
    fn new(mut intervals: Vec<Interval>) -> Self {
        intervals.sort_unstable_by_key(|i| i.begin);
        let steps = intervals.len() - 1;
        for i in (0..steps).rev() {
            if let Some(merged) = intervals[i].try_merge(&intervals[i + 1]) {
                intervals.swap_remove(i + 1);
                intervals[i] = merged;
            }
        }
        Collection { intervals }
    }
}

impl Mapping {
    fn apply_n(&self, source: Interval) -> Vec<Interval> {
        match self.entries.binary_search_by(|e| e.compare(source.begin)) {
            Ok(i) => self.push_transform(i, source, vec![]),
            Err(i) => self.push_direct(i, source, vec![]),
        }
    }

    fn push_direct(&self, i: usize, source: Interval, mut acc: Vec<Interval>) -> Vec<Interval> {
        if self.entries.len() <= i {
            acc.push(source);
            return acc;
        }
        let range = source.range.min(self.entries[i].source - source.begin);
        if range > 0 {
            acc.push(Interval {
                begin: source.begin,
                range,
            });
        }
        if range < source.range {
            return self.push_transform(
                i,
                Interval {
                    begin: self.entries[i].source,
                    range: source.range - range,
                },
                acc,
            );
        }
        acc
    }

    fn push_transform(&self, i: usize, source: Interval, mut acc: Vec<Interval>) -> Vec<Interval> {
        let delta = source.begin - self.entries[i].source;
        let range = source.range.min(self.entries[i].range - delta);
        acc.push(Interval {
            begin: self.entries[i].target + delta,
            range,
        });
        if range < source.range {
            return self.push_direct(
                i + 1,
                Interval {
                    begin: source.begin + range,
                    range: source.range - range,
                },
                acc,
            );
        }
        acc
    }
}

fn solve_part2(mut seeds: Vec<Interval>, mappings: &[Mapping]) -> Id {
    for m in mappings {
        seeds = seeds
            .into_iter()
            .flat_map(|s| m.apply_n(s).into_iter())
            .collect();
        seeds = Collection::new(seeds).intervals;
    }
    seeds[0].begin
}

fn main() {
    let input = include_str!("../../data/day05.txt");
    let (seeds, mappings) = parse_input(input);
    let answer1 = solve_part1(&seeds, &mappings);
    println!("The answer to part 1 is {}", answer1);
    let answer2 = solve_part2(into_intervals(&seeds), &mappings);
    println!("The answer to part 2 is {}", answer2);
}
