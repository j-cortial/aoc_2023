#[derive(Debug)]
struct City {
    blocks: Vec<Vec<u8>>,
}

fn parse_input(input: &str) -> City {
    City {
        blocks: input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect(),
    }
}

fn main() {
    let input = include_str!("../../data/day17.txt");
    let city = parse_input(input);
    dbg!(&city);
}
