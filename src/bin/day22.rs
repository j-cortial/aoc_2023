type Coord = i16;
type Loc = [Coord; 3];

#[derive(Debug)]
struct Brick {
    ends: [Loc; 2],
}

impl Brick {
    fn new(first: Loc, second: Loc) -> Self {
        Self {
            ends: [first, second],
        }
    }
}

fn parse_loc(input: &str) -> Loc {
    let mut iter = input.split(',').map(|s| s.parse().unwrap());
    [
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    ]
}

fn parse_input(input: &str) -> Vec<Brick> {
    input
        .lines()
        .map(|line| {
            line.split_once('~')
                .map(|(f, s)| Brick::new(parse_loc(f), parse_loc(s)))
                .unwrap()
        })
        .collect()
}

fn main() {
    let input = include_str!("../../data/day22.txt");
    let bricks = parse_input(input);
    dbg!(&bricks);
}
