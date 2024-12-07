use crate::aoc::common::input::{parse_int, split_2, split_2_str, to_tuple2, to_tuple2_str};
use crate::read_day_input;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

fn group_by<K: Eq + Hash, V, I>(it: I) -> HashMap<K, Vec<V>>
where
    I: Iterator<Item = (K, V)>,
{
    let mut m: HashMap<K, Vec<V>> = HashMap::new();
    for (k, v) in it {
        m.entry(k).or_default().push(v);
    }
    m
}

pub fn run() {
    let input = read_day_input!();

    let (orderings, print_list) = split_2(&input, "\n\n");
    let orderings: HashMap<String, HashSet<String>> =
        group_by(orderings.lines().map(|l| split_2_str(l, "|")))
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().collect::<HashSet<String>>()))
            .collect();

    let in_wrong_order = print_list
        .lines()
        .map(|l| l.split(",").collect::<Vec<&str>>())
        .filter(|files| {
            for (i, e1) in files.iter().enumerate() {
                for j in i + 1..files.len() {
                    let ok = match orderings.get(*e1) {
                        Some(v) => v.contains(files[j]),
                        None => false,
                    };
                    if !ok {
                        return true;
                    }
                }
            }
            false
        });

    let r: i32 = in_wrong_order
        .map(|files| {
            let mut files_set: HashSet<&str> = files.into_iter().collect();

            let mut succ: HashMap<&str, HashSet<&str>> = HashMap::new();

            for v in files_set.iter() {
                let v_succ = succ.entry(v).or_default();
                for v2 in files_set.iter() {
                    if orderings.get(*v).map(|x| x.contains(*v2)).unwrap_or(false) {
                        v_succ.insert(v2);
                    }
                }
            }

            let mut res: Vec<&str> = Vec::new();
            while !files_set.is_empty() {
                let next = files_set
                    .iter()
                    .find(|v| succ.get(*v).unwrap().is_empty())
                    .unwrap()
                    .clone();
                res.push(next);
                succ.values_mut().for_each(|v| {
                    v.remove(next);
                });
                files_set.remove(next);
            }
            parse_int(res[res.len() / 2])
        })
        .sum();
    println!("{r}")
}

pub fn run1() {
    let input = read_day_input!();

    let (orderings, print_list) = to_tuple2(input.split("\n\n"));
    let orderings: HashMap<String, HashSet<String>> =
        group_by(orderings.lines().map(|l| to_tuple2_str(l.split("|"))))
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().collect::<HashSet<String>>()))
            .collect();

    let res: i32 = print_list
        .lines()
        .map(|l| l.split(",").collect::<Vec<&str>>())
        .filter(|files| {
            for (i, e1) in files.iter().enumerate() {
                for j in i + 1..files.len() {
                    let ok = match orderings.get(*e1) {
                        Some(v) => v.contains(files[j]),
                        None => false,
                    };
                    if !ok {
                        return false;
                    }
                }
            }
            true
        })
        .map(|files| parse_int(files[files.len() / 2]))
        .sum();
    println!("{res}")
}
