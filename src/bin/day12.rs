use std::str::FromStr;

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

#[derive(Debug)]
struct Record {
    conditions: Vec<Condition>,
    checksum: Vec<usize>,
}

fn integer<I: FromStr>(input: &str) -> IResult<&str, I> {
    map_res(
        recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))),
        |s: &str| s.parse(),
    )(input)
}

#[derive(Debug)]
struct Status {
    conditions: Vec<Condition>,
    checksum: Vec<usize>,
    damaged_count: usize,
    upper_bound: usize,
    lower_bound: usize,
}

impl Record {
    fn valid_combinations(&self) -> usize {
        let damaged_count = self.checksum.iter().sum::<usize>();
        let upper_bound = self
            .conditions
            .iter()
            .filter(|c| !c.is_operational())
            .count();
        let lower_bound = self.conditions.iter().filter(|c| c.is_damaged()).count();
        let status = Status {
            conditions: self.conditions.clone(),
            checksum: self.checksum.clone(),
            damaged_count,
            upper_bound,
            lower_bound,
        };
        status.valid_combinations()
    }
}

impl Status {
    fn valid_combinations(mut self) -> usize {
        if self.damaged_count < self.lower_bound || self.damaged_count > self.upper_bound {
            return 0;
        }
        if self.damaged_count == 0 {
            return 1;
        }
        let tail = self.conditions.last().unwrap();
        match tail {
            Condition::Damaged => {
                let checksum_tail = *self.checksum.last().unwrap();
                let (damaged_count, unknown_count) =
                    self.conditions.iter().rev().take(checksum_tail).fold(
                        (0, 0),
                        |acc, c| match c {
                            Condition::Operational => acc,
                            Condition::Damaged => (acc.0 + 1, acc.1),
                            Condition::Unknown => (acc.0, acc.1 + 1),
                        },
                    );
                if damaged_count + unknown_count != checksum_tail {
                    return 0;
                }
                if checksum_tail == self.conditions.len() {
                    return 1;
                }
                let additional_unknown = match self
                    .conditions
                    .get((self.conditions.len() - 1) - checksum_tail)
                    .unwrap()
                {
                    Condition::Operational => 0,
                    Condition::Damaged => {
                        return 0;
                    }
                    Condition::Unknown => 1,
                };
                self.checksum.pop();
                self.conditions
                    .truncate(self.conditions.len() - (checksum_tail + 1));
                Self {
                    damaged_count: self.damaged_count - checksum_tail,
                    upper_bound: self.upper_bound - (checksum_tail + additional_unknown),
                    lower_bound: self.lower_bound - damaged_count,
                    ..self
                }
                .valid_combinations()
            }
            Condition::Operational => {
                self.conditions.pop();
                Self { ..self }
            }
            .valid_combinations(),
            Condition::Unknown => {
                let mut conditions_damaged = self.conditions.clone();
                *conditions_damaged.last_mut().unwrap() = Condition::Damaged;
                let mut conditions_operational = self.conditions;
                conditions_operational.pop();
                Self {
                    conditions: conditions_damaged,
                    lower_bound: self.lower_bound + 1,
                    checksum: self.checksum.clone(),
                    ..self
                }
                .valid_combinations()
                    + Self {
                        conditions: conditions_operational,
                        upper_bound: self.upper_bound - 1,
                        ..self
                    }
                    .valid_combinations()
            }
        }
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
    records.iter().map(|r| r.valid_combinations()).sum()
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
            .valid_combinations(),
            1
        );
    }
}
