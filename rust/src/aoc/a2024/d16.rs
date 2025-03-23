use crate::aoc::common::grid::Grid;
use crate::aoc::common::grid_point::GridPoint;
use crate::read_day_input;
use priority_queue::PriorityQueue;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

pub fn run() {
    solve(Grid::from_str(&read_day_input!()))
}

type Vertex = (GridPoint, bool);

fn solve(input: Grid<char>) {
    let walls = input.map(|c| *c == '#');
    let start = input.find_point('S').unwrap();
    let end = input.find_point('E').unwrap();

    let horizontal = false;
    let vertical = true;

    let get_n = |v: &Vertex| -> Vec<(Vertex, i64)> {
        let dir = if v.1 == horizontal {
            GridPoint::new(1, 0)
        } else {
            GridPoint::new(0, 1)
        };
        vec![
            ((v.0.add(&dir), v.1), 1),
            ((v.0.add(&dir.scale(-1)), v.1), 1),
            ((v.0, !v.1), 1000),
        ]
    };

    let mut distances: HashMap<Vertex, i64> = HashMap::new();
    let mut prev_map: HashMap<Vertex, HashSet<Vertex>> = HashMap::new();
    distances.insert((start, horizontal), 0);
    let mut pq = PriorityQueue::<Vertex, i64>::new();
    for v in distances.iter() {
        pq.push(*v.0, *v.1);
    }
    while !pq.is_empty() {
        let (v_c, dist) = pq.pop().unwrap();
        for (v_n, cost_n) in get_n(&v_c) {
            if *walls.index(&v_n.0).unwrap() {
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

    let last_point =
        if distances.get(&(end, horizontal)).unwrap() < distances.get(&(end, vertical)).unwrap() {
            (end, horizontal)
        } else {
            (end, vertical)
        };

    let nb_places = {
        let mut visited: HashSet<Vertex> = HashSet::new();
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
            .iter()
            .map(|v| v.0)
            .collect::<HashSet<GridPoint>>()
            .len()
    };
    println!("{nb_places}");
}
