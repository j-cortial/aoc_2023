use std::{collections::HashSet, convert::identity};

type Coord = i16;
type Loc = [Coord; 2];

#[derive(Debug)]
struct Garden {
    open_plots: HashSet<Loc>,
}

fn parse_input(input: &str) -> (Garden, Loc) {
    let (plots, start): (Vec<_>, Vec<_>) = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().map(move |(col, tile)| {
                let loc = [row as Coord, col as Coord];
                match tile {
                    '.' => (Some(loc), None),
                    '#' => (None, None),
                    'S' => (Some(loc), Some(loc)),
                    _ => panic!(),
                }
            })
        })
        .unzip();
    (
        Garden {
            open_plots: plots.into_iter().filter_map(identity).collect(),
        },
        start.into_iter().filter_map(identity).next().unwrap(),
    )
}

fn main() {
    let input = include_str!("../../data/day21.txt");
    let (garden, start)= parse_input(input);
    dbg!(&garden);
    dbg!(&start);
}
