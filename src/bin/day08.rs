use gcd::Gcd;
use nom::{
    bytes::complete::tag,
    character::complete::{anychar, multispace1, one_of},
    combinator::{map, recognize},
    multi::{many1, many_m_n, separated_list1},
    sequence::{delimited, separated_pair},
};
use std::collections::HashMap;

type Direction = char;
type Node = &'static str;

fn parse_input(input: &'static str) -> (Vec<Direction>, HashMap<Node, (Node, Node)>) {
    let node = |s| recognize(many_m_n(3, 3, anychar::<&str, ()>))(s);
    separated_pair(
        many1(one_of("LR")),
        multispace1,
        map(
            separated_list1(
                tag("\n"),
                separated_pair(
                    node,
                    tag(" = "),
                    delimited(tag("("), separated_pair(node, tag(", "), node), tag(")")),
                ),
            ),
            |list| list.into_iter().collect(),
        ),
    )(input)
    .unwrap()
    .1
}

fn solve_part1(directions: &[Direction], transitions: &HashMap<Node, (Node, Node)>) -> usize {
    directions
        .iter()
        .cycle()
        .scan("AAA", |state, &dir| {
            if state == &"ZZZ" {
                return None;
            }
            let candidates = transitions.get(state).unwrap();
            *state = match dir {
                'L' => &candidates.0,
                'R' => &candidates.1,
                _ => panic!(),
            };
            Some(*state)
        })
        .count()
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / first.gcd(second)
}

fn solve_part2(directions: &[Direction], transitions: &HashMap<Node, (Node, Node)>) -> usize {
    let initial_states: Vec<_> = transitions
        .keys()
        .copied()
        .filter(|node| node.ends_with('A'))
        .collect();
    let periods: Vec<Vec<_>> = initial_states
        .into_iter()
        .map(|initial_state| {
            directions
                .iter()
                .cycle()
                .scan(initial_state, |state, &dir| {
                    let reached = Some(state.ends_with('Z'));
                    let candidates = transitions.get(state).unwrap();
                    *state = match dir {
                        'L' => &candidates.0,
                        'R' => &candidates.1,
                        _ => panic!(),
                    };
                    reached
                })
                .enumerate()
                .filter_map(|(i, b)| if b { Some(i) } else { None })
                .take(2)
                .collect()
        })
        .collect();
    periods.into_iter().fold(1, |acc, x| lcm(acc, x[1] - x[0]))
}

fn main() {
    let input = include_str!("../../data/day08.txt");
    let (directions, transitions) = parse_input(input);
    let answer1 = solve_part1(&directions, &transitions);
    println!("The answer to part 1 is {}", answer1);
    let answer2 = solve_part2(&directions, &transitions);
    println!("The answer to part 2 is {}", answer2);
}
