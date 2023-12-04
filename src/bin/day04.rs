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
        .map(|c| {
            let wins = c.win_count();
            match wins {
                0 => 0,
                _ => 1 << (wins - 1),
            }
        })
        .sum()
}

fn main() {
    let input = include_str!("../../data/day04.txt");
    let data = parse_input(input);
    let answer1 = solve_part1(&data);
    println!("The answer to part 1 is {}", answer1);
}
