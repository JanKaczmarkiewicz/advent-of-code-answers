use std::collections::HashSet;

use crate::utils::read_lines;

pub fn answer() -> i64 {
    let mut elves = HashSet::new();

    let map = read_lines("src/y2022/d23/input").collect::<Vec<_>>();
    let map_height = map.len();

    for (i, line) in map.iter().enumerate() {
        let y = map_height as i32 - 1 - i as i32;
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert((x as i32, y));
            }
        }
    }

    let n = [(0, 1), (1, 1), (-1, 1)];
    let s = [(0, -1), (1, -1), (-1, -1)];
    let w = [(-1, 0), (-1, -1), (-1, 1)];
    let e = [(1, 0), (1, -1), (1, 1)];

    let dirs = [n, s, w, e];

    for _ in 0..10 {
        let proposed_moves = elves
            .iter()
            .filter(|elf| {
                dirs.iter()
                    .flatten()
                    .any(|(x, y)| elves.contains(&(elf.0 + x, elf.1 + y)))
            })
            .map(|elf| {
                dirs.iter()
                    .find(|d| {
                        !d.iter()
                            .any(|(x, y)| elves.contains(&(elf.0 + x, elf.1 + y)))
                    })
                    .unwrap()[0]
            })
            .collect::<Vec<_>>();

        // filter duplicates

        // mutate directions
    }

    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_compute_solution() {
        assert_eq!(answer(), 0);
    }
}
