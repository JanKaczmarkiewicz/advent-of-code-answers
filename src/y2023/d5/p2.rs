use itertools::Itertools;

use crate::utils::read_lines;

fn read_map(lines: &mut impl Iterator<Item = String>) -> Option<Vec<(usize, usize, usize)>> {
    lines.next().map(|_| {
        lines
            .take_while(|line| line.starts_with(|e: char| e.is_alphanumeric()))
            .map(|line| {
                line.split(" ")
                    .map(|r| r.parse::<usize>().unwrap())
                    .next_tuple()
                    .unwrap()
            })
            .collect::<Vec<_>>()
    })
}

fn read_seeds(lines: &mut impl Iterator<Item = String>) -> Vec<(usize, usize)> {
    let mut inputs = vec![];

    let raw = lines.next().unwrap().replace("seeds: ", "");

    let mut inputs_iter = raw.split(" ").map(|r| r.parse::<usize>().unwrap());

    while let Some((s, r)) = inputs_iter.next_tuple() {
        inputs.push((s, r));
    }

    lines.next();

    inputs
}

pub fn answer() -> usize {
    let mut lines = read_lines("src/y2023/d5/input");

    let mut inputs = read_seeds(&mut lines);

    while let Some(mapping) = read_map(&mut lines) {
        let mut next_inputs = vec![];

        for (input_source, input_range) in inputs.iter_mut() {
            while *input_range != 0 {
                let mapped_input = if let Some((group_dest, group_source, group_range)) = mapping
                    .iter()
                    .find(|(_, source, range)| (*source..(source + range)).contains(&input_source))
                {
                    (
                        *group_dest + *input_source - *group_source,
                        (group_source + group_range - *input_source).min(*input_range),
                    )
                } else if let Some(distance) = mapping
                    .iter()
                    .map(|(_, source, _)| *source as i64 - *input_source as i64)
                    .find(|distance| !distance.is_negative() && *distance as usize <= *input_range)
                    .map(|distance| distance as usize)
                {
                    (*input_source, distance)
                } else {
                    (*input_source, *input_range)
                };
                *input_source += mapped_input.1;
                *input_range -= mapped_input.1;

                next_inputs.push(mapped_input);
            }
        }

        inputs = next_inputs;
    }

    *inputs.iter().map(|(source, _)| source).min().unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_compute_solution() {
        assert_eq!(answer(), 78775051);
    }
}
