use std::{
    collections::{HashMap, HashSet},
    iter::once,
};

use rustworkx_core::{connectivity::stoer_wagner_min_cut, petgraph::graph::UnGraph};

fn parse_input(input: &str) -> Vec<(&str, Vec<&str>)> {
    input
        .lines()
        .map(|line| {
            line.split_once(": ")
                .map(|(left, right)| (left, right.split(' ').collect()))
                .unwrap()
        })
        .collect()
}

fn solve_part1(data: &[(&str, Vec<&str>)]) -> usize {
    let labels: HashSet<_> = data
        .iter()
        .flat_map(|(a, bs)| once(a).chain(bs.iter()).copied())
        .collect();
    let mut graph: UnGraph<&str, ()> = UnGraph::new_undirected();
    let nodes: HashMap<_, _> = labels
        .iter()
        .map(|label| (*label, graph.add_node(*label)))
        .collect();
    data.iter().for_each(|(start, ends)| {
        graph.extend_with_edges(ends.iter().map(|end| (nodes[*start], nodes[*end])));
    });
    let min_cut_res: Result<Option<(usize, Vec<_>)>, ()> = stoer_wagner_min_cut(&graph, |_| Ok(1));

    let (min_cut, partition) = min_cut_res.unwrap().unwrap();
    assert_eq!(min_cut, 3);
    partition.len() * (labels.len() - partition.len())
}

fn main() {
    let input = include_str!("../../data/day25.txt");
    let data = parse_input(input);
    let answer1 = solve_part1(&data);
    println!("The answer to part 1 is {answer1}");
}
