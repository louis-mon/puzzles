use crate::aoc::common::grid_point::GridPoint;
use crate::read_day_input;

pub fn run() {
    println!("{}", solve2(read_day_input!()))
}

struct Problem {
    chars: Vec<char>,
    width: i32,
    all_dirs: Vec<GridPoint>,
}

impl Problem {
    fn height(&self) -> i32 {
        self.chars.len() as i32 / self.width
    }

    fn index(&self, p: &GridPoint) -> Option<char> {
        if p.x < 0 || p.y < 0 || p.x >= self.width || p.y >= self.height() {
            return None;
        }
        Some(self.chars[(self.width * p.y + p.x) as usize])
    }
}

fn search_from_dir(problem: &Problem, p: &GridPoint, dir: &GridPoint) -> bool {
    let mut current_pos: GridPoint = p.clone();
    for c in vec!['X', 'M', 'A', 'S'] {
        match problem.index(&current_pos) {
            None => return false,
            Some(current_char) => {
                if c != current_char {
                    return false;
                }
            }
        }
        current_pos = current_pos.add(dir);
    }
    true
}

fn search_from(problem: &Problem, p: &GridPoint) -> usize {
    problem
        .all_dirs
        .iter()
        .filter(|dir| search_from_dir(problem, p, dir))
        .count()
}

fn match_letters(problem: &Problem, p1: &GridPoint, p2: &GridPoint) -> bool {
    problem.index(p1).unwrap_or('.') == 'M' && problem.index(p2).unwrap_or('.') == 'S'
}

fn search_from2(problem: &Problem, p: &GridPoint) -> bool {
    if problem.index(p).unwrap_or('.') != 'A' {
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
    let chars: Vec<char> = input.chars().filter(|c| *c != '\n').collect();
    let width = input.find("\n").unwrap() as i32;
    let total_size = chars.len();
    let all_dirs = vec![
        GridPoint::new(1, 0),
        GridPoint::new(1, 1),
        GridPoint::new(0, 1),
        GridPoint::new(-1, 1),
        GridPoint::new(-1, 0),
        GridPoint::new(-1, -1),
        GridPoint::new(0, -1),
        GridPoint::new(1, -1),
    ];
    let problem = Problem {
        chars,
        width,
        all_dirs,
    };
    (0..total_size)
        .map(|i| {
            let pos = GridPoint::from_width(i as i32, width);
            search_from(&problem, &pos)
        })
        .sum()
}

fn solve2(input: String) -> usize {
    let chars: Vec<char> = input.chars().filter(|c| *c != '\n').collect();
    let width = input.find("\n").unwrap() as i32;
    let total_size = chars.len();
    let all_dirs = vec![];
    let problem = Problem {
        chars,
        width,
        all_dirs,
    };
    (0..total_size)
        .filter(|i| {
            let pos = GridPoint::from_width(*i as i32, width);
            search_from2(&problem, &pos)
        })
        .count()
}
