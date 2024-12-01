pub fn run() -> usize {
    part_two(std::fs::read_to_string("src/aoc/a2015/d01.input").unwrap())
}

fn solve_str(s: String) -> i32 {
    let mut floor = 0;
    for c in s.chars() {
        match c {
            '(' => {
                floor += 1;
            }
            ')' => {
                floor -= 1;
            }
            _ => {}
        }
    }
    floor
}

fn part_two(s: String) -> usize {
    let mut floor: i32 = 0;
    for (i, c) in s.chars().enumerate() {
        match c {
            '(' => {
                floor += 1;
            }
            ')' => {
                floor -= 1;
            }
            _ => {}
        }
        if floor < 0 {
            return i + 1;
        }
    }
    panic!("Didn't find a floor");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basement() {
        assert_eq!(solve_str(String::from("))")), -2)
    }

    #[test]
    fn going_up() {
        assert_eq!(solve_str(String::from("))(((")), 1)
    }

    #[test]
    fn first_floor() {
        assert_eq!(part_two(String::from("(()()))))(")), 7)
    }
}
