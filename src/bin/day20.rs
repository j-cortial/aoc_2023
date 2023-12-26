use std::{collections::HashMap, iter::once};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, one_of},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{pair, separated_pair},
};

type ModuleId = &'static str;

#[derive(Debug, Clone, Copy)]
enum ModuleKind {
    Button,
    Broadcast,
    FlipFlop,
    Conjunction,
}

#[derive(Debug)]
struct Module {
    kind: ModuleKind,
    destinations: Vec<ModuleId>,
}

#[derive(Debug)]
struct Network {
    modules: HashMap<ModuleId, Module>,
}

fn parse_input(input: &'static str) -> Network {
    map(
        separated_list1(
            newline::<&str, ()>,
            map(
                separated_pair(
                    pair(opt(one_of("&%")), alpha1),
                    tag(" -> "),
                    separated_list1(tag(", "), alpha1),
                ),
                |((sigil, id), destinations)| {
                    let kind = match sigil {
                        Some(c) => match c {
                            '%' => ModuleKind::FlipFlop,
                            '&' => ModuleKind::Conjunction,
                            _ => panic!(),
                        },
                        None => ModuleKind::Broadcast,
                    };
                    let module = Module { kind, destinations };
                    (id, module)
                },
            ),
        ),
        |entries| Network {
            modules: once((
                "button",
                Module {
                    kind: ModuleKind::Button,
                    destinations: vec!["broadcaster"],
                },
            ))
            .chain(entries.into_iter())
            .collect(),
        },
    )(input)
    .unwrap()
    .1
}

fn main() {
    let input = include_str!("../../data/day20.txt");
    let network = parse_input(input);
    dbg!(&network);
}
