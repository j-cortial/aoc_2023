use std::str::FromStr;

use itertools::Itertools;
use nom::{
    character::complete::{anychar, char, newline, one_of, space1},
    combinator::{map, map_res, recognize},
    multi::{many0, many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};
use strum::EnumIs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIs)]
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

fn checksum(conditions: &[Condition]) -> Option<Vec<u8>> {
    if conditions.contains(&Condition::Unknown) {
        return None;
    }
    Some(
        conditions
            .split(|condition| condition.is_operational())
            .map(|slice| slice.len() as u8)
            .filter(|&len| len != 0)
            .collect(),
    )
}

fn restore(conditions: &[Condition], guess: &[Condition]) -> Vec<Condition> {
    let mut res = conditions.to_vec();
    res.iter_mut()
        .filter(|condition| condition.is_unknown())
        .zip_eq(guess.iter())
        .for_each(|(c, &g)| *c = g);
    res
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

impl Record {
    fn valid_combinations<'a>(&'a self) -> impl Iterator<Item = Vec<Condition>> + 'a {
        let missing_damaged_count = self.checksum.iter().sum::<u8>() as usize
            - self.conditions.iter().filter(|c| c.is_damaged()).count();
        let unknown_count = self.conditions.iter().filter(|c| c.is_unknown()).count();
        (0..unknown_count)
            .combinations(missing_damaged_count)
            .map(move |indices| {
                let guess: Vec<_> = (0..unknown_count)
                    .map(|i| {
                        if indices.contains(&i) {
                            Condition::Damaged
                        } else {
                            Condition::Operational
                        }
                    })
                    .collect();
                restore(&self.conditions, &guess)
            })
            .filter(|candidate| &checksum(candidate).unwrap() == &self.checksum)
    }
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

fn solve_part1(records: &[Record]) -> usize {
    records
        .iter()
        .map(|record| record.valid_combinations().count())
        .sum()
}

fn main() {
    let input = include_str!("../../data/day12.txt");
    let records = parse_input(input);
    let answer1 = solve_part1(&records);
    println!("The answer to part 1 is {}", answer1);
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_checksum() {
        use Condition::*;
        assert_eq!(
            checksum(&[
                Operational,
                Damaged,
                Operational,
                Operational,
                Damaged,
                Damaged,
                Operational
            ]),
            Some(vec![1, 2])
        );
    }

    #[test]
    fn test_restore() {
        use Condition::*;
        assert_eq!(
            restore(
                &[
                    Operational,
                    Damaged,
                    Unknown,
                    Unknown,
                    Unknown,
                    Damaged,
                    Unknown
                ],
                &[Operational, Operational, Damaged, Operational]
            ),
            vec![
                Operational,
                Damaged,
                Operational,
                Operational,
                Damaged,
                Damaged,
                Operational
            ]
        );
    }

    #[test]
    fn test_valid_combinations() {
        use Condition::*;
        assert_eq!(
            Record {
                conditions: vec![
                    Unknown,
                    Unknown,
                    Unknown,
                    Operational,
                    Damaged,
                    Damaged,
                    Damaged,
                ],
                checksum: vec![1, 1, 3],
            }
            .valid_combinations()
            .count(),
            1
        );
    }
}
