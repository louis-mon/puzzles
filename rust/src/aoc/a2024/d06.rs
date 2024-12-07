use crate::aoc::common::input::{parse_int, parse_long, split_2};
use crate::read_day_input;

fn test_line(input: &str) -> i64 {
    let (res, ops_str) = split_2(input, ": ");
    let res = parse_long(res);
    let ops: Vec<i64> = ops_str.split_whitespace().map(parse_long).collect();
    if test_operation(res, ops[0], &ops[1..ops.len()]) {
        res
    } else {
        0
    }
}

fn sym_to_op(op: char) -> impl Fn(i64, i64) -> i64 {
    match op {
        '+' => |x, y| x + y,
        '*' => |x, y| x * y,
        '|' => |x, y| parse_long(&format!("{x}{y}")),
        _ => panic!("unsupported"),
    }
}

fn test_operation(target: i64, result: i64, operands: &[i64]) -> bool {
    match operands.first() {
        Some(first) => {
            for op in ['+', '*', '|'] {
                let applied = sym_to_op(op)(result, *first as i64);
                if test_operation(target, applied, &operands[1..operands.len()]) {
                    return true;
                }
            }
            false
        }
        None => target == result,
    }
}

fn solve(input: &str) -> i64 {
    input.lines().map(test_line).sum()
}

pub fn run() {
    println!("{}", solve(&read_day_input!()))
}
