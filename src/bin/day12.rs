use std::{collections::HashMap, str::FromStr};

use nom::{
    character::complete::{anychar, char, newline, one_of, space1},
    combinator::{map, map_res, recognize},
    multi::{many0, many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};
use strum::EnumIs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIs, Hash)]
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

#[derive(Debug, Default)]
struct Memoization(HashMap<(Vec<Condition>, Vec<u8>), usize>);

impl Memoization {
    fn get(&self, key: &(Vec<Condition>, Vec<u8>)) -> Option<usize> {
        self.0.get(key).copied()
    }

    fn set(&mut self, key: (Vec<Condition>, Vec<u8>), value: usize) {
        self.0.insert(key, value);
    }
}

#[derive(Debug)]
struct Status {
    conditions: Vec<Condition>,
    checksum: Vec<u8>,
    damaged_count: usize,
    upper_bound: usize,
    lower_bound: usize,
}

impl Record {
    fn valid_combinations(&self, memo: &mut Memoization) -> usize {
        let damaged_count = self.checksum.iter().fold(0, |acc, x| acc + *x as usize);
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
        status.valid_combinations(memo)
    }

    fn unfold(self) -> Self {
        Self {
            conditions: vec![&self.conditions[..]; 5].join(&Condition::Unknown),
            checksum: self.checksum.repeat(5),
            ..self
        }
    }
}

impl Status {
    fn valid_combinations(mut self, memo: &mut Memoization) -> usize {
        let key = (self.conditions, self.checksum);
        if let Some(res) = memo.get(&key) {
            return res;
        }
        self.conditions = key.0.clone();
        self.checksum = key.1.clone();
        if self.damaged_count < self.lower_bound || self.damaged_count > self.upper_bound {
            memo.set(key, 0);
            return 0;
        }
        if self.damaged_count == 0 {
            memo.set(key, 1);
            return 1;
        }
        let tail = self.conditions.last().unwrap();
        match tail {
            Condition::Damaged => {
                let checksum_tail = *self.checksum.last().unwrap() as usize;
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
                    memo.set(key, 0);
                    return 0;
                }
                if checksum_tail == self.conditions.len() {
                    memo.set(key, 1);
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
                let res = Self {
                    damaged_count: self.damaged_count - checksum_tail,
                    upper_bound: self.upper_bound - (checksum_tail + additional_unknown),
                    lower_bound: self.lower_bound - damaged_count,
                    ..self
                }
                .valid_combinations(memo);
                memo.set(key, res);
                res
            }
            Condition::Operational => {
                self.conditions.pop();
                let res = Self { ..self }.valid_combinations(memo);
                memo.set(key, res);
                res
            }
            Condition::Unknown => {
                let mut conditions_damaged = self.conditions.clone();
                *conditions_damaged.last_mut().unwrap() = Condition::Damaged;
                let mut conditions_operational = self.conditions;
                conditions_operational.pop();
                let res = Self {
                    conditions: conditions_damaged,
                    lower_bound: self.lower_bound + 1,
                    checksum: self.checksum.clone(),
                    ..self
                }
                .valid_combinations(memo)
                    + Self {
                        conditions: conditions_operational,
                        upper_bound: self.upper_bound - 1,
                        ..self
                    }
                    .valid_combinations(memo);
                memo.set(key, res);
                res
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

fn solve(records: &[Record], memo: &mut Memoization) -> usize {
    records.iter().map(|r| r.valid_combinations(memo)).sum()
}

fn main() {
    let input = include_str!("../../data/day12.txt");
    let records = parse_input(input);
    let mut memo = Memoization::default();
    let answer1 = solve(&records, &mut memo);
    println!("The answer to part 1 is {}", answer1);
    let records: Vec<_> = records.into_iter().map(|r| r.unfold()).collect();
    let answer2 = solve(&records, &mut memo);
    println!("The answer to part 2 is {}", answer2);
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
            .valid_combinations(&mut Memoization::default()),
            1
        );
    }

    #[test]
    fn test_valid_combinations_unfolded() {
        use Condition::*;
        assert_eq!(
            Record {
                conditions: vec![
                    Unknown, Damaged, Damaged, Damaged, Unknown, Unknown, Unknown, Unknown,
                    Unknown, Unknown, Unknown, Unknown,
                ],
                checksum: vec![3, 2, 1],
            }
            .unfold()
            .valid_combinations(&mut Memoization::default()),
            506250
        );
    }
}
