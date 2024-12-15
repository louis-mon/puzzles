use crate::aoc::common::grid::Grid;
use crate::aoc::common::grid_point::GridPoint;
use crate::read_day_input;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn run() {
    let grid = Grid::from_str(&read_day_input!()).to_digit();

    println!("{}", solve2(grid));
}

fn solve(input: Grid<i32>) -> usize {
    let mut res = 0;
    for (p, start_height) in input.iter_points() {
        if *start_height != 0 {
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
            let current_val = *input.index(&current).unwrap();
            for (p_n, n) in input.neighbors_4(&current) {
                if *n != current_val + 1 {
                    continue;
                }
                queue.push_back(p_n);
            }
        }
        res += visited
            .iter()
            .filter(|v| *input.index(v).unwrap() == 9)
            .count();
    }
    res
}

fn number_of_paths(prev_map: &HashMap<GridPoint, HashSet<GridPoint>>, from: &GridPoint) -> usize {
    match prev_map.get(from) {
        Some(prevs) => prevs.iter().map(|p| number_of_paths(prev_map, p)).sum(),
        None => 1,
    }
}

fn solve2(input: Grid<i32>) -> usize {
    let mut res = 0;
    for (p, start_height) in input.iter_points() {
        if *start_height != 0 {
            continue;
        }
        let mut queue: VecDeque<GridPoint> = VecDeque::new();
        let mut visited: HashSet<GridPoint> = HashSet::new();
        let mut prev_map: HashMap<GridPoint, HashSet<GridPoint>> = HashMap::new();
        queue.push_back(p);
        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            if !visited.insert(current) {
                continue;
            }
            let current_val = *input.index(&current).unwrap();
            if current_val == 9 {
                res += number_of_paths(&prev_map, &current);
            }
            for (p_n, n) in input.neighbors_4(&current) {
                if *n != current_val + 1 {
                    continue;
                }
                queue.push_back(p_n);
                prev_map.entry(p_n).or_default().insert(current);
            }
        }
    }
    res
}
