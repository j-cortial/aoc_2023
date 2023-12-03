use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Location([isize; 2]);

type PartId = u32;

#[derive(Debug)]
struct Schematic {
    symbols: HashMap<Location, char>,
    numbers: Vec<(PartId, Location)>,
}

fn parse_input(input: &str) -> Schematic {
    let symbols = input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.' && !c.is_digit(10))
                .map(move |(j, c)| (Location([i as isize, j as isize]), c))
        })
        .collect();
    let numbers = input
        .lines()
        .enumerate()
        .flat_map(|(i, mut line)| {
            let mut nums = Vec::default();
            let mut j_offset = 0;
            while let Some(j) = line.chars().position(|c| c.is_digit(10)) {
                let (head, rem) = line.split_at(j);
                let mut it = rem.splitn(2, |c: char| !c.is_digit(10));
                let num = it.next().unwrap();
                nums.push((
                    num.parse().unwrap(),
                    Location([i as isize, (j + j_offset) as isize]),
                ));
                line = it.next().unwrap_or_default();
                j_offset += head.len() + num.len() + 1;
            }
            nums.into_iter()
        })
        .collect();
    Schematic { symbols, numbers }
}

impl Location {
    fn neighbors(&self) -> impl Iterator<Item = Location> + '_ {
        [
            [-1, -1],
            [-1, 0],
            [-1, 1],
            [0, -1],
            [0, 1],
            [1, -1],
            [1, 0],
            [1, 1],
        ]
        .into_iter()
        .map(|offset| Location([self.0[0] + offset[0], self.0[1] + offset[1]]))
    }
}

fn number_locations(mut number: PartId, mut head: Location) -> impl Iterator<Item = Location> {
    let mut res = Vec::default();
    res.push(head);
    number /= 10;
    while number > 0 {
        head.0[1] += 1;
        res.push(head);
        number /= 10;
    }
    res.into_iter()
}

impl Schematic {
    fn neighbors_symbol(&self, loc: &Location) -> bool {
        loc.neighbors().any(|l| self.symbols.contains_key(&l))
    }

    fn parts<'a>(&'a self) -> impl Iterator<Item = &'a PartId> {
        self.numbers
            .iter()
            .filter(|(id, loc)| number_locations(*id, *loc).any(|l| self.neighbors_symbol(&l)))
            .map(|(id, _)| id)
    }
}

fn solve_part1(schematic: &Schematic) -> u32 {
    schematic.parts().sum()
}

fn main() {
    let input = include_str!("../../data/day03.txt");
    let schematic = parse_input(input);
    let answer1 = solve_part1(&schematic);
    println!("The answer to part 1 is {}", answer1);
    //let answer2 = solve_part2();
    //println!("The answer to part 2 is {}", answer2);
}
