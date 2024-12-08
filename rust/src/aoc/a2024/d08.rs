use crate::aoc::common::algo::GroupBy;
use crate::aoc::common::grid::Grid;
use crate::aoc::common::grid_point::GridPoint;
use crate::read_day_input;
use std::collections::HashSet;
use std::ops::RangeInclusive;

pub fn run() {
    let grid = Grid::from_str(&read_day_input!());

    println!("{}", solve2(grid));
}

fn solve(grid: Grid<char>) -> i32 {
    let grouped = grid
        .iter_points()
        .filter(|x| *x.1 != '.')
        .map(|(a, b)| (*b, a))
        .group_by_key();
    let mut found: HashSet<GridPoint> = HashSet::new();

    for (_, point_list) in grouped {
        for p0 in &point_list {
            for p1 in &point_list {
                if p0 == p1 {
                    continue;
                }
                let other = p0.add(&p1.sub(p0).scale(2));
                if grid.index(&other).is_some() {
                    found.insert(other);
                }
            }
        }
    }
    found.len() as i32
}

fn allowed_range(m: i32, a: i32, b: i32) -> RangeInclusive<i32> {
    let c = b as f32 - a as f32;
    let min = -a as f32 / c;
    let max = (m - a) as f32 / c;
    (min.ceil() as i32)..=(max.floor() as i32)
}

fn solve2(grid: Grid<char>) -> i32 {
    let grouped = grid
        .iter_points()
        .filter(|x| *x.1 != '.')
        .map(|(a, b)| (*b, a))
        .group_by_key();
    let mut found: HashSet<GridPoint> = HashSet::new();

    for (_, point_list) in grouped {
        for p0 in &point_list {
            for p1 in &point_list {
                if p0 == p1 {
                    continue;
                }
                for k in allowed_range(grid.width(), p0.x, p1.x) {
                    let other = p0.add(&p1.sub(p0).scale(k));
                    if grid.index(&other).is_some() {
                        found.insert(other);
                    }
                }
            }
        }
    }
    found.len() as i32
}
