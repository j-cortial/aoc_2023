use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Location([isize; 2]);

#[derive(Debug)]
struct Schematic {
    symbols: HashMap<Location, char>,
    numbers: Vec<(u32, Location)>,
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
            let mut nums = Vec::<(u32, Location)>::default();
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

fn main() {
    let input = include_str!("../../data/day03.txt");
    let schematic = parse_input(input);
    //let answer1 = solve_part1();
    //println!("The answer to part 1 is {}", answer1);
    //let answer2 = solve_part2();
    //println!("The answer to part 2 is {}", answer2);
}
