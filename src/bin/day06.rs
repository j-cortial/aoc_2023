use nom::{
    bytes::complete::tag,
    character::complete::{char, multispace0, multispace1, one_of},
    combinator::{map_res, recognize},
    multi::{many0, many1, separated_list1},
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

#[derive(Debug)]
struct Race {
    time: i32,
    distance: i32,
}

fn integer(input: &str) -> IResult<&str, i32> {
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
    race_time: i32,
    hold_time: i32,
}

impl Outcome {
    fn distance(&self) -> i32 {
        if self.hold_time <= 0 || self.hold_time >= self.race_time {
            return 0;
        }
        (self.race_time - self.hold_time) * self.hold_time
    }
}

fn solve_part1(races: &[Race]) -> usize {
    races
        .iter()
        .map(|r| {
            (1..r.time)
                .map(|h| {
                    Outcome {
                        race_time: r.time,
                        hold_time: h,
                    }
                    .distance()
                })
                .filter(|&d| d > r.distance)
                .count()
        })
        .product()
}

fn main() {
    let input = include_str!("../../data/day06.txt");
    let races = parse_input(input);
    let answer1 = solve_part1(&races);
    println!("The answer to part 1 is {}", answer1);
}
