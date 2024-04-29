use crate::utils::read;

pub fn answer() {
    println!("Answer to day11: {}", a1());
}

#[derive(Debug, PartialEq)]
enum OperationValue {
    Number(usize),
    State,
}

#[derive(Debug, PartialEq)]
enum Operation {
    Addition(OperationValue, OperationValue),
    Multiplication(OperationValue, OperationValue),
}

#[derive(Debug, PartialEq)]
struct Monkey {
    items: Vec<usize>,
    decision: (usize, usize, usize),
    operation_performed: usize,
    operation: Operation,
}

fn parse_monkeys(text: &str) -> Vec<Monkey> {
    text.split("Monkey")
        .skip(1)
        .map(|monkey| {
            let mut lines = monkey.split("\n");
            lines.next();

            let mut items = lines
                .next()
                .map(|s| {
                    String::from_utf8_lossy(&s.as_bytes()[18..])
                        .split(", ")
                        .map(|num| num.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .unwrap();

            items.reverse();

            let operation = lines
                .next()
                .map(|s| {
                    let binding = String::from_utf8_lossy(&s.as_bytes()[19..]);
                    let mut operation_parts = binding.split(" ");
                    let left = operation_parts
                        .next()
                        .map(|raw| {
                            raw.parse()
                                .map(OperationValue::Number)
                                .unwrap_or(OperationValue::State)
                        })
                        .unwrap();

                    let operation_part = operation_parts.next().unwrap();

                    let right = operation_parts
                        .next()
                        .map(|raw| {
                            raw.parse()
                                .map(OperationValue::Number)
                                .unwrap_or(OperationValue::State)
                        })
                        .unwrap();

                    match operation_part {
                        "*" => Operation::Multiplication(left, right),
                        "+" => Operation::Addition(left, right),
                        _ => panic!(),
                    }
                })
                .unwrap();

            let decision_criteria = lines
                .next()
                .map(|s| {
                    String::from_utf8_lossy(&s.as_bytes()[21..])
                        .parse::<usize>()
                        .unwrap()
                })
                .unwrap();

            let target_true = lines
                .next()
                .map(|s| {
                    String::from_utf8_lossy(&s.as_bytes()[29..])
                        .parse::<usize>()
                        .unwrap()
                })
                .unwrap();

            let target_false = lines
                .next()
                .map(|s| {
                    String::from_utf8_lossy(&s.as_bytes()[30..])
                        .parse::<usize>()
                        .unwrap()
                })
                .unwrap();

            Monkey {
                items,
                operation,
                operation_performed: 0,
                decision: (decision_criteria, target_true, target_false),
            }
        })
        .collect()
}

pub fn a1() -> usize {
    let text = read("src/y2022/d11/input");

    let mut monkeys = parse_monkeys(&text);

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            for item_i in (0..monkeys[i].items.len()).rev() {
                monkeys[i].operation_performed += 1;

                monkeys[i].items[item_i] = match &monkeys[i].operation {
                    Operation::Addition(l, r) => {
                        let l = match l {
                            OperationValue::Number(n) => n,
                            OperationValue::State => &monkeys[i].items[item_i],
                        };

                        let r = match r {
                            OperationValue::Number(n) => n,
                            OperationValue::State => &monkeys[i].items[item_i],
                        };

                        *l + *r
                    }
                    Operation::Multiplication(l, r) => {
                        let l = match l {
                            OperationValue::Number(n) => n,
                            OperationValue::State => &monkeys[i].items[item_i],
                        };

                        let r = match r {
                            OperationValue::Number(n) => n,
                            OperationValue::State => &monkeys[i].items[item_i],
                        };

                        *l * *r
                    }
                } / 3;

                let new_item = monkeys[i].items[item_i];

                let target_monkey = if new_item % monkeys[i].decision.0 == 0 {
                    monkeys[i].decision.1
                } else {
                    monkeys[i].decision.2
                };

                monkeys[i].items.pop();
                monkeys[target_monkey].items.insert(0, new_item);
            }
        }
    }

    let mut max1 = 0;

    for Monkey {
        operation_performed,
        ..
    } in &monkeys
    {
        if *operation_performed > max1 {
            max1 = *operation_performed;
        }
    }

    let mut max2 = 0;

    for Monkey {
        operation_performed,
        ..
    } in monkeys
    {
        if operation_performed > max2 && operation_performed != max1 {
            max2 = operation_performed;
        }
    }

    return max1 * max2;
}

pub fn a2() -> usize {
    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_parse_monkeys() {
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0";
        let monkeys = parse_monkeys(input);

        assert_eq!(
            monkeys,
            [
                Monkey {
                    items: [98, 79].to_vec(),
                    decision: (23, 2, 3),
                    operation_performed: 0,
                    operation: Operation::Multiplication(
                        OperationValue::State,
                        OperationValue::Number(19)
                    )
                },
                Monkey {
                    items: [74, 75, 65, 54].to_vec(),
                    decision: (19, 2, 0),
                    operation_performed: 0,
                    operation: Operation::Addition(
                        OperationValue::State,
                        OperationValue::Number(6)
                    )
                }
            ]
        );
    }

    #[test]
    fn should_solve_first_problem() {
        assert_eq!(a1(), 0);
    }

    #[test]
    fn should_solve_second_problem() {
        assert_eq!(a2(), 0);
    }
}
