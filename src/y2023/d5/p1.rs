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

pub fn answer() -> usize {
    let mut lines = read_lines("src/y2023/d5/input");

    let mut inputs = lines
        .next()
        .unwrap()
        .replace("seeds: ", "")
        .split(" ")
        .map(|r| r.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    lines.next();

    while let Some(mapping) = read_map(&mut lines) {
        for input in inputs.iter_mut() {
            if let Some(new_input) = mapping.iter().find_map(|(dest, source, range)| {
                (*source..(source + range))
                    .contains(&input)
                    .then(|| dest + *input - source)
            }) {
                *input = new_input;
            }
        }
    }

    *inputs.iter().min().unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_compute_solution() {
        assert_eq!(answer(), 0);
    }
}
