use nom::{
    bytes::complete::tag,
    character::complete::{char, multispace0, multispace1, one_of},
    combinator::{map_res, recognize},
    multi::{many0, many1, separated_list1},
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

#[derive(Debug, Default)]
struct Race {
    time: u64,
    distance: u64,
}

fn integer(input: &str) -> IResult<&str, u64> {
    map_res(
        recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))),
        |s: &str| s.parse(),
    )(input)
}

fn parse_input(input: &str) -> Vec<Race> {
    let (times, distances) = separated_pair(
        preceded(
            tag("Time:"),
            preceded(multispace0, separated_list1(multispace1, integer)),
        ),
        multispace1,
        preceded(
            tag("Distance:"),
            preceded(multispace0, separated_list1(multispace1, integer)),
        ),
    )(input)
    .unwrap()
    .1;
    times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

struct Outcome {
    race_time: u64,
    hold_time: u64,
}

impl Outcome {
    fn distance(&self) -> u64 {
        if self.hold_time <= 0 || self.hold_time >= self.race_time {
            return 0;
        }
        (self.race_time - self.hold_time) * self.hold_time
    }
}

fn record_count(race: &Race) -> usize {
    (1..race.time)
        .map(|h| {
            Outcome {
                race_time: race.time,
                hold_time: h,
            }
            .distance()
        })
        .filter(|&d| d > race.distance)
        .count()
}

fn solve_part1(races: &[Race]) -> usize {
    races.iter().map(record_count).product()
}

fn join(head: u64, tail: u64) -> u64 {
    let mut offset = 10;
    let mut rem = tail / 10;
    while rem > 0 {
        offset *= 10;
        rem /= 10;
    }
    tail + offset * head
}

impl Race {
    fn collapse(races: &[Self]) -> Self {
        races.iter().fold(Race::default(), |acc, x| Race {
            time: join(acc.time, x.time),
            distance: join(acc.distance, x.distance),
        })
    }
}

fn solve_part2(races: &[Race]) -> usize {
    record_count(&Race::collapse(races))
}

fn main() {
    let input = include_str!("../../data/day06.txt");
    let races = parse_input(input);
    let answer1 = solve_part1(&races);
    println!("The answer to part 1 is {}", answer1);
    let answer2 = solve_part2(&races);
    println!("The answer to part 2 is {}", answer2);
}
