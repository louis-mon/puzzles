use crate::aoc::common::input::{parse_int, parse_u128};
use crate::read_day_input;
use std::collections::HashMap;

pub fn run() {
    let str = read_day_input!();
    let input: Vec<u128> = str.split_whitespace().map(parse_u128).collect();
    println!("{:?}", solve(input));
}

fn blink_count(cache: &mut HashMap<(u128, i32), u128>, stone: u128, depth: i32) -> u128 {
    if depth == 0 {
        return 1;
    }
    let n_depth = depth - 1;
    match cache.get(&(stone, depth)) {
        Some(x) => *x,
        None => {
            let x = {
                if stone == 0 {
                    blink_count(cache, 1, n_depth)
                } else {
                    let str = stone.to_string();
                    if str.len() % 2 == 0 {
                        let div = 10_u128.pow(str.len() as u32 / 2);
                        blink_count(cache, stone / div, n_depth)
                            + blink_count(cache, stone % div, n_depth)
                    } else {
                        blink_count(cache, stone * 2024, n_depth)
                    }
                }
            };
            cache.insert((stone, depth), x);
            x
        }
    }
}

fn solve(input: Vec<u128>) -> u128 {
    let mut cache: HashMap<(u128, i32), u128> = HashMap::new();
    input.iter().map(|x| blink_count(&mut cache, *x, 75)).sum()
}
