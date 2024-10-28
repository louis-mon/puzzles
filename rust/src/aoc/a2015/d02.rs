use std::fs;

pub fn run() {
    println!(
        "{}",
        compute2(parse(
            fs::read_to_string("src/aoc/a2015/d02.input").unwrap()
        ))
    );
}

type Box = Vec<i32>;

fn parse(input: String) -> Vec<Box> {
    input
        .split_whitespace()
        .map(|b_str| {
            let mut v: Box = b_str
                .split("x")
                .map(|n| -> i32 { n.parse().unwrap() })
                .collect();
            v.sort();
            v
        })
        .collect()
}

fn compute(boxes: Vec<Box>) -> i32 {
    boxes
        .iter()
        .map(|b| {
            let ss1 = b[0];
            let ss2 = b[1];
            let ss3 = b[2];
            return ss1 * ss2 * 3 + 2 * ss2 * ss3 + 2 * ss1 * ss3;
        })
        .sum()
}

fn compute2(boxes: Vec<Box>) -> i32 {
    boxes
        .iter()
        .map(|b| {
            let ss1 = b[0];
            let ss2 = b[1];
            let ss3 = b[2];
            return (ss1 + ss2) * 2 + ss1 * ss2 * ss3;
        })
        .sum()
}
