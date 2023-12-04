type Number = u16;

struct Card {
    winning_numbers: Vec<Number>,
    hand: Vec<Number>,
}

fn parse_input(input: &str) -> Vec<Card> {
    input
        .lines()
        .flat_map(|l| {
            l.split_once(':').map(|(_, l)| {
                l.trim()
                    .split_once('|')
                    .map(|(w, h)| Card {
                        winning_numbers: w
                            .trim()
                            .split_whitespace()
                            .map(|s| s.parse().unwrap())
                            .collect(),
                        hand: h
                            .trim()
                            .split_whitespace()
                            .map(|s| s.parse().unwrap())
                            .collect(),
                    })
                    .unwrap()
            })
        })
        .collect()
}

impl Card {
    fn win_count(&self) -> usize {
        self.hand
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count()
    }
}

fn solve_part1(data: &[Card]) -> usize {
    data.iter()
        .map(|c| match c.win_count() {
            0 => 0,
            wins => 1 << (wins - 1),
        })
        .sum()
}

fn solve_part2(data: &[Card]) -> usize {
    data.iter()
        .fold((0, vec![1; data.len()]), |(sum, mut copies), c| {
            let count = copies.pop().unwrap_or_default();
            let wins = c.win_count();
            copies.iter_mut().rev().take(wins).for_each(|c| {
                *c += count;
            });
            (sum + count, copies)
        })
        .0
}

fn main() {
    let input = include_str!("../../data/day04.txt");
    let data = parse_input(input);
    let answer1 = solve_part1(&data);
    println!("The answer to part 1 is {}", answer1);
    let answer2 = solve_part2(&data);
    println!("The answer to part 2 is {}", answer2);
}
