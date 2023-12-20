use itertools::Itertools;
use std::{io, vec};

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    println!("part 1: {:?}", part1(&lines));
    println!("part 2: {:?}", part2(&lines));
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Cell {
    Empty,
}

fn part1(lines: &Vec<String>) -> String {
    let lines = parse(lines);
    let sum = lines
        .iter()
        .map(|line: &Vec<isize>| {
            let mut vec_of_computed_lines = vec![];
            vec_of_computed_lines.push(line.clone());
            let mut computed_line = compute_line(line);
            vec_of_computed_lines.push(computed_line.clone());
            while !computed_line.iter().all(|x| *x == 0) {
                computed_line = compute_line(&computed_line);
                vec_of_computed_lines.push(computed_line.clone());
            }
            // vec_of_computed_lines.push(computed_line.clone());
            // computed_line = compute_line(&computed_line);

            vec_of_computed_lines
        })
        .map(|x| {
            x.iter()
                .map(|y| y.last().expect("must number"))
                .sum::<isize>()
        })
        .sum::<isize>();
    format!("{}", sum)
}

fn compute_line(line: &Vec<isize>) -> Vec<isize> {
    line.iter()
        .tuple_windows()
        .map(|(current, next)| next - current)
        .collect_vec()
}

fn part2(lines: &Vec<String>) -> String {
    let lines = parse(lines);
    let sum = lines
        .iter()
        .map(|line: &Vec<isize>| {
            let mut vec_of_computed_lines = vec![];
            vec_of_computed_lines.push(line.clone());
            let mut computed_line = compute_line(line);
            vec_of_computed_lines.push(computed_line.clone());
            while !computed_line.iter().all(|x| *x == 0) {
                computed_line = compute_line(&computed_line);
                vec_of_computed_lines.push(computed_line.clone());
            }
            // vec_of_computed_lines.push(computed_line.clone());
            // computed_line = compute_line(&computed_line);

            vec_of_computed_lines
        })
        .map(|x| {
            x.iter()
                .rev()
                .map(|y| y.first().expect("must number"))
                .fold(0, |acc, y| (y - acc))
        })
        .sum::<isize>();
    format!("{}", sum)
}

fn parse(lines: &Vec<String>) -> Vec<Vec<isize>> {
    lines
        .iter()
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.parse::<isize>().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn sampledata() -> Vec<String> {
        vec![
            "0 3 6 9 12 15".to_string(),
            "1 3 6 10 15 21".to_string(),
            "10 13 16 21 30 45".to_string(),
        ]
    }

    #[rstest]
    #[case(sampledata(), "114")]
    fn test_part1_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part1(&input));
    }

    #[rstest]
    #[case(sampledata(), "20")]
    fn test_part2_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part2(&input));
    }
}
