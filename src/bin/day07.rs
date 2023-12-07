use std::str::FromStr;

use counter::Counter;
use nom::{
    character::complete::{char, multispace1, one_of, space1},
    combinator::{map, map_res, recognize},
    multi::{many0, many1, many_m_n, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};

type Money = u32;

#[derive(Debug, Clone, Copy)]
struct Cards(&'static str);

const RANKS: &str = "23456789TJQKA";
const ALT_RANKS: &str = "J23456789TQKA";

#[derive(Debug, Clone)]
struct Hand {
    cards: Cards,
    bet: Money,
}

fn integer<I: FromStr>(input: &str) -> IResult<&str, I> {
    map_res(
        recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))),
        |s: &str| s.parse(),
    )(input)
}

fn parse_input(input: &'static str) -> Vec<Hand> {
    separated_list1(
        multispace1,
        map(
            separated_pair(
                recognize(many_m_n(5, 5, one_of(RANKS))),
                space1,
                integer::<Money>,
            ),
            |(cards, bet)| Hand {
                cards: Cards(cards),
                bet,
            },
        ),
    )(input)
    .unwrap()
    .1
}

fn count(cards: &Cards) -> Counter<char> {
    cards.0.chars().collect()
}

fn ranks(cards: &Cards) -> Vec<usize> {
    cards
        .0
        .chars()
        .map(|c| RANKS.chars().position(|a| a == c).unwrap())
        .collect()
}

fn alt_ranks(cards: &Cards) -> Vec<usize> {
    cards
        .0
        .chars()
        .map(|c| ALT_RANKS.chars().position(|a| a == c).unwrap())
        .collect()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Strength {
    counts: Vec<usize>,
    ranks: Vec<usize>,
}

impl Hand {
    fn strength(&self) -> Strength {
        Strength {
            counts: count(&self.cards)
                .most_common()
                .into_iter()
                .map(|(_, c)| c)
                .collect(),
            ranks: ranks(&self.cards),
        }
    }

    fn alt_strength(&self) -> Strength {
        let counts = count(&self.cards);
        let jokers = counts.get(&'J').copied().unwrap_or_default();
        let mut counts: Vec<_> = counts
            .most_common()
            .into_iter()
            .filter_map(|(c, n)| match c {
                'J' => None,
                _ => Some(n),
            })
            .collect();
        if counts.is_empty() {
            counts.push(0);
        }
        counts[0] += jokers;
        Strength {
            counts,
            ranks: alt_ranks(&self.cards),
        }
    }
}

fn solve_part1(hands: &[Hand]) -> usize {
    let sorted_hands = {
        let mut hands = hands.to_vec();
        hands.sort_unstable_by_key(Hand::strength);
        hands
    };
    sorted_hands
        .into_iter()
        .enumerate()
        .map(|(i, h)| (i + 1) * h.bet as usize)
        .sum()
}

fn solve_part2(hands: &[Hand]) -> usize {
    let sorted_hands = {
        let mut hands = hands.to_vec();
        hands.sort_unstable_by_key(Hand::alt_strength);
        hands
    };
    sorted_hands
        .into_iter()
        .enumerate()
        .map(|(i, h)| (i + 1) * h.bet as usize)
        .sum()
}

fn main() {
    let input = include_str!("../../data/day07.txt");
    let hands = parse_input(input);
    let answer1 = solve_part1(&hands);
    println!("The answer to part 1 is {}", answer1);
    let answer2 = solve_part2(&hands);
    println!("The answer to part 2 is {}", answer2);
}
