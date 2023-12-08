use nom::{
    bytes::complete::tag,
    character::complete::{anychar, multispace1, one_of},
    combinator::recognize,
    multi::{many1, many_m_n, separated_list1},
    sequence::{delimited, separated_pair},
};

type Direction = char;
type Node = &'static str;

fn parse_input(input: &'static str) -> (Vec<Direction>, Vec<(Node, (Node, Node))>) {
    let node = |s| recognize(many_m_n(3, 3, anychar::<&str, ()>))(s);
    separated_pair(
        many1(one_of("LR")),
        multispace1,
        separated_list1(
            tag("\n"),
            separated_pair(
                node,
                tag(" = "),
                delimited(tag("("), separated_pair(node, tag(", "), node), tag(")")),
            ),
        ),
    )(input)
    .unwrap()
    .1
}

fn main() {
    let input = include_str!("../../data/day08.txt");
    let (directions, transitions) = parse_input(input);
    dbg!(&transitions);
    dbg!(&directions);
}
