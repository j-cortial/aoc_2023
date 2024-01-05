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

fn main() {
    let input = include_str!("../../data/day25.txt");
    let data = parse_input(input);
    dbg!(&data);
}
