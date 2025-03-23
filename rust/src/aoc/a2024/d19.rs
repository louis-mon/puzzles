use crate::aoc::common::input::split_2;
use crate::read_day_input;
use std::cmp::Ordering;
use std::collections::HashSet;

pub fn run() {
    println!("{}", solve(parse_input(read_day_input!())));
}

type Problem = (Vec<String>, Vec<String>);

fn parse_input(input: String) -> Problem {
    let (ts, ds) = split_2(&input, "\n\n");
    (
        ts.split(", ").map(|l| String::from(l)).collect(),
        ds.lines().map(String::from).collect(),
    )
}

struct Patterns {
    towels: HashSet<String>,
    min_s: usize,
    max_s: usize,
}

fn can_design(patterns: &Patterns, design: &str) -> i128 {
    let mut array: Vec<i128> = (0..=design.len()).map(|x| 0).collect();
    array[design.len()] = 1;
    for i in 0..=design.len() {
        let end_i = design.len() - i;
        if array[end_i] == 0 {
            continue;
        }
        for j in patterns.min_s..=patterns.max_s {
            let start_i = end_i as i32 - j as i32;
            if start_i < 0 {
                continue;
            }
            let start_i = start_i as usize;
            let sub = &design[start_i..end_i];
            //println!("analyse s={start_i}, e={end_i} {sub}");
            if patterns.towels.contains(sub) {
                //println!("found {sub} at {start_i}");
                array[start_i] += array[end_i];
            }
        }
    }
    array[0]
}

fn solve((towels, designs): Problem) -> i128 {
    let towels: HashSet<String> = towels.into_iter().collect();
    let patterns = Patterns {
        min_s: towels.iter().map(String::len).min().unwrap(),
        max_s: towels.iter().map(String::len).max().unwrap(),
        towels,
    };
    designs.iter().map(|d| can_design(&patterns, d)).sum()
}
