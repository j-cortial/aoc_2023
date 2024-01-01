type Coord = i64;

type Loc<const D: usize> = [Coord; D];

fn parse_loc3(input: &str) -> Loc<3> {
    let mut iter = input.split(", ");
    [
        iter.next().map(|s| s.parse()).unwrap().unwrap(),
        iter.next().map(|s| s.parse()).unwrap().unwrap(),
        iter.next().map(|s| s.parse()).unwrap().unwrap(),
    ]
}

fn parse_input(input: &str) -> Vec<(Loc<3>, Loc<3>)> {
    input
        .lines()
        .map(|line| {
            line.split_once(" @ ")
                .map(|(p, v)| (parse_loc3(p), parse_loc3(v)))
                .unwrap()
        })
        .collect()
}

fn main() {
    let input = include_str!("../../data/day24.txt");
    let data = parse_input(input);
    dbg!(&data);
}
