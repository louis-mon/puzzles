use crate::read_day_input;
use std::collections::BTreeSet;

pub fn run() {
    println!("{}", solve(read_day_input!()));
}

fn parse_disk_map(input: &String) -> Vec<i32> {
    let mut current_id = 0;
    let mut is_file = true;
    let mut disk_map: Vec<i32> = Vec::new();
    for space in input.chars() {
        let d = space.to_digit(10).unwrap();
        let id = if is_file { current_id } else { -1 };
        for _ in (0..d) {
            disk_map.push(id);
        }
        if is_file {
            is_file = false;
        } else {
            current_id += 1;
            is_file = true;
        }
    }
    disk_map
}

fn find_next_space(disk_map: &Vec<i32>, pos: &mut usize) {
    *pos = *pos
        + disk_map[*pos..]
            .iter()
            .position(|b| *b == -1)
            .unwrap_or(disk_map.len());
}

fn optimize(disk_map: &mut Vec<i32>) {
    let mut next_free_space = 0;
    find_next_space(disk_map, &mut next_free_space);
    for i in (0..disk_map.iter().len()).rev() {
        if i < next_free_space {
            break;
        }
        if disk_map[i] != -1 {
            disk_map[next_free_space] = disk_map[i];
            disk_map[i] = -1;
            find_next_space(disk_map, &mut next_free_space);
        }
    }
}

fn find_next_to_move(disk_map: &Vec<i32>, curr_pos: &mut i32, block_size: &mut usize) {
    *curr_pos -= 1;
    while *curr_pos >= 0 && disk_map[*curr_pos as usize] == -1 {
        *curr_pos -= 1;
    }
    if *curr_pos < 0 {
        return;
    }
    let id = disk_map[*curr_pos as usize];
    *block_size = 1;
    while *curr_pos > 0 && disk_map[*curr_pos as usize - 1] == id {
        *curr_pos -= 1;
        *block_size += 1;
    }
}

fn list_free_space(disk_map: &Vec<i32>) -> BTreeSet<(usize, usize)> {
    let mut b_size = 0;
    let mut res: BTreeSet<(usize, usize)> = BTreeSet::new();
    for (i, b) in disk_map.iter().enumerate() {
        if *b == -1 {
            b_size += 1;
        } else if b_size > 0 {
            res.insert((i - b_size, b_size));
            b_size = 0;
        }
    }
    res
}

fn optimize2(disk_map: &mut Vec<i32>) {
    let mut next_block_start: i32 = disk_map.len() as i32;
    let mut next_block_size = 0;
    let mut free_space = list_free_space(disk_map);
    find_next_to_move(disk_map, &mut next_block_start, &mut next_block_size);
    while next_block_start >= 0 {
        free_space
            .iter()
            .find(|(i, s)| *i < next_block_start as usize && *s >= next_block_size)
            .copied()
            .iter()
            .for_each(|(i, s)| {
                let id = disk_map[next_block_start as usize];
                for j in 0..(next_block_size) {
                    disk_map[i + j] = id;
                    disk_map[next_block_start as usize + j] = -1;
                }
                free_space.remove(&(*i, *s));
                if *s > next_block_size {
                    free_space.insert((*i + next_block_size, *s - next_block_size));
                }
            });
        find_next_to_move(disk_map, &mut next_block_start, &mut next_block_size);
    }
}

fn solve(input: String) -> i64 {
    let mut disk_map = parse_disk_map(&input);
    optimize2(&mut disk_map);

    disk_map
        .iter()
        .enumerate()
        .map(|(i, id)| if *id >= 0 { (id * i as i32) as i64 } else { 0 })
        .sum()
}
