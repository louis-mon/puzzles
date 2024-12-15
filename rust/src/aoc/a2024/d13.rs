use crate::aoc::common::input::{parse_f64, parse_int, parse_long, split_2};
use crate::read_day_input;
use regex::Regex;

pub fn run() {
    println!("{:?}", solve(parse_input(read_day_input!())))
}

type V2 = [i64; 2];
type M2 = [V2; 2];

#[derive(Debug)]
struct Machine {
    buttons: M2,
    target: V2,
}

fn solve(input: Vec<Machine>) -> i64 {
    input.iter().map(solve_machine).sum()
}

fn parse_input(input: String) -> Vec<Machine> {
    input.split("\n\n").into_iter().map(parse_line).collect()
}

fn det(m: &M2) -> i64 {
    m[0][0] * m[1][1] - m[0][1] * m[1][0]
}

fn is_float(f: f64) -> bool {
    (f.round() - f).abs() > 0.0001
}

fn solve_machine(data: &Machine) -> i64 {
    let d = det(&data.buttons);
    if d == 0 {
        return 0;
    }
    let m = &data.buttons;
    let a = m[1][1] * data.target[0] - m[1][0] * data.target[1];
    let b = -m[0][1] * data.target[0] + m[0][0] * data.target[1];
    if a % d != 0 || b % d != 0 {
        return 0;
    }
    3 * (a / d) + (b / d)
}

fn parse_line(input: &str) -> Machine {
    let mut it = input.lines();

    let parse_coords = |s| {
        let reg = Regex::new(r"\+(\d+)").unwrap();
        let mut reg_iter = reg.captures_iter(s);
        [
            parse_long(reg_iter.next().unwrap().get(1).unwrap().as_str()),
            parse_long(reg_iter.next().unwrap().get(1).unwrap().as_str()),
        ]
    };

    let variant = 0;

    let parse_target = |s| {
        let reg = Regex::new(r"=(\d+)").unwrap();
        let mut reg_iter = reg.captures_iter(s);
        [
            parse_long(reg_iter.next().unwrap().get(1).unwrap().as_str()) + variant,
            parse_long(reg_iter.next().unwrap().get(1).unwrap().as_str()) + variant,
        ]
    };

    let buttons = [
        parse_coords(it.next().unwrap()),
        parse_coords(it.next().unwrap()),
    ];
    let target = parse_target(it.next().unwrap());

    Machine { buttons, target }
}
