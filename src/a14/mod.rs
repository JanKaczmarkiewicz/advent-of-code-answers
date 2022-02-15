use std::collections::HashMap;
use itertools::Itertools;
use crate::utils::read_lines;

fn a() -> u32 {
    let lines = read_lines("src/a14/input").collect::<Vec<_>>();

    let mut instructions = HashMap::new();

    for line in  &lines[2..lines.len()] {
        let instruction_parts = line.split(" -> ").collect::<Vec<_>>();
        instructions.insert(instruction_parts[0], instruction_parts[1]);
    }

    let mut template = HashMap::new();

    let initial_template = lines[0].to_owned();

    (0..initial_template.len()-1)
        .map(|i| &initial_template[i..i+2])
        .for_each(|part| {
            *template.entry(String::from(part)).or_insert(0) += 1;
        });

    for _i in 0..10 {
        let mut result_template = HashMap::new();

        for (key, value) in &template {

            let value = *value;

            let template_part = *instructions.get(key.as_str()).unwrap();

            // left addition
            let mut part1 = String::from(&key[0..1]);
            part1.push_str(template_part);
            *result_template.entry(part1).or_insert(0) += value;

            // right addition
            let mut part2 = String::from(template_part);
            part2.push_str(&key[1..2]);
            *result_template.entry(part2).or_insert(0) += value;
        }

        template = result_template;
    }

    let mut chars = HashMap::new();

    for (key, value) in template {
        for char in key.chars() {
            *chars.entry(char).or_insert(0) += value;
        }
    }

    let (min, max) = chars
        .iter()
        .map(|(_key, value)| ((*value as f32) / 2 as f32).ceil() as u32)
        .minmax()
        .into_option()
        .unwrap();

     max - min
}

pub fn answer() {
    println!("Answer to problem 14: {}", a());
}


#[cfg(test)]
mod tests {
    use super::{a};

    #[test]
    fn should_solve_first_problem() {
        assert_eq!(a(), 3009);
    }

}