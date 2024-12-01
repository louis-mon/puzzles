use regex::Regex;
use std::collections::HashMap;
use std::fs;

const SIZE: usize = 1000;

struct Lights {
    leds: [u32; SIZE * SIZE],
}

pub fn run() {
    println!(
        "{}",
        solve(fs::read_to_string("src/aoc/a2015/d06.input").unwrap())
    );
}

/*
pub fn turn_on(v: &mut bool) -> () {
    *v = true;
}

pub fn turn_off(v: &mut bool) -> () {
    *v = false;
}

pub fn toggle(v: &mut bool) -> () {
    *v = !*v;
}
*/

pub fn turn_on(v: &mut u32) -> () {
    *v += 1;
}

pub fn turn_off(v: &mut u32) -> () {
    if *v > 0 {
        *v -= 1;
    }
}

pub fn toggle(v: &mut u32) -> () {
    *v += 2;
}
fn solve(commands: String) -> u32 {
    let mut lights = Lights {
        leds: [0; SIZE * SIZE],
    };

    let mut actions: HashMap<&str, fn(v: &mut u32) -> ()> = HashMap::new();
    actions.insert("turn on", turn_on);
    actions.insert("turn off", turn_off);
    actions.insert("toggle", toggle);

    let reg = Regex::new(r"(^.*) (\d+),(\d+) through (\d+),(\d+)").unwrap();

    for line in commands.lines() {
        let (_, [cmd, ls, ts, rs, bs]) = reg.captures(line).unwrap().extract();
        let t: usize = ts.parse().unwrap();
        let b: usize = bs.parse().unwrap();
        let r: usize = rs.parse().unwrap();
        let l: usize = ls.parse().unwrap();
        let act = actions.get(cmd).unwrap();
        for y in t..=b {
            for x in l..=r {
                act(&mut lights.leds[y * SIZE + x])
            }
        }
    }
    lights.leds.iter().sum()
}
