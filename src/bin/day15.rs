fn apply_hash(string: &[u8]) -> u8 {
    string
        .iter()
        .fold(0, |acc, &x| acc.wrapping_add(x).wrapping_mul(17))
}

#[derive(Debug, Clone, Copy)]
struct Lens<'a> {
    label: &'a [u8],
    focal_length: u8,
}

#[derive(Debug, Default, Clone)]
struct Box<'a> {
    lenses: Vec<Lens<'a>>,
}

impl<'a> Box<'a> {
    fn local_focusing_power(&self) -> u64 {
        self.lenses
            .iter()
            .enumerate()
            .map(|(i, lens)| ((i + 1) as u64) * lens.focal_length as u64)
            .sum()
    }
}

#[derive(Debug)]
enum Instruction<'a> {
    Set(&'a [u8], u8),
    Rm(&'a [u8]),
}

fn decode_instruction(instruction: &[u8]) -> Instruction {
    if instruction.ends_with(b"-") {
        return Instruction::Rm(instruction.split_last().unwrap().1);
    }
    let (&focal_length, head) = instruction.split_last().unwrap();
    let (_, label) = head.split_last().unwrap();
    Instruction::Set(label, focal_length - b'0')
}

fn parse_input(input: &str) -> Vec<&[u8]> {
    input.trim().split(',').map(|s| s.as_bytes()).collect()
}

fn solve_part1(data: &[&[u8]]) -> u64 {
    data.iter().map(|&s| apply_hash(s) as u64).sum()
}

fn solve_part2(instructions: &[&[u8]]) -> u64 {
    let mut boxes: Vec<Box> = vec![Default::default(); 256];
    for &instruction in instructions {
        let cmd = decode_instruction(instruction);
        match cmd {
            Instruction::Set(label, focal_length) => {
                let box_id = apply_hash(label) as usize;
                let lenses = &mut boxes.get_mut(box_id).unwrap().lenses;
                if let Some(idx) = lenses.iter().position(|l| l.label == label) {
                    lenses[idx] = Lens {
                        label,
                        focal_length,
                    }
                } else {
                    lenses.push(Lens {
                        label,
                        focal_length,
                    });
                }
            }
            Instruction::Rm(label) => {
                let box_id = apply_hash(label) as usize;
                let lenses = &mut boxes.get_mut(box_id).unwrap().lenses;
                if let Some(idx) = lenses.iter().position(|l| l.label == label) {
                    lenses.remove(idx);
                }
            }
        }
    }
    boxes
        .into_iter()
        .enumerate()
        .map(|(i, b)| (i as u64 + 1) * b.local_focusing_power())
        .sum()
}

fn main() {
    let input = include_str!("../../data/day15.txt");
    let data = parse_input(input);
    let answer1 = solve_part1(&data);
    println!("The answer to part 1 is {}", answer1);
    let answer2 = solve_part2(&data);
    println!("The answer to part 2 is {}", answer2);
}

#[cfg(test)]
mod test {
    use crate::{apply_hash, parse_input, solve_part1, solve_part2};

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

    #[test]
    fn test_solve_part2() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7\n";
        let data = parse_input(input);
        assert_eq!(solve_part2(&data), 145);
    }
}
