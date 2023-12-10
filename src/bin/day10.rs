use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Tile {
    Ground,
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

type Loc = (i64, i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn opposite(&self) -> Self {
        match *self {
            Direction::East => Direction::West,
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
    fn next(&self, loc: Loc) -> Loc {
        let delta = match *self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        };
        (loc.0 + delta.0, loc.1 + delta.1)
    }
}

impl Tile {
    fn directions(&self) -> Vec<Direction> {
        use Direction::*;
        use Tile::*;
        match *self {
            Ground => vec![],
            NorthSouth => vec![North, South],
            EastWest => vec![East, West],
            NorthEast => vec![North, East],
            NorthWest => vec![North, West],
            SouthWest => vec![South, West],
            SouthEast => vec![South, East],
        }
    }
}

#[derive(Debug)]
struct Grid {
    tiles: Vec<Vec<Tile>>,
}

impl Grid {
    fn new(tiles: Vec<Vec<Tile>>, start: Loc) -> Self {
        let mut res = Self { tiles };
        *res.tile_mut(start) = Tile::EastWest; // HACK
        res
    }

    fn tile(&self, loc: Loc) -> &Tile {
        &self.tiles[loc.0 as usize][loc.1 as usize]
    }

    fn tile_mut(&mut self, loc: Loc) -> &mut Tile {
        &mut self.tiles[loc.0 as usize][loc.1 as usize]
    }
}

fn parse_input(input: &str) -> (Grid, Loc) {
    use Tile::*;
    let mut start = Loc::default();
    let tiles = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '.' => Ground,
                    '|' => NorthSouth,
                    '-' => EastWest,
                    'L' => NorthEast,
                    'J' => NorthWest,
                    '7' => SouthWest,
                    'F' => SouthEast,
                    'S' => {
                        start = (i as i64, j as i64);
                        Ground
                    }
                    _ => panic!(),
                })
                .collect()
        })
        .collect();
    (Grid::new(tiles, start), start)
}

fn find_circuit (grid: &Grid, start: Loc) -> HashMap<Loc, usize> {
    let mut distances = HashMap::from([(start, 0)]);
    let mut front: Vec<_> = grid
        .tile(start)
        .directions()
        .into_iter()
        .map(|dir| (start, dir))
        .collect();
    while !front.is_empty() {
        let mut new_front = vec![];
        for (loc, dir) in front {
            let candidate = dir.next(loc);
            if !distances.contains_key(&candidate) {
                distances.insert(candidate, *distances.get(&loc).unwrap() + 1);
                new_front.push((
                    candidate,
                    grid.tile(candidate)
                        .directions()
                        .into_iter()
                        .filter(|&d| d != dir.opposite())
                        .next()
                        .unwrap(),
                ));
            }
        }
        front = new_front;
    }
    distances
}

fn solve_part1(circuit: &HashMap<Loc, usize>) -> usize {
    circuit.values().copied().max().unwrap()
}


fn main() {
    let input = include_str!("../../data/day10.txt");
    let (grid, start) = parse_input(input);
    let circuit = find_circuit(&grid, start);
    let answer1 = solve_part1(&circuit);
    println!("The answer to part 1 is {}", answer1);
}
