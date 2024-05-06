use std::cmp::Ordering;

use itertools::Itertools;

use crate::utils::read_lines;

pub fn answer() {
    println!("Answer to day13: {}", a1());
}

#[derive(PartialEq, Debug, Clone)]
enum Value {
    List(Vec<Value>),
    Number(i32),
}

fn split_by_first_level_comma(str: &str) -> Vec<&str> {
    if str == "" {
        return vec![];
    }

    let mut values = vec![];

    let mut depth_level = 0;
    let mut index_start_of_new_value = 0;

    for (index, c) in str.chars().enumerate() {
        match c {
            ',' => {
                if depth_level == 0 {
                    values.push(&str[index_start_of_new_value..index]);
                    index_start_of_new_value = index + 1; // after comma
                }
            }
            '[' => depth_level += 1,
            ']' => depth_level -= 1,
            _ => {}
        }
    }

    // commit last part
    values.push(&str[index_start_of_new_value..]);

    values
}

fn parse_value(str: &str) -> Value {
    if str.chars().nth(0) == Some('[') {
        Value::List(
            split_by_first_level_comma(&str[1..str.len() - 1])
                .into_iter()
                .map(|v| parse_value(v))
                .collect::<Vec<_>>(),
        )
    } else {
        Value::Number(str.parse().unwrap())
    }
}

fn compare_values(left: &Value, right: &Value) -> Option<bool> {
    match (left, right) {
        (Value::Number(l), Value::Number(r)) => (l != r).then_some(l < r),
        (Value::List(l), Value::List(r)) => {
            let l_len = l.len();
            let r_len = r.len();

            if let Some(res) = l.into_iter().zip(r).find_map(|(l, r)| compare_values(l, r)) {
                Some(res)
            } else {
                (l_len != r_len).then_some(l_len < r_len)
            }
        }
        (Value::Number(l), Value::List(_)) => {
            compare_values(&Value::List(vec![Value::Number(*l)]), &right.clone())
        }
        (Value::List(_), Value::Number(l)) => {
            compare_values(&left.clone(), &Value::List(vec![Value::Number(*l)]))
        }
    }
}

fn a1() -> usize {
    let (lefts, rights): (Vec<_>, Vec<_>) = read_lines("src/y2022/d13/input")
        .filter(|line| line != "")
        .map(|line| parse_value(&line))
        .enumerate()
        .partition(|(i, _)| i % 2 == 0);

    lefts
        .into_iter()
        .zip(rights)
        .enumerate()
        .filter_map(|(index, ((_, left), (_, right)))| {
            if let Some(result) = compare_values(&left, &right) {
                if result {
                    Some(index + 1)
                } else {
                    None
                }
            } else {
                panic!();
            }
        })
        .sum()
}

fn a2() -> usize {
    let dividers = [
        Value::List(vec![Value::List(vec![Value::Number(2)])]),
        Value::List(vec![Value::List(vec![Value::Number(6)])]),
    ];

    let sorted: Vec<_> = read_lines("src/y2022/d13/input")
        .filter(|line| line != "")
        .map(|line| parse_value(&line))
        .chain(dividers.clone())
        .sorted_by(|left, right| {
            if let Some(result) = compare_values(left, right) {
                if result {
                    return Ordering::Less;
                } else {
                    return Ordering::Greater;
                }
            } else {
                return Ordering::Equal;
            }
        })
        .collect();

    let div_pos_1 = sorted
        .iter()
        .position(|value| value == &dividers[0])
        .unwrap();

    let div_pos_2 = sorted
        .iter()
        .position(|value| value == &dividers[1])
        .unwrap();

    (div_pos_1 + 1) * (div_pos_2 + 1)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_solve_first_problem() {
        assert_eq!(a1(), 0);
    }

    #[test]
    fn parse_value_test() {
        assert_eq!(
            parse_value("[[1],2,[10,[212,[1]],[30],[22]]]"),
            Value::List(vec![
                Value::List(vec![Value::Number(1)]),
                Value::Number(2),
                Value::List(vec![
                    Value::Number(10),
                    Value::List(vec![
                        Value::Number(212),
                        Value::List(vec![Value::Number(1)])
                    ]),
                    Value::List(vec![Value::Number(30)]),
                    Value::List(vec![Value::Number(22)])
                ])
            ])
        );
    }

    #[test]
    fn should_solve_second_problem() {
        assert_eq!(a2(), 21582);
    }
}
