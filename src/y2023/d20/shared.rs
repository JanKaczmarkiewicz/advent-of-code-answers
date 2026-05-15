use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

use itertools::Itertools;

use crate::utils::read_lines;

#[derive(Clone, PartialEq, Eq)]
pub enum Module {
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
    Broadcaster,
}

impl Hash for Module {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Conjunction(h) => h
                .values()
                .for_each(|v| state.write_u8(if *v { 1 } else { 0 })),
            Self::FlipFlop(v) => state.write_u8(if *v { 1 } else { 0 }),
            _ => {}
        }
    }
}

pub fn parse_module_names(raw: &str) -> Vec<String> {
    raw.split(", ").map(|s| s.to_owned()).collect_vec()
}

pub fn get_modules() -> HashMap<String, (Module, Vec<String>)> {
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
