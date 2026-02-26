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

// fn send_signal(
//     modules: &mut HashMap<String, (Module, Vec<String>)>,
//     to: &str,
//     from: &str,
//     signal: bool,
// ) -> usize {
//     let (m, o) = modules.get_mut(to).unwrap();

//     let next_signal = match m {
//         Module::Broadcaster => signal,
//         Module::Conjunction(v) => {
//             *v.get_mut(from).unwrap() = signal;
//             v.values().all(|e| *e)
//         }
//         Module::FlipFlop(state) => {
//             if signal == false {
//                 *state = !*state;
//             }
//             *state
//         }
//     };

//     o.clone()
//         .iter()
//         .map(|dest| {
//             //button -low-> broadcaster
//             println!(
//                 "{from} -{}-> {to}",
//                 if next_signal { "high" } else { "low" }
//             );

//             1 + send_signal(modules, &dest, to, next_signal)
//         })
//         .sum()
// }

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
            .map(|n| (n.to_owned(), true))
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
    let mut queue = VecDeque::from([("button".to_string(), false, "broadcaster".to_string())]);
    let mut i = 0;
    while let Some((from, signal, to)) = queue.pop_front() {
        println!("{from} -{}-> {to}", if signal { "high" } else { "low" });

        i += 1;
        let (m, outputs) = modules.get_mut(&to).unwrap();

        let next_signal = match m {
            Module::Broadcaster => signal,
            Module::Conjunction(v) => {
                *v.get_mut(&from).unwrap() = signal;

                !v.values().all(|e| *e)
            }
            Module::FlipFlop(state) => {
                if signal == false {
                    *state = !*state;
                } else {
                    continue;
                }
                *state
            }
        };

        for dest in outputs {
            queue.push_back((to.to_string(), next_signal, dest.clone()));
        }
    }

    i
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn should_compute_solution() {
        assert_eq!(answer(), 0);
    }
}
