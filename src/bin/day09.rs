fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|token| token.parse().unwrap())
                .collect()
        })
        .collect()
}

fn derive(values: &[i64]) -> Vec<i64> {
    values.windows(2).map(|w| w[1] - w[0]).collect()
}

fn extrapolate(values: &[i64]) -> (i64, i64) {
    let mut extrema = vec![(
        values.first().copied().unwrap_or_default(),
        values.last().copied().unwrap_or_default(),
    )];
    let mut current = values.to_vec();
    while current.iter().any(|&x| x != 0) {
        current = derive(&current);
        extrema.push((
            current.first().copied().unwrap(),
            current.last().copied().unwrap(),
        ));
    }
    extrema
        .into_iter()
        .rev()
        .fold((0, 0), |(acc_h, acc_t), (h, t)| (-acc_h + h, acc_t + t))
}

fn solve(values: &[Vec<i64>]) -> (i64, i64) {
    values
        .iter()
        .map(|v| extrapolate(&v))
        .fold((0, 0), |(acc_h, acc_t), (h, t)| (acc_h + h, acc_t + t))
}

fn main() {
    let input = include_str!("../../data/day09.txt");
    let values = parse_input(input);
    let (answer2, answer1) = solve(&values);
    println!("The answer to part 1 is {}", answer1);
    println!("The answer to part 2 is {}", answer2);
}

#[cfg(test)]
mod test {
    use crate::extrapolate;

    #[test]
    fn test_extrapolate() {
        assert_eq!(extrapolate(&[0, 3, 6, 9, 12, 15]), (-3, 18));
        assert_eq!(extrapolate(&[1, 3, 6, 10, 15, 21]), (0, 28));
        assert_eq!(extrapolate(&[10, 13, 16, 21, 30, 45]), (5, 68));
    }
}
