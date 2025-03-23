use crate::aoc::common::input::{parse_int, parse_long, split_2_str};
use crate::read_day_input;
use std::collections::HashSet;

pub fn run() {
    println!("{}", find_reg_a(parse_input(read_day_input!())))
}

#[derive(Clone)]
struct Registers {
    instr: i8,
    reg: Vec<i64>,
}

type Program = Vec<(u8, u8)>;

struct Input {
    reg: Registers,
    prog: Program,
    raw_data: Vec<u8>,
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct ProgState {
    a: i64,
    b: i64,
    c: i64,
    p: i8,
    o: i8,
}

fn parse_input(input: String) -> Input {
    let (reg, instr) = split_2_str(&input, "\n\n");
    let reg: Vec<i64> = reg
        .lines()
        .map(|l| parse_long(l.split_whitespace().skip(2).next().unwrap()))
        .collect();
    let (_, instr) = split_2_str(&instr, " ");
    let instr_list: Vec<u8> = instr.split(",").map(|v| v.parse::<u8>().unwrap()).collect();
    let prog: Program = (0..instr_list.len() / 2)
        .map(|i| (instr_list[i * 2], instr_list[i * 2 + 1]))
        .collect();
    Input {
        reg: Registers { instr: 0, reg },
        prog,
        raw_data: instr_list,
    }
}

fn find_reg_a(input: Input) -> i64 {
    //let mut res = 1144000000000;
    let mut res = 0;
    let mut failed_states: HashSet<ProgState> = HashSet::new();
    while !f1(res) && res < 100000 {
        if res % 1000000000 == 0 {
            println!("{res}");
        }
        res += 1;
    }
    res
}

fn run_program(input: &Input, failed_states: &mut HashSet<ProgState>, a: i64) -> bool {
    let mut reg = input.reg.clone();
    reg.reg[0] = a;

    let mut output: Vec<u8> = Default::default();
    // let mut local_states = HashSet::<ProgState>::new();
    let mut failed = false;

    while reg.instr < input.prog.len() as i8 {
        let (opcode, operand) = input.prog[reg.instr as usize];
        /*let state = ProgState {
            a: reg.reg[0],
            b: reg.reg[1],
            c: reg.reg[2],
            o: output.len() as i8,
            p: reg.instr,
        };
        if failed_states.contains(&state) {
            failed = true;
            break;
        }
        local_states.insert(state);*/
        let combo = || -> i64 {
            if operand < 4 {
                operand as i64
            } else {
                reg.reg[operand as usize - 4]
            }
        };
        match opcode {
            0 => {
                reg.reg[0] = reg.reg[0] / (1 << combo());
            }
            1 => reg.reg[1] = reg.reg[1] ^ operand as i64,
            2 => reg.reg[1] = combo() % 8,
            3 => {
                if reg.reg[0] > 0 {
                    reg.instr = operand as i8 - 1;
                }
            }
            4 => reg.reg[1] = reg.reg[1] ^ reg.reg[2],
            5 => {
                let new_val = (combo() % 8) as u8;
                if input.raw_data.len() <= output.len() || new_val != input.raw_data[output.len()] {
                    failed = true;
                    break;
                }
                output.push(new_val);
            }
            6 => reg.reg[1] = reg.reg[0] / (1 << combo()),
            7 => reg.reg[2] = reg.reg[0] / (1 << combo()),
            _ => panic!("invalid opcode"),
        };
        reg.instr += 1;
    }
    if output.len() != input.raw_data.len() {
        failed = true;
    }
    /*if failed {
        local_states.iter().for_each(|s| {failed_states.insert(s.clone()); });
    }*/
    !failed
}

/*

Program:
2,4 -
1,2 -
7,5 -
4,3 -
0,3 -
1,7 -
5,5 -
3,0
 */

const TARGET1: [i8; 16] = [2, 4, 1, 2, 7, 5, 4, 3, 0, 3, 1, 7, 5, 5, 3, 0];

fn f1(a0: i64) -> bool {
    let mut a = a0;
    let mut b = 0;
    let mut c = 0;
    let mut res_size = 0;
    let mut res = [0i8; 16];
    loop {
        b = a & 7;
        b = b ^ 2;
        c = a >> b;
        b = b ^ c;
        a = a >> 3;
        b = b ^ 7;
        res_size += 1;
        let i = (b & 7) as i8;
        /*if res_size > TARGET1.len() || i != TARGET1[res_size -1] {
            break;
        }*/
        res[res_size - 1] = i;
        if res_size == 1 && i == TARGET1[res_size - 1] {
            println!("{res_size}: {}", a0)
        }
        if a == 0 {
            break;
        }
    }
    res_size == TARGET1.len()
}
