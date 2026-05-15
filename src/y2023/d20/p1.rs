use std::collections::VecDeque;

use super::shared::{get_modules, Module};

pub fn answer() -> usize {
    let modules = &mut get_modules();
    let mut low_signals = 0;
    let mut high_signals = 0;

    for _ in 0..1000 {
        let mut queue = VecDeque::from([("button".to_string(), false, "broadcaster".to_string())]);
        while let Some((from, signal, to)) = queue.pop_front() {
            println!("{from} -{}-> {to}", if signal { "high" } else { "low" });
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
