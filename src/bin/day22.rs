use std::collections::{HashMap, HashSet};

type Coord = i16;
type Loc2 = [Coord; 2];
type Loc3 = [Coord; 3];

fn projection(loc: &Loc3) -> Loc2 {
    [loc[0], loc[1]]
}

fn z_coord(loc: &Loc3) -> Coord {
    loc[2]
}

#[derive(Debug)]
struct Brick {
    ends: [Loc3; 2],
}

impl Brick {
    fn new(first: Loc3, second: Loc3) -> Self {
        Self {
            ends: [first.min(second), first.max(second)],
        }
    }

    fn main_direction(&self) -> Option<usize> {
        (0..3).find(|&i| self.ends[0][i] != self.ends[1][i])
    }

    fn z_extents(&self) -> Vec<(Loc2, (Coord, Coord))> {
        match self.main_direction() {
            Some(0) => (self.ends[0][0]..=self.ends[1][0])
                .map(|i| ([i, self.ends[0][1]], (self.ends[0][2], self.ends[0][2])))
                .collect(),
            Some(1) => (self.ends[0][1]..=self.ends[1][1])
                .map(|j| ([self.ends[0][0], j], (self.ends[0][2], self.ends[0][2])))
                .collect(),
            Some(2) | None => vec![(
                projection(&self.ends[0]),
                (z_coord(&self.ends[0]), z_coord(&self.ends[1])),
            )],
            Some(_) => unreachable!(),
        }
    }
}

fn z_extents(bricks: &[Brick]) -> HashMap<Loc2, Vec<(usize, (Coord, Coord))>> {
    let mut res: HashMap<_, Vec<_>> = HashMap::new();
    for (rank, brick) in bricks.iter().enumerate() {
        for (col, extent) in brick.z_extents() {
            res.entry(col).or_default().push((rank, extent));
        }
    }
    for v in res.values_mut() {
        v.sort_unstable_by_key(|(_, (z_lower, _))| *z_lower);
    }
    res
}

fn parse_loc(input: &str) -> Loc3 {
    let mut iter = input.split(',').map(|s| s.parse().unwrap());
    [
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    ]
}

fn parse_input(input: &str) -> Vec<Brick> {
    input
        .lines()
        .map(|line| {
            line.split_once('~')
                .map(|(f, s)| Brick::new(parse_loc(f), parse_loc(s)))
                .unwrap()
        })
        .collect()
}

#[derive(Debug)]
struct Stack<'a> {
    bricks: &'a [Brick],
    extents: HashMap<Loc2, Vec<(usize, (Coord, Coord))>>,
}

impl<'a> Stack<'a> {
    fn new(bricks: &'a [Brick]) -> Self {
        Self {
            bricks,
            extents: z_extents(bricks),
        }
    }

    fn settle(&mut self) {
        let mut ranks: Vec<_> = self
            .bricks
            .iter()
            .enumerate()
            .map(|(i, b)| (i, b.ends[0][2]))
            .collect();
        ranks.sort_unstable_by_key(|(_, z_lowest)| *z_lowest);
        let mut offsets = vec![0; self.bricks.len()];
        loop {
            let mut no_change = true;
            for (index, _) in ranks.iter() {
                let mut gap = Coord::MAX;
                for (col, (z_low, _)) in self.bricks[*index].z_extents() {
                    let col_extents = &self.extents[&col];
                    let col_rank = col_extents
                        .binary_search_by_key(&z_low, |(_, (l, _))| *l)
                        .unwrap();
                    let z_support = if col_rank > 0 {
                        let (other_index, (_, other_high)) = col_extents[col_rank - 1];
                        other_high - offsets[other_index]
                    } else {
                        0
                    };
                    gap = gap.min((z_low - offsets[*index]) - (z_support + 1));
                }
                if gap > 0 {
                    no_change = false;
                    offsets[*index] += gap;
                }
            }
            if no_change {
                break;
            }
            ranks.sort_unstable_by_key(|(index, z_lowest)| *z_lowest - offsets[*index]);
        }
        for col in self.extents.values_mut() {
            for (index, (low, high)) in col {
                let offset = offsets[*index];
                *low -= offset;
                *high -= offset;
            }
        }
    }

    fn supports(&self) -> Vec<HashSet<usize>> {
        let mut res: Vec<_> = vec![HashSet::default(); self.bricks.len()];
        for col in self.extents.values() {
            for pair in col.windows(2) {
                if pair[0].1 .1 == pair[1].1 .0 - 1 {
                    res[pair[1].0].insert(pair[0].0);
                }
            }
        }
        res
    }

    fn dependencies(&self) -> Vec<HashSet<usize>> {
        let mut res: Vec<_> = vec![HashSet::default(); self.bricks.len()];
        for col in self.extents.values() {
            for pair in col.windows(2) {
                if pair[0].1 .1 == pair[1].1 .0 - 1 {
                    res[pair[0].0].insert(pair[1].0);
                }
            }
        }
        res
    }
}

fn solve_part1(bricks: &[Brick]) -> usize {
    let mut stack = Stack::new(bricks);
    stack.settle();
    let key_bricks: HashSet<_> = stack
        .supports()
        .into_iter()
        .filter_map(|supports| {
            if supports.len() == 1 {
                return supports.into_iter().next();
            }
            None
        })
        .collect();
    bricks.len() - key_bricks.len()
}

fn solve_part2(bricks: &[Brick]) -> usize {
    let mut stack = Stack::new(bricks);
    stack.settle();
    let supports = stack.supports();
    let key_bricks: HashSet<_> = supports
        .iter()
        .filter_map(|supports| {
            if supports.len() == 1 {
                return supports.into_iter().next();
            }
            None
        })
        .collect();
    let dependencies = stack.dependencies();
    key_bricks
        .into_iter()
        .map(|&root| {
            let mut removed = HashSet::from([root]);
            let mut front = removed.clone();
            loop {
                let falling: Vec<_> = front
                    .drain()
                    .flat_map(|falling_id| {
                        dependencies[falling_id]
                            .iter()
                            .copied()
                            .filter(|&candidate_id| supports[candidate_id].is_subset(&removed))
                    })
                    .collect();
                if falling.is_empty() {
                    break;
                }
                front.extend(falling.into_iter());
                removed.extend(front.iter())
            }
            removed.len() - 1
        })
        .sum()
}

fn main() {
    let input = include_str!("../../data/day22.txt");
    let bricks = parse_input(input);
    let answer1 = solve_part1(&bricks);
    println!("The answer to part 1 is {}", answer1);
    let answer2 = solve_part2(&bricks);
    println!("The answer to part 2 is {}", answer2);
}

#[cfg(test)]
mod test {
    use crate::*;

    const INPUT: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[test]
    fn test_solve_part1() {
        let answer = solve_part1(&parse_input(INPUT));
        assert_eq!(answer, 5);
    }
    #[test]
    fn test_solve_part2() {
        let answer = solve_part2(&parse_input(INPUT));
        assert_eq!(answer, 7);
    }
}
