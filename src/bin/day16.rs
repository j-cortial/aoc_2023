enum Tile {
    Empty,
    Slash,
    Backslash,
    Dash,
    Pipe,
}

impl Tile {
    fn parse(c: char) -> Option<Self> {
        match c {
            '.' => Some(Self::Empty),
            '/' => Some(Self::Slash),
            '\\' => Some(Self::Backslash),
            '-' => Some(Self::Dash),
            '|' => Some(Self::Pipe),
            _ => None,
        }
    }
}

struct Layout {
    tiles: Vec<Vec<Tile>>,
}

fn parse_input(input: &str) -> Layout {
    Layout {
        tiles: input
            .lines()
            .map(|line| line.chars().map(|c| Tile::parse(c).unwrap()).collect())
            .collect(),
    }
}

fn main() {
    let input = include_str!("../../data/day16.txt");
    let layout = parse_input(input);
}
