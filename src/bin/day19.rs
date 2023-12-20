use std::str::FromStr;

use nom::{
    branch::alt,
    character::complete::{alpha1, anychar, char, newline, one_of},
    combinator::{map, map_res, recognize},
    multi::{many0, many1, separated_list1},
    sequence::{delimited, pair, separated_pair, terminated, tuple},
    IResult,
};
use strum::{EnumCount, EnumIter, IntoEnumIterator};

#[derive(EnumCount, EnumIter)]
enum Category {
    X,
    M,
    A,
    S,
}

impl TryFrom<char> for Category {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'x' => Ok(Category::X),
            'm' => Ok(Category::M),
            's' => Ok(Category::A),
            'a' => Ok(Category::S),
            _ => Err(()),
        }
    }
}

enum Relation {
    Greater,
    Less,
}

type Rating = u16;

struct Condition {
    category: Category,
    relation: Relation,
    threshold: Rating,
}

type WorkflowId = &'static str;

enum Fate {
    Accept,
    Reject,
    Forward(WorkflowId),
}

struct Workflow {
    default: Fate,
    logic: Vec<(Condition, Fate)>,
}

struct Part {
    ratings: [Rating; 4],
}

fn integer<I: FromStr>(input: &str) -> IResult<&str, I> {
    map_res(
        recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))),
        |s: &str| s.parse(),
    )(input)
}

fn category(input: &str) -> IResult<&str, Category> {
    map_res(anychar, |c| Category::try_from(c))(input)
}

fn rating(input: &str) -> IResult<&str, (Category, Rating)> {
    separated_pair(category, char('='), integer::<Rating>)(input)
}

fn fate(input: &'static str) -> IResult<&str, Fate> {
    alt((
        map(char('A'), |_| Fate::Accept),
        map(char('R'), |_| Fate::Reject),
        map(alpha1, |s| Fate::Forward(s)),
    ))(input)
}

fn condition(input: &str) -> IResult<&str, Condition> {
    map(
        tuple((
            category,
            alt((
                map(char('>'), |_| Relation::Greater),
                map(char('<'), |_| Relation::Less),
            )),
            integer::<Rating>,
        )),
        |(category, relation, threshold)| Condition {
            category,
            relation,
            threshold,
        },
    )(input)
}

fn parse_input(input: &'static str) -> (Vec<(WorkflowId, Workflow)>, Vec<Part>) {
    separated_pair(
        separated_list1(
            newline,
            pair(
                alpha1,
                map(
                    delimited(
                        char('{'),
                        pair(
                            many1(terminated(
                                separated_pair(condition, char(':'), fate),
                                char(','),
                            )),
                            fate,
                        ),
                        char('}'),
                    ),
                    |(logic, default)| Workflow { default, logic },
                ),
            ),
        ),
        many1(newline),
        separated_list1(
            newline,
            delimited(
                char('{'),
                map(separated_list1(char(','), rating), |v| Part {
                    ratings: [v[0].1, v[1].1, v[2].1, v[3].1],
                }),
                char('}'),
            ),
        ),
    )(input)
    .unwrap()
    .1
}

fn main() {
    let input = include_str!("../../data/day19.txt");
    let (workflows, parts) = parse_input(input);
}
