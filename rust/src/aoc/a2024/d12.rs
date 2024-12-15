use crate::aoc::common::grid::Grid;
use crate::aoc::common::grid_point::{GridPoint, DIRECTIONS_4};
use crate::read_day_input;
use std::collections::{HashSet, VecDeque};

pub fn run() {
    let grid = Grid::from_str(&read_day_input!());

    println!("{}", solve2(grid));
}

fn solve(grid: Grid<char>) -> usize {
    let mut all_visited: HashSet<GridPoint> = HashSet::new();
    let mut res = 0;
    for (p, c_v) in grid.iter_points() {
        if all_visited.contains(&p) {
            continue;
        }
        let mut queue: VecDeque<GridPoint> = VecDeque::new();
        let mut visited: HashSet<GridPoint> = HashSet::new();
        queue.push_back(p);
        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            if !visited.insert(current) {
                continue;
            }
            for (p_n, n) in grid.neighbors_4(&current) {
                if *n != *c_v {
                    continue;
                }
                queue.push_back(p_n);
            }
        }
        for p in &visited {
            all_visited.insert(*p);
        }
        let perimeter: usize = visited
            .iter()
            .map(|p| {
                DIRECTIONS_4
                    .iter()
                    .filter(|n| match grid.index(&p.add(&n)) {
                        None => true,
                        Some(v) => *v != *c_v,
                    })
                    .count()
            })
            .sum();
        res += perimeter * visited.len()
    }

    res
}

type Fence = (GridPoint, GridPoint);

fn solve2(grid: Grid<char>) -> usize {
    let mut all_visited: HashSet<GridPoint> = HashSet::new();
    let mut res = 0;
    for (p, c_v) in grid.iter_points() {
        if all_visited.contains(&p) {
            continue;
        }
        let mut queue: VecDeque<GridPoint> = VecDeque::new();
        let mut visited: HashSet<GridPoint> = HashSet::new();
        queue.push_back(p);
        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            if !visited.insert(current) {
                continue;
            }
            for (p_n, n) in grid.neighbors_4(&current) {
                if *n != *c_v {
                    continue;
                }
                queue.push_back(p_n);
            }
        }
        for p in &visited {
            all_visited.insert(*p);
        }
        let mut visited_fence = HashSet::new();
        let mut sides = 0;

        let is_same_type = |p: &GridPoint| match grid.index(&p) {
            None => false,
            Some(v) => *v == *c_v,
        };

        let is_fence = |(p, d): &Fence| is_same_type(p) && !is_same_type(&p.add(&d));

        let find_next_fence = |(p, d): Fence| {
            let candidates = vec![
                (p, d.turn_trigo()),
                (p.add(&d.turn_trigo()), d),
                (p.add(&d).add(&d.turn_trigo()), d.turn_trigo().scale(-1)),
            ];
            candidates.iter().find(|f| is_fence(*f)).unwrap().clone()
        };

        for p_0 in &visited {
            let candidates: Vec<Fence> = DIRECTIONS_4
                .iter()
                .map(|p| (*p_0, *p))
                .filter(is_fence)
                .collect();
            for start_fence in candidates {
                if !visited_fence.insert(start_fence) {
                    continue;
                }
                let mut current_fence = start_fence;
                loop {
                    let next_fence = find_next_fence(current_fence);
                    visited_fence.insert(next_fence);
                    if next_fence.1 != current_fence.1 {
                        sides += 1;
                    }
                    if next_fence == start_fence {
                        break;
                    }
                    current_fence = next_fence;
                }
            }
        }
        res += sides * visited.len()
    }

    res
}
