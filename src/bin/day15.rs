fn apply_hash(string: &[u8]) -> u8 {
    string
        .iter()
        .fold(0, |acc, &x| acc.wrapping_add(x).wrapping_mul(17))
}

fn parse_input(input: &str) -> Vec<&[u8]> {
    input.trim().split(',').map(|s| s.as_bytes()).collect()
}

fn solve_part1(data: &[&[u8]]) -> u64 {
    data.iter().map(|&s| apply_hash(s) as u64).sum()
}

fn main() {
    let input = include_str!("../../data/day15.txt");
    let data = parse_input(input);
    let answer1 = solve_part1(&data);
    println!("The answer to part 1 is {}", answer1);
}

#[cfg(test)]
mod test {
    use crate::{apply_hash, parse_input, solve_part1};

    #[test]
    fn test_apply_hash() {
        assert_eq!(apply_hash(b"HASH"), 52);
        assert_eq!(apply_hash(b"rn=1"), 30);
        assert_eq!(apply_hash(b"ot=7"), 231);
    }

    #[test]
    fn test_solve_part1() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7\n";
        let data = parse_input(input);
        assert_eq!(solve_part1(&data), 1320);
    }
}
