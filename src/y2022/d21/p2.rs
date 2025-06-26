use std::collections::HashMap;

use crate::utils::read_lines;

enum Operation {
    Equality(String, String),
    Value(i64),
    Multiplication(String, String),
    Division(String, String),
    Addition(String, String),
    Subtration(String, String),
}

// check in both parts of root equation for humn usage

fn get_value_rec(name: &str, map: &HashMap<String, Operation>) -> i64 {
    map.get(name)
        .map(|o| match o {
            Operation::Addition(l, r) => get_value_rec(&l, map) + get_value_rec(&r, map),
            Operation::Division(l, r) => get_value_rec(&l, map) / get_value_rec(&r, map),
            Operation::Subtration(l, r) => get_value_rec(&l, map) - get_value_rec(&r, map),
            Operation::Multiplication(l, r) => get_value_rec(&l, map) * get_value_rec(&r, map),
            Operation::Value(l) => *l,
            Operation::Equality(_, _) => panic!(),
        })
        .unwrap()
}

fn get_inverted_operation<'a>(name: &str, map: &'a HashMap<String, Operation>) -> i64 {
    map.iter()
        .find_map(|(parent, operation)| match operation {
            Operation::Addition(l, r) => {
                if name == l {
                    Some(get_inverted_operation(&parent, map) - get_value_rec(r, map))
                } else if name == r {
                    Some(get_inverted_operation(&parent, map) - get_value_rec(l, map))
                } else {
                    None
                }
            }
            Operation::Division(l, r) => {
                if name == l {
                    Some(get_inverted_operation(&parent, map) * get_value_rec(r, map))
                } else if name == r {
                    Some(get_value_rec(l, map) / get_inverted_operation(&parent, map))
                } else {
                    None
                }
            }
            Operation::Subtration(l, r) => {
                if name == l {
                    Some(get_inverted_operation(&parent, map) + get_value_rec(r, map))
                } else if name == r {
                    Some(get_value_rec(l, map) - get_inverted_operation(&parent, map))
                } else {
                    None
                }
            }
            Operation::Multiplication(l, r) => {
                if name == l {
                    Some(get_inverted_operation(&parent, map) / get_value_rec(r, map))
                } else if name == r {
                    Some(get_inverted_operation(&parent, map) / get_value_rec(l, map))
                } else {
                    None
                }
            }
            Operation::Equality(l, r) => {
                if name == l {
                    Some(get_value_rec(r, map))
                } else if name == r {
                    Some(get_value_rec(l, map))
                } else {
                    None
                }
            }
            Operation::Value(_) => None,
        })
        .unwrap()
}

pub fn answer() -> i64 {
    let map = read_lines("src/y2022/d21/input")
        .map(|l| {
            let (name, operation_raw) = &l.split_at(6);
            let name = name[0..4].to_string();

            let operation = operation_raw.parse::<i64>().map_or_else(
                |_| {
                    let mut iter = operation_raw.split_whitespace();

                    let left_ref = iter.next().unwrap().to_string();
                    let operator_symbol = iter.next().unwrap();
                    let right_ref = iter.next().unwrap().to_string();

                    if name == "root" {
                        return Operation::Equality(left_ref, right_ref);
                    }

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

    get_inverted_operation("humn", &map)
}

#[cfg(test)]
mod tests {

    #[test]
    fn answer() {
        assert_eq!(super::answer(), 0);
    }
}
