use aho_corasick::AhoCorasick;
use std::collections::HashMap;

fn parse(input: &str) -> Vec<&str> {
    input.split_terminator('\n').collect()
}

fn calibration_value_part1(string: &str) -> u32 {
    let mut digits = string.chars().filter_map(|c| c.to_digit(10));
    let first_digit = digits.next().unwrap();
    let last_digit = digits.last().unwrap_or(first_digit);
    first_digit * 10 + last_digit
}

fn solve_part1(data: &[&str]) -> u32 {
    data.iter().map(|s| calibration_value_part1(s)).sum()
}

fn calibration_value_part2(string: &str) -> u32 {
    let words = HashMap::<&str, u32>::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);
    let keys: Vec<_> = words.keys().copied().collect();
    let ac = AhoCorasick::new(&keys).unwrap();
    let mut digits = ac.find_iter(string);
    let first_digit = words
        .get(keys[digits.next().map(|m| m.pattern()).unwrap()])
        .unwrap();
    let last_digit = digits
        .last()
        .map(|m| m.pattern())
        .map(|id| keys[id])
        .map(|k| words.get(k).unwrap())
        .unwrap_or(first_digit);
    first_digit * 10 + last_digit
}

fn solve_part2(data: &[&str]) -> u32 {
    data.iter().map(|s| calibration_value_part2(s)).sum()
}

fn main() {
    let input = include_str!("../../data/day01.txt");
    let data = parse(input);
    let answer1 = solve_part1(&data);
    println!("The answer to part 1 is {}", answer1);
    let answer2 = solve_part2(&data);
    println!("The answer to part 2 is {}", answer2);
}
