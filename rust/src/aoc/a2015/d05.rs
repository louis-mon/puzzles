use std::collections::HashSet;
use std::fs;

pub fn run() {
    println!("{}", solve(fs::read_to_string("src/aoc/a2015/d05.input").unwrap()))
}


fn count_vowels(s: &str) -> usize {
    let vowels: HashSet<char> = HashSet::from(['a', 'e', 'i', 'o', 'u']);
    s.chars().filter(|c| vowels.contains(c)).count()
}

fn has_doubles(s: &str) -> bool {
    let mut lc: char = 'a';
    for (i, c) in s.chars().enumerate() {
        if i == 0 {
            lc = c;
            continue;
        }
        if lc == c {
            return true;
        }
        lc = c;
    }
    false
}

fn has_no_special(s: &str) -> bool {
    let sp = vec!["ab", "cd", "pq", "xy"];
    !sp.iter().any(|p| s.contains(p))
}

fn is_nice(s: &str) -> bool {
    count_vowels(s) >= 3 && has_doubles(s)
        && has_no_special(s)
}

fn solve(input: String) -> usize {
    input.split_whitespace().filter(|p| is_nice2(p)).count()
}

fn is_nice2(input: &str) -> bool {
    has_interwind(input) && has_repeat(input)
}

fn has_interwind(s: &str) -> bool {
    let mut lc: char = 'a';
    let mut llc: char = 'a';
    for (i, c) in s.chars().enumerate() {
        if i < 2 {
            if i == 0 {
                llc = c;
            } else {
                lc = c;
            }
            continue;
        }
        if llc == c {
            return true;
        }
        llc = lc;
        lc = c;
    }
    false
}

fn has_repeat(s: &str) -> bool {
    for (i, _) in s.chars().enumerate() {
        if i == 0 {
            continue;
        }
        if s[(i + 1)..].contains(&s[(i - 1)..=i]) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn has_vowels() {
        assert_eq!(count_vowels(&String::from("afi")), 2);
    }

    #[test]
    fn start_double() {
        assert!(has_doubles(&String::from("aac")))
    }
    #[test]
    fn simple_double() {
        assert!(has_doubles(&String::from("aa")))
    }
    #[test]
    fn end_double() {
        assert!(has_doubles(&String::from("acc")))
    }

    #[test]
    fn mid_double() {
        assert!(has_doubles(&String::from("acce")))
    }
    #[test]
    fn no_double() {
        assert!(!has_doubles(&String::from("ace")))
    }

    #[test]
    fn no_double_1char() {
        assert!(!has_doubles(&String::from("a")))
    }

    #[test]
    fn wrong_double() {
        assert!(!has_doubles(&String::from("ca")))
    }

    #[test]
    fn no_double_empty() {
        assert!(!has_doubles(&String::from("")))
    }

    #[test]
    fn rep_1() {
        assert!(has_repeat(&String::from("abab")))
    }

    #[test]
    fn rep_2() {
        assert!(has_repeat(&String::from("ab ab")))
    }

    #[test]
    fn rep_3() {
        assert!(!has_repeat(&String::from("aaa")))
    }

    #[test]
    fn rep_4() {
        assert!(!has_repeat(&String::from("aacc")))
    }
}