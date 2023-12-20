use std::{collections::HashMap, str::FromStr};

use nom::{
    branch::alt,
    character::complete::{alpha1, anychar, char, newline, one_of},
    combinator::{map, map_res, recognize},
    multi::{many0, many1, separated_list1},
    sequence::{delimited, pair, separated_pair, terminated, tuple},
    IResult,
};
use strum::EnumCount;

#[derive(Debug, Clone, Copy, EnumCount)]
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
            'a' => Ok(Category::A),
            's' => Ok(Category::S),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Relation {
    Greater,
    Less,
}

type Rating = u16;

#[derive(Debug, Clone)]
struct Condition {
    category: Category,
    relation: Relation,
    threshold: Rating,
}

impl Condition {
    fn holds_for(&self, part: &Part) -> bool {
        match self.relation {
            Relation::Greater => part.ratings[self.category as usize] > self.threshold,
            Relation::Less => part.ratings[self.category as usize] < self.threshold,
        }
    }
}

type WorkflowId = &'static str;

#[derive(Debug, Clone, Copy)]
enum Fate {
    Accept,
    Reject,
    Forward(WorkflowId),
}

#[derive(Debug, Clone)]
struct Workflow {
    default: Fate,
    logic: Vec<(Condition, Fate)>,
}

impl Workflow {
    fn process(&self, part: &Part) -> Fate {
        self.logic
            .iter()
            .find(|(condition, _)| condition.holds_for(part))
            .map(|(_, fate)| *fate)
            .unwrap_or(self.default)
    }
}

#[derive(Debug)]
struct Part {
    ratings: [Rating; Category::COUNT],
}

struct Oracle {
    workflows: HashMap<WorkflowId, Workflow>,
}

impl Oracle {
    fn new<'a>(workflows: impl Iterator<Item = &'a (WorkflowId, Workflow)>) -> Self {
        Self {
            workflows: workflows.cloned().collect(),
        }
    }

    fn is_valid(&self, part: &Part) -> bool {
        let workflow_id = &mut "in";
        loop {
            let workflow = self.workflows.get(workflow_id).unwrap();
            let next_id: &'static str = match workflow.process(part) {
                Fate::Accept => return true,
                Fate::Reject => return false,
                Fate::Forward(next_id) => &next_id,
            };
            *workflow_id = next_id;
        }
    }
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

fn rating_component(input: &str) -> IResult<&str, (Category, Rating)> {
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
                map(separated_list1(char(','), rating_component), |v| Part {
                    ratings: [v[0].1, v[1].1, v[2].1, v[3].1],
                }),
                char('}'),
            ),
        ),
    )(input)
    .unwrap()
    .1
}

fn solve_part1(workflows: &[(WorkflowId, Workflow)], parts: &[Part]) -> u64 {
    let oracle = Oracle::new(workflows.iter());
    parts
        .iter()
        .filter(|&part| oracle.is_valid(part))
        .map(|part| part.ratings.iter().sum::<Rating>() as u64)
        .sum()
}

fn main() {
    let input = include_str!("../../data/day19.txt");
    let (workflows, parts) = parse_input(input);
    let answer1 = solve_part1(&workflows, &parts);
    println!("The answer to part 1 is {}", answer1);
}

#[cfg(test)]
mod test {
    use crate::{parse_input, solve_part1};

    const INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test_solve_part1() {
        let (workflows, parts) = parse_input(INPUT);
        let answer1 = solve_part1(&workflows, &parts);
        assert_eq!(answer1, 19114);
    }
}
