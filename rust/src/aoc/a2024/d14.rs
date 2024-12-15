use crate::aoc::common::grid_point::GridPoint;
use crate::aoc::common::input::{parse_int, parse_long};
use crate::read_day_input;
use regex::{Captures, Regex};
use std::collections::HashSet;
use std::io::stdin;

pub fn run() {
    println!("{}", solve2(parse_input(read_day_input!())))
}

#[derive(Debug)]
struct Robot {
    pos: GridPoint,
    vel: GridPoint,
}

fn parse_input(input: String) -> Vec<Robot> {
    let coord_reg = Regex::new(r"=(-?\d+),(-?\d+)").unwrap();

    let get_coord = |c: Captures| {
        GridPoint::new(
            parse_int(c.get(1).unwrap().as_str()),
            parse_int(c.get(2).unwrap().as_str()),
        )
    };
    input
        .lines()
        .map(|l| {
            let mut it = coord_reg.captures_iter(l);
            let pos = get_coord(it.next().unwrap());
            let vel = get_coord(it.next().unwrap());
            Robot { pos, vel }
        })
        .collect()
}

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;
const DURATION: i32 = 100;

fn move_robot(coord: i32, vel: i32, size: i32) -> i32 {
    (((coord + vel * DURATION) % size) + size) % size
}

fn move_robot_1(coord: i32, vel: i32, size: i32) -> i32 {
    (((coord + vel) % size) + size) % size
}

fn sig_robots(robots: &Vec<GridPoint>) -> String {
    let mut res = String::from("");
    let set: HashSet<&GridPoint> = robots.iter().collect();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let p = GridPoint::new(x, y);
            let c = if set.contains(&p) { '*' } else { '.' };
            res.push(c);
        }
    }
    res
}

fn display_robots(robots: &Vec<GridPoint>) {
    let mut res = String::from("");
    let set: HashSet<&GridPoint> = robots.iter().collect();
    for y in 0..HEIGHT / 2 {
        for x in 0..WIDTH {
            let p1 = GridPoint::new(x, y * 2);
            let p2 = GridPoint::new(x, y * 2 + 1);
            let c = if set.contains(&p1) || set.contains(&p2) {
                '#'
            } else {
                ' '
            };
            print!("{c}");
        }
        println!();
    }
}

fn compute_safety(positions: &Vec<GridPoint>) -> i32 {
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    let x = WIDTH / 2;
    let y = HEIGHT / 2;
    for p in positions {
        if p.x < x {
            if p.y < y {
                q1 += 1;
            } else if p.y > y {
                q3 += 1;
            }
        } else if p.x > x {
            if p.y < y {
                q2 += 1;
            } else if p.y > y {
                q4 += 1;
            }
        }
    }
    q1 * q2 * q3 * q4
}

fn solve(robots: Vec<Robot>) -> i32 {
    let final_pos = robots
        .iter()
        .map(|r| {
            GridPoint::new(
                move_robot(r.pos.x, r.vel.x, WIDTH),
                move_robot(r.pos.y, r.vel.y, HEIGHT),
            )
        })
        .collect();
    compute_safety(&final_pos)
}

fn look_like_tree(positions: &Vec<GridPoint>) -> bool {
    let set: HashSet<&GridPoint> = positions.iter().collect();
    for y in 50..90 {
        if !set.contains(&GridPoint::new(WIDTH / 2, y)) {
            return false;
        }
    }
    true
}

fn solve2(robots: Vec<Robot>) -> i32 {
    let mut positions: Vec<GridPoint> = robots.iter().map(|r| r.pos).collect();
    let mut elapsed = 0;
    let mut seen: HashSet<String> = HashSet::new();
    loop {
        if !seen.insert(sig_robots(&positions)) {
            println!("{elapsed}");
            break;
        }
        // 1532 + 1535
        // 1635 + 1636
        // 1738 + 1737
        // 1841 + 1838
        if elapsed % 103 == 90 && elapsed % 101 == 20 {
            println!("Elapsed {elapsed}");
            display_robots(&positions);
            let mut s: String = String::new();
            stdin().read_line(&mut s).unwrap();
        }
        /*let mut s: String = String::new();
        if elapsed > 2000 {

        stdin().read_line(& mut s).unwrap();
        }*/
        positions = positions
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let r = &robots[i];
                GridPoint::new(
                    move_robot_1(p.x, r.vel.x, WIDTH),
                    move_robot_1(p.y, r.vel.y, HEIGHT),
                )
            })
            .collect();
        elapsed += 1;
    }
    0
}
