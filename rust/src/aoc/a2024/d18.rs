use crate::aoc::common::grid_point::{GridPoint, DIRECTIONS_4};
use crate::read_day_input;
use priority_queue::PriorityQueue;
use std::collections::{HashMap, HashSet, VecDeque};
use std::env::var;

pub fn run() {
    println!("{:?}", solve2(read_day_input!()));
}

const WIDTH: i32 = 71;
const HEIGHT: i32 = 71;

fn solve(input: String) -> i64 {
    let bytes: HashSet<GridPoint> = input
        .lines()
        .map(|l| GridPoint::from_str_coord(l))
        .take(1024)
        .collect();

    let mut distances: HashMap<GridPoint, i64> = HashMap::new();
    distances.insert(GridPoint::new(0, 0), 0);
    let mut pq = PriorityQueue::<GridPoint, i64>::new();
    for v in distances.iter() {
        pq.push(*v.0, *v.1);
    }

    let get_n = |p: &GridPoint| -> Vec<GridPoint> {
        DIRECTIONS_4
            .iter()
            .map(|d| d.add(p))
            .filter(|p| !bytes.contains(p) && p.within_rect(0, 0, WIDTH, HEIGHT))
            .collect()
    };

    while !pq.is_empty() {
        let (v_c, dist) = pq.pop().unwrap();
        for v_n in get_n(&v_c) {
            let new_dist = dist + 1;
            let cell_n = distances.entry(v_n).or_insert(i64::MAX);
            if new_dist < *cell_n {
                *cell_n = new_dist;
                pq.push(v_n, new_dist);
            }
        }
    }
    distances[&GridPoint::new(WIDTH - 1, HEIGHT - 1)]
}

fn is_path(bytes: &Vec<GridPoint>, nb: usize) -> bool {
    let walls: HashSet<GridPoint> = bytes.clone().into_iter().take(nb).collect();
    let mut q = VecDeque::from(vec![GridPoint::new(0, 0)]);
    let mut visited: HashSet<GridPoint> = Default::default();
    let get_n = |p: &GridPoint| -> Vec<GridPoint> {
        DIRECTIONS_4
            .iter()
            .map(|d| d.add(p))
            .filter(|p| !walls.contains(p) && p.within_rect(0, 0, WIDTH, HEIGHT))
            .collect()
    };

    while !q.is_empty() {
        let v_c = q.pop_front().unwrap();
        if !visited.insert(v_c) {
            continue;
        }
        for v_n in get_n(&v_c) {
            q.push_back(v_n)
        }
    }
    visited.contains(&GridPoint::new(WIDTH - 1, HEIGHT - 1))
}

fn solve2(input: String) -> GridPoint {
    let bytes: Vec<GridPoint> = input
        .lines()
        .map(|l| GridPoint::from_str_coord(l))
        .collect();

    for (i, v) in bytes.iter().enumerate() {
        if !is_path(&bytes, i + 1) {
            return *v;
        }
    }
    panic!("not found");
}
