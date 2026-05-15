use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

use super::shared::{get_modules, Module};

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        return 0;
    }
    (a / gcd(a, b)) * b
}

fn get_parents_rec(
    node_name: &String,
    modules: &HashMap<String, (Module, Vec<String>)>,
    result: &mut HashSet<String>,
) {
    for (parent_name, (_, o)) in modules.iter() {
        if o.contains(node_name) {
            if result.insert(parent_name.clone()) {
                get_parents_rec(parent_name, modules, result)
            }
        }
    }
}

pub fn answer() -> usize {
    let modules: &mut HashMap<String, (Module, Vec<String>)> = &mut get_modules();
    let output_module = "rx".to_string();

    let mut input_processes = modules
        .iter()
        .find_map(|(name, (_, o))| {
            o.contains(&output_module).then(|| {
                modules
                    .iter()
                    .filter_map(|(input_name, (_, input_o))| {
                        input_o.contains(name).then_some(input_name)
                    })
                    .map(|name| {
                        let mut parents = HashSet::new();
                        get_parents_rec(name, modules, &mut parents);
                        (
                            name.clone(),
                            parents,
                            HashMap::<Vec<Module>, usize>::new(), /*cache */
                        )
                    })
                    .collect_vec()
            })
        })
        .unwrap();

    let mut ciclicities = vec![None; input_processes.len()];
    let mut i = 0;
    loop {
        i += 1;
        let mut queue = VecDeque::from([("button".to_string(), false, "broadcaster".to_string())]);
        while let Some((from, signal, to)) = queue.pop_front() {
            for (process_i, (name, parents, ref mut cache)) in
                input_processes.iter_mut().enumerate()
            {
                if ciclicities[process_i].is_none() && from == **name && signal {
                    let cache_key = parents
                        .iter()
                        .flat_map(|name| modules.get(name).map(|(m, _)| m.clone()))
                        .collect();

                    if let Some(prev_i) = cache.get(&cache_key) {
                        ciclicities[process_i] = Some((*prev_i, i - prev_i));
                    } else {
                        cache.insert(cache_key, i);
                    }
                }
            }

            if ciclicities.iter().all(|a| a.is_some()) {
                return ciclicities
                    .iter()
                    .map(|a| a.unwrap().1)
                    .fold(1, |acc, x| lcm(acc, x));
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
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn should_compute_solution() {
        assert_eq!(answer(), 228060006554227);
    }
}
