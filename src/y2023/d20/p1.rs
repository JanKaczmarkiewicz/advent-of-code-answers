use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

use crate::utils::read_lines;

enum Module {
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
    Broadcaster,
}

fn parse_module_names(raw: &str) -> Vec<String> {
    raw.split(", ").map(|s| s.to_owned()).collect_vec()
}

fn get_modules() -> HashMap<String, (Module, Vec<String>)> {
    let mut conjuntion_modules = vec![];

    let mut modules = read_lines("src/y2023/d20/input")
        .map(|line| {
            let (module, rest) = if let Some(r) = line.strip_prefix("&") {
                conjuntion_modules.push(r.split(" -> ").next().unwrap().to_string());
                (Module::Conjunction(HashMap::new()), r)
            } else if let Some(r) = line.strip_prefix("%") {
                (Module::FlipFlop(false), r)
            } else {
                (Module::Broadcaster, line.as_str())
            };

            let (name, output) = rest.split_once(" -> ").unwrap();

            (name.to_string(), (module, parse_module_names(output)))
        })
        .collect::<HashMap<_, _>>();

    conjuntion_modules.into_iter().for_each(|c_name| {
        let new_inputs = modules
            .iter()
            .filter_map(|(input_name, (_, outputs))| {
                outputs.iter().find(|n| *n == &c_name).map(|_| input_name)
            })
            .map(|n| (n.to_owned(), false))
            .collect();

        if let (Module::Conjunction(inputs), _) = modules.get_mut(&c_name).unwrap() {
            *inputs = new_inputs;
        } else {
            panic!()
        }
    });

    modules
}

pub fn answer() -> usize {
    let modules = &mut get_modules();
    let mut low_signals = 0;
    let mut high_signals = 0;

    for _ in 0..1000 {
        let mut queue = VecDeque::from([("button".to_string(), false, "broadcaster".to_string())]);
        while let Some((from, signal, to)) = queue.pop_front() {
            // println!("{from} -{}-> {to}", if signal { "high" } else { "low" });
            if signal {
                high_signals += 1;
            } else {
                low_signals += 1;
            }

            if let Some((m, outputs)) = modules.get_mut(&to) {
                let next_signal = match m {
                    Module::Broadcaster => signal,
                    Module::Conjunction(v) => {
                        *v.get_mut(&from).unwrap() = signal;
                        !v.values().all(|e| *e)
                    }
                    Module::FlipFlop(state) => {
                        if signal {
                            continue;
                        }
                        *state = !*state;
                        *state
                    }
                };

                for dest in outputs {
                    queue.push_back((to.to_string(), next_signal, dest.clone()));
                }
            }
        }
        println!()
    }

    low_signals * high_signals
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn should_compute_solution() {
        assert_eq!(answer(), 806332748);
    }
}
