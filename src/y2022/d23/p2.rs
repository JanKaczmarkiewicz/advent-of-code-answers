use std::collections::HashSet;

use crate::utils::read_lines;

pub fn answer() -> usize {
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
    let mut i = 0;

    loop {
        let proposed_moves = elves
            .iter()
            .filter(|elf| {
                dirs.iter()
                    .flatten()
                    .any(|(x, y)| elves.contains(&(elf.0 + x, elf.1 + y)))
            })
            .filter_map(|elf| {
                (0..dirs.len())
                    .map(|index| dirs[(index + i) % dirs.len()])
                    .find(|d| {
                        !d.iter()
                            .any(|(x, y)| elves.contains(&(elf.0 + x, elf.1 + y)))
                    })
                    .map(|dir| (*elf, (dir[0].0 + elf.0, dir[0].1 + elf.1)))
            })
            .collect::<Vec<_>>();

        if proposed_moves.is_empty() {
            return i;
        }

        let unique_proposed_moves = proposed_moves.iter().filter(|(_, p_outer)| {
            proposed_moves
                .iter()
                .filter(|(_, p_inner)| p_outer == p_inner)
                .count()
                == 1
        });

        for (elf, elf_next) in unique_proposed_moves {
            elves.remove(elf);
            elves.insert(*elf_next);
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_compute_solution() {
        assert_eq!(answer(), 1008);
    }
}
