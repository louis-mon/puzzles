use crate::aoc::common::grid::Grid;
use crate::aoc::common::grid_point::GridPoint;
use crate::read_day_input;
use std::collections::HashSet;

pub fn run() {
    let grid = Grid::from_str(&read_day_input!());

    let start_pos = grid.find_point('^').unwrap();
    let on_path = positions_on_path(&grid);

    let mut possible_obstacles = 0;

    'obstacle: for (obstacle_pos) in on_path {
        if obstacle_pos == start_pos {
            continue;
        }
        let mut found_pos = HashSet::<(GridPoint, GridPoint)>::new();
        let mut curr_pos = start_pos;
        let mut curr_dir = GridPoint::new(0, -1);

        while grid.index(&curr_pos).is_some() {
            if !found_pos.insert((curr_pos, curr_dir)) {
                possible_obstacles += 1;
                continue 'obstacle;
            }
            let next_pos = curr_pos.add(&curr_dir);
            if next_pos == obstacle_pos || *grid.index(&next_pos).unwrap_or(&' ') == '#' {
                curr_dir = curr_dir.turn_trigo();
            } else {
                curr_pos = next_pos;
            }
        }
    }
    println!("{}", possible_obstacles);
}

fn positions_on_path(grid: &Grid<char>) -> HashSet<GridPoint> {
    let start_pos = grid.find_point('^').unwrap();

    let mut found_pos = HashSet::<GridPoint>::new();
    let mut curr_pos = start_pos;
    let mut curr_dir = GridPoint::new(0, -1);

    while grid.index(&curr_pos).is_some() {
        found_pos.insert(curr_pos);
        let next_pos = curr_pos.add(&curr_dir);
        if *grid.index(&next_pos).unwrap_or(&' ') == '#' {
            curr_dir = curr_dir.turn_trigo();
        } else {
            curr_pos = next_pos;
        }
    }
    found_pos
}

pub fn run1() {
    let grid = Grid::from_str(&read_day_input!());

    let found_pos = positions_on_path(&grid);
    println!("{}", found_pos.len());
}
