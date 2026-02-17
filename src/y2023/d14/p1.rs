use std::ops::Sub;

use itertools::Itertools;

use crate::utils::read_lines;

pub fn answer() -> usize {
    let mut map = read_lines("src/y2023/d14/input")
        .map(|row| row.chars().collect_vec())
        .collect_vec();

    for column_i in 0..map[0].len() {
        for row_i in 0..map.len() {
            if map[row_i][column_i] != 'O' {
                continue;
            }

            map[row_i][column_i] = '.';
            for i in 1..usize::MAX {
                if row_i.checked_sub(i).is_none() || ['O', '#'].contains(&map[row_i - i][column_i])
                {
                    map[(row_i + 1) - i][column_i] = 'O';
                    break;
                }
            }
        }
    }

    for row in &map {
        println!("{}", row.into_iter().join(""));
    }

    map.iter()
        .enumerate()
        .map(|(i, row)| (map.len() - i) * row.iter().filter(|c| **c == 'O').count())
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_compute_solution() {
        assert_eq!(answer(), 0);
    }
}
