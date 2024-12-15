use crate::aoc::common::grid::Grid;
use crate::aoc::common::grid_point::GridPoint;
use crate::aoc::common::input::split_2_str;
use crate::read_day_input;
use std::collections::{HashSet, VecDeque};

pub fn run() {
    println!("{}", solve(parse_input(read_day_input!())));
}

struct Input {
    grid: Grid<char>,
    path: String,
}

fn parse_input(input: String) -> Input {
    let (g, path) = split_2_str(&input, "\n\n");
    let g_rep: String = g
        .chars()
        .map(|c| match c {
            '.' => String::from(".."),
            '@' => String::from("@."),
            '#' => String::from("##"),
            'O' => String::from("[]"),
            '\n' => String::from("\n"),
            _ => panic!("invalid input"),
        })
        .collect();
    Input {
        grid: Grid::from_str(&g_rep),
        path: path.lines().collect(),
    }
}

fn parse_input_raw(input: String) -> Input {
    let (g, path) = split_2_str(&input, "\n\n");
    Input {
        grid: Grid::from_str(&g),
        path: path.lines().collect(),
    }
}

fn move_robot(g: &Grid<char>, dir: GridPoint) -> Grid<char> {
    let robot_pos = g.find_point('@').unwrap();

    let mut items_to_push: HashSet<GridPoint> = HashSet::new();
    let mut visited: HashSet<GridPoint> = HashSet::new();
    let mut to_process: VecDeque<GridPoint> = VecDeque::new();
    to_process.push_back(robot_pos);
    let mut stuck = false;

    while !to_process.is_empty() {
        let cp = to_process.pop_front().unwrap();
        if !visited.insert(cp) {
            continue;
        }
        match g.index(&cp).unwrap() {
            '.' => {}
            '#' => {
                stuck = true;
            }
            '@' | 'O' => {
                to_process.push_back(cp.add(&dir));
                items_to_push.insert(cp);
            }
            '[' => {
                to_process.push_back(cp.add(&dir));
                to_process.push_back(cp.add(&GridPoint::new(1, 0)));
                items_to_push.insert(cp);
            }
            ']' => {
                to_process.push_back(cp.add(&dir));
                to_process.push_back(cp.add(&GridPoint::new(-1, 0)));
                items_to_push.insert(cp);
            }
            _ => panic!("invalid char"),
        }
    }

    if stuck {
        return g.clone();
    }
    let mut new_grid = g.clone();
    for p in &items_to_push {
        *new_grid.index_mut(&p.add(&dir)).unwrap() = *g.index(p).unwrap();
        if !items_to_push.contains(&p.sub(&dir)) {
            *new_grid.index_mut(p).unwrap() = '.';
        }
    }
    new_grid
}

fn solve(input: Input) -> i32 {
    let mut current_grid = input.grid.clone();
    for dir_c in input.path.chars() {
        let dir = GridPoint::from_char(dir_c).unwrap();
        current_grid = move_robot(&current_grid, dir);
    }
    current_grid
        .iter_points()
        .map(|(p, c)| match c {
            'O' | '[' => p.x + 100 * p.y,
            _ => 0,
        })
        .sum()
}
