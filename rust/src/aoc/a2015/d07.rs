use crate::aoc::a2015::d07::Expr::Value;
use std::collections::HashMap;
use std::fs;

pub fn run() {
    println!(
        "{}",
        solve2(fs::read_to_string("src/aoc/a2015/d07.input").unwrap())
    );
}

#[derive(Debug, Clone)]
enum Expr {
    Value {
        var: String,
    },
    Unary {
        var: String,
    },
    Binary {
        lhs: String,
        rhs: String,
        op: String,
    },
}

fn parse_command(cmd: &str) -> (String, Expr) {
    let v: Vec<&str> = cmd.split(" -> ").collect();
    let deps = v[0];
    let var = v[1];
    let ops: Vec<&str> = deps.split_whitespace().collect();
    let expr = {
        if ops.len() == 1 {
            Expr::Value {
                var: String::from(ops[0]),
            }
        } else if ops.len() == 2 {
            Expr::Unary {
                var: String::from(ops[1]),
            }
        } else {
            Expr::Binary {
                lhs: String::from(ops[0]),
                rhs: String::from(ops[2]),
                op: String::from(ops[1]),
            }
        }
    };
    (String::from(var), expr)
}

type ComputeGraph = HashMap<String, Expr>;

fn compute_value(graph: &mut ComputeGraph, var: &str) -> u16 {
    match var.parse::<u16>() {
        Ok(val) => val,
        _ => {
            let expr = graph.get(var).unwrap().clone();
            let expr_val = match expr {
                Expr::Unary { var } => !compute_value(graph, &var),
                Expr::Binary { lhs, rhs, op } => {
                    let rhs_val = compute_value(graph, &rhs);
                    let lhs_val = compute_value(graph, &lhs);
                    match op.as_str() {
                        "AND" => rhs_val & lhs_val,
                        "OR" => rhs_val | lhs_val,
                        "LSHIFT" => lhs_val << rhs_val,
                        "RSHIFT" => lhs_val >> rhs_val,
                        _ => panic!("Unknown operator {}", op),
                    }
                }
                Value { var } => compute_value(graph, &var),
            };
            graph.insert(
                String::from(var),
                Value {
                    var: expr_val.to_string(),
                },
            );
            expr_val
        }
    }
}

fn solve(text: String) -> u16 {
    let mut graph: ComputeGraph = text.lines().map(parse_command).collect();

    compute_value(&mut graph, "a")
}

fn solve2(text: String) -> u16 {
    let mut graph: ComputeGraph = text.lines().map(parse_command).collect();

    let a = compute_value(&mut graph.clone(), "a");
    graph.insert(String::from("b"), Value { var: a.to_string() });
    compute_value(&mut graph, "a")
}
