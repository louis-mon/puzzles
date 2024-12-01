use crate::aoc::common::input::parse_int;
use crate::read_day_input;
use std::collections::HashSet;
use std::fs;

pub fn run() {
    println!("{}", solve2(read_day_input!()))
}

fn solve(input: String) -> i32 {
    let v: Vec<Vec<&str>> = input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
        .collect();
    let mut l1: Vec<i32> = v.iter().map(|l| parse_int(l[0])).collect();
    let mut l2: Vec<i32> = v.iter().map(|l| parse_int(l[1])).collect();
    l1.sort();
    l2.sort();
    l1.iter().zip(l2).map(|(a, b)| (a - b).abs()).sum()
}

fn solve2(input: String) -> i32 {
    let v: Vec<Vec<&str>> = input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
        .collect();
    let mut l1: HashSet<i32> = v.iter().map(|l| parse_int(l[0])).collect();
    let mut l2: Vec<i32> = v.iter().map(|l| parse_int(l[1])).collect();
    l2.iter().filter(|x| l1.contains(x)).sum()
}
