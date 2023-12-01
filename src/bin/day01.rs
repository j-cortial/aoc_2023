fn parse(input: &str) -> Vec<&str> {
    input.split_terminator('\n').collect()
}

fn calibration_value(string: &str) -> u32 {
    let mut digits = string.chars().filter_map(|c| c.to_digit(10));
    let first_digit = digits.next().unwrap();
    let last_digit = digits.last().unwrap_or(first_digit);
    first_digit * 10 + last_digit
}

fn solve_part1(data: &[&str]) -> u32 {
    data.iter().map(|s| calibration_value(s)).sum()
}

fn main() {
    let input = include_str!("../../data/day01.txt");
    let data = parse(input);
    let answer1 = solve_part1(&data);
    println!("The answer to part 1 is {}", answer1);
}
