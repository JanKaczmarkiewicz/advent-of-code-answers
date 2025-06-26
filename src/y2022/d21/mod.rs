use std::collections::HashMap;

use crate::utils::read_lines;

pub mod p2;

pub fn answer() {
    println!("Answer to day21a: {}", a1());
}

enum Operation {
    Value(i64),
    Multiplication(String, String),
    Division(String, String),
    Addition(String, String),
    Subtration(String, String),
}

fn get_value_rec(name: &str, map: &HashMap<String, Operation>) -> i64 {
    map.get(name)
        .map(|o| match o {
            Operation::Addition(l, r) => get_value_rec(&l, map) + get_value_rec(&r, map),
            Operation::Division(l, r) => get_value_rec(&l, map) / get_value_rec(&r, map),
            Operation::Subtration(l, r) => get_value_rec(&l, map) - get_value_rec(&r, map),
            Operation::Multiplication(l, r) => get_value_rec(&l, map) * get_value_rec(&r, map),
            Operation::Value(l) => *l,
        })
        .unwrap()
}

fn a1() -> i64 {
    let map = read_lines("src/y2022/d21/input")
        .map(|l| {
            let (name, operation_raw) = &l.split_at(6);

            let operation = operation_raw.parse::<i64>().map_or_else(
                |_| {
                    let mut iter = operation_raw.split_whitespace();

                    let left_ref = iter.next().unwrap().to_string();
                    let operator_symbol = iter.next().unwrap();
                    let right_ref = iter.next().unwrap().to_string();

                    match operator_symbol {
                        "*" => Operation::Multiplication(left_ref, right_ref),
                        "/" => Operation::Division(left_ref, right_ref),
                        "-" => Operation::Subtration(left_ref, right_ref),
                        "+" => Operation::Addition(left_ref, right_ref),
                        _ => panic!(),
                    }
                },
                |value| Operation::Value(value),
            );

            (name[0..4].to_string(), operation)
        })
        .collect::<HashMap<_, _>>();

    get_value_rec("root", &map)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_solve_first_problem() {
        assert_eq!(a1(), 56490240862410);
    }
}
