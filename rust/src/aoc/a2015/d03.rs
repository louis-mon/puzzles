use std::collections::HashSet;
use std::fs;

pub fn run() {
    println!("{}", solve2(
        fs::read_to_string("src/aoc/a2015/d03.input").unwrap()
    ))
}

pub fn solve(s: String) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut pos: (i32, i32) = (0, 0);
    visited.insert(pos);
    s.chars().for_each(|c| {
        match c {
            '^' => pos.1 -= 1,
            'v' => pos.1 += 1,
            '>' => pos.0 += 1,
            '<' => pos.0 -= 1,
            _ => {}
        };
        visited.insert(pos);
    });
    visited.len()
}

type Pos = (i32, i32);

pub fn solve2(s: String) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut pos_s: [Pos; 2] = [(0, 0), (0, 0)];
    let mut index = 0;
    visited.insert(pos_s[index]);
    s.chars().for_each(|c| {
        let pos = &mut pos_s[index];
        match c {
            '^' => pos.1 -= 1,
            'v' => pos.1 += 1,
            '>' => pos.0 += 1,
            '<' => pos.0 -= 1,
            _ => {}
        };
        visited.insert(pos.clone());
        index = (index + 1) % pos_s.len();
    });
    visited.len()
}