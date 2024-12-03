use crate::aoc::common::input::parse_int;
use crate::read_day_input;
use regex::Regex;
use std::fs;

pub fn run() {
    println!("{}", solve(read_day_input!()));
}

fn solve(input: String) -> i32 {
    let reg = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();
    let mut res = 0;
    let mut activated = true;
    for x in reg.captures_iter(&input) {
        match x.get(0).unwrap().as_str() {
            "do()" => {
                activated = true;
            }
            "don't()" => {
                activated = false;
            }
            _ => {
                if activated {
                    let n1 = parse_int(x.get(1).unwrap().as_str());
                    let n2 = parse_int(x.get(2).unwrap().as_str());
                    res += n1 * n2;
                }
            }
        }
    }
    res
}
