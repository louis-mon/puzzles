use crate::aoc::common::algo::GroupBy;
use crate::aoc::common::grid::Grid;
use crate::aoc::common::grid_point::{GridPoint, DIRECTIONS_4};
use crate::read_day_input;
use priority_queue::PriorityQueue;
use std::collections::{BTreeSet, HashMap, HashSet};

pub fn run() {
    println!("{}", solve(Grid::from_str(&read_day_input!())));
}

fn solve(input: Grid<char>) -> i64 {
    let walls = input.map(|c| *c == '#');
    let start = input.find_point('S').unwrap();
    let end = input.find_point('E').unwrap();

    let get_n = |v: &GridPoint| -> Vec<(GridPoint, i64)> {
        DIRECTIONS_4.iter().map(|d| (d.add(v), 1)).collect()
    };

    let mut distances: HashMap<GridPoint, i64> = HashMap::new();
    let mut prev_map: HashMap<GridPoint, HashSet<GridPoint>> = HashMap::new();
    distances.insert(start, 0);
    let mut pq = PriorityQueue::<GridPoint, i64>::new();
    for v in distances.iter() {
        pq.push(*v.0, *v.1);
    }
    while !pq.is_empty() {
        let (v_c, dist) = pq.pop().unwrap();
        for (v_n, cost_n) in get_n(&v_c) {
            if *walls.index(&v_n).unwrap() {
                continue;
            }
            let new_dist = dist + cost_n;
            let cell_n = distances.entry(v_n).or_insert(i64::MAX);
            if new_dist < *cell_n {
                *cell_n = new_dist;
                pq.push(v_n, new_dist);
                prev_map.insert(v_n, HashSet::from([v_c]));
            }
            if new_dist == *cell_n {
                prev_map.get_mut(&v_n).unwrap().insert(v_c);
            }
        }
    }

    let last_point = end;

    let critical_path: HashSet<GridPoint> = {
        let mut visited: HashSet<GridPoint> = HashSet::new();
        let mut to_visit = vec![last_point];
        while !to_visit.is_empty() {
            let c = to_visit.pop().unwrap();
            if !visited.insert(c) {
                continue;
            }
            for n in prev_map.entry(c).or_default().iter() {
                to_visit.push(*n);
            }
        }
        visited
    };

    let mut count_save: Vec<i64> = Default::default();
    let mut count100 = 0;
    let cheat_duration = 20i32;
    for (cheat_start, _) in input.iter_points() {
        if !critical_path.contains(&cheat_start) {
            continue;
        }
        let start_dist = distances[&cheat_start];
        for i in -cheat_duration..=cheat_duration {
            for j in -cheat_duration..=cheat_duration {
                let cost = i.abs() + j.abs();
                if cost < 2 || cost > cheat_duration {
                    continue;
                }
                let cheat_end = cheat_start.add(&GridPoint::new(i, j));
                if !critical_path.contains(&cheat_end) {
                    continue;
                }
                let end_dist = distances[&cheat_end];
                let dist_diff = end_dist - start_dist;
                let saved_time = dist_diff - cost as i64;
                if saved_time >= 100 {
                    count_save.push(saved_time);
                    count100 += 1;
                }
            }
        }
    }
    count100
}
