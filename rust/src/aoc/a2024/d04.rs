use crate::aoc::common::grid::Grid;
use crate::aoc::common::grid_point::{GridPoint, DIRECTIONS_8};
use crate::read_day_input;

pub fn run() {
    println!("{}", solve2(read_day_input!()))
}

type Problem = Grid<char>;

fn search_from_dir(problem: &Grid<char>, p: &GridPoint, dir: &GridPoint) -> bool {
    let mut current_pos: GridPoint = p.clone();
    for c in vec!['X', 'M', 'A', 'S'] {
        match problem.index(&current_pos) {
            None => return false,
            Some(current_char) => {
                if c != *current_char {
                    return false;
                }
            }
        }
        current_pos = current_pos.add(dir);
    }
    true
}

fn search_from(problem: &Problem, p: &GridPoint) -> usize {
    DIRECTIONS_8
        .iter()
        .filter(|dir| search_from_dir(problem, p, dir))
        .count()
}

fn match_letters(problem: &Problem, p1: &GridPoint, p2: &GridPoint) -> bool {
    *problem.index(p1).unwrap_or(&'.') == 'M' && *problem.index(p2).unwrap_or(&'.') == 'S'
}

fn search_from2(problem: &Problem, p: &GridPoint) -> bool {
    if *problem.index(p).unwrap_or(&'.') != 'A' {
        return false;
    }
    let tl = p.add(&GridPoint::new(-1, -1));
    let br = p.add(&GridPoint::new(1, 1));
    let tr = p.add(&GridPoint::new(1, -1));
    let bl = p.add(&GridPoint::new(-1, 1));
    (match_letters(problem, &tl, &br) || match_letters(problem, &br, &tl))
        && (match_letters(problem, &tr, &bl) || match_letters(problem, &bl, &tr))
}

fn solve(input: String) -> usize {
    let problem = Grid::from_str(&input);
    problem
        .iter_points()
        .map(|(pos, _)| search_from(&problem, &pos))
        .sum()
}

fn solve2(input: String) -> usize {
    let chars: Vec<char> = input.chars().filter(|c| *c != '\n').collect();
    let width = input.find("\n").unwrap() as i32;
    let total_size = chars.len();
    let problem = Grid::from_str(&input);
    problem
        .iter_points()
        .filter(|(pos, _)| search_from2(&problem, &pos))
        .count()
}
