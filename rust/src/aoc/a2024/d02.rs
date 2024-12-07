use crate::aoc::common::input::parse_int;
use crate::read_day_input;

pub fn run() {
    println!("{}", solve(parse_input(&read_day_input!())))
}

type ParsedInput = Vec<Vec<i32>>;

fn parse_input(input: &str) -> ParsedInput {
    input
        .lines()
        .map(|l| l.split_whitespace().map(parse_int).collect())
        .collect()
}

fn is_safe2(line: &&Vec<i32>) -> bool {
    if is_safe(line) {
        return true;
    }
    for j in 0..line.len() {
        let line1: Vec<i32> = line
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != j)
            .map(|(_, a)| *a)
            .collect();
        if is_safe(&line1) {
            return true;
        }
    }
    false
}

fn is_safe(line: &[i32]) -> bool {
    if line.len() < 2 {
        return false;
    }
    let s = (line[1] - line[0]).signum();
    if s == 0 {
        return false;
    }
    for i in 1..line.len() {
        let diff = line[i] - line[i - 1];
        let diff_abs = diff.abs();
        if diff_abs > 3 || diff.signum() != s {
            return false;
        }
    }
    true
}

fn solve(input: ParsedInput) -> usize {
    input.iter().filter(is_safe2).count()
}
