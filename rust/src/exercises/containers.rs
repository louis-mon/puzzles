use std::collections::HashMap;

#[derive(Debug)]
pub struct Stats {
    median: i32,
    mode: i32,
}

pub fn get_stats(values: &Vec<i32>) -> Stats {
    let mut copied = values.clone();
    copied.sort();
    let median = copied[copied.len() / 2];
    let mut hist = HashMap::new();
    for val in copied {
        let count = hist.entry(val).or_insert(1);
        *count += 1;
    }
    let mut mode: Option<i32> = None;
    let mut max_count = i32::MIN;
    for (key, val) in hist {
        if val > max_count {
            max_count = val;
            mode = Some(key);
        }
    }
    Stats {
        mode: mode.unwrap_or(0),
        median,
    }
}

pub fn to_pig_latin(s: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut result: String = String::new();
    for word in &mut s.split_whitespace() {
        let tr = if word.starts_with(&vowels) {
            format!("{word}-hay")
        } else {
            let st = &word[0..1];
            let end = &word[1..];
            format!("{end}-{st}ay")
        };
        result += &format!("{tr} ")
    }
    result
}
