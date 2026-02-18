use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::utils::read_lines;

fn snap_to_direction(map: &mut Vec<Vec<char>>, direction: (i32, i32)) -> () {
    for column_i in 0..map[0].len() {
        let column_i = if direction.1.is_positive() {
            map[0].len() - 1 - column_i
        } else {
            column_i
        };
        for row_i in 0..map.len() {
            let row_i = if direction.0.is_positive() {
                map.len() - 1 - row_i
            } else {
                row_i
            };

            if map[row_i][column_i] != 'O' {
                continue;
            }

            map[row_i][column_i] = '.';
            for i in 1..i32::MAX {
                let next_cords = (
                    direction.0 * i + row_i as i32,
                    direction.1 * i + column_i as i32,
                );

                if !(0..map.len() as i32).contains(&next_cords.0)
                    || !(0..map[0].len() as i32).contains(&next_cords.1)
                    || ['O', '#'].contains(&map[next_cords.0 as usize][next_cords.1 as usize])
                {
                    map[(direction.0 * (i - 1) + row_i as i32) as usize]
                        [(direction.1 * (i - 1) + column_i as i32) as usize] = 'O';
                    break;
                }
            }
        }
    }
}

pub fn compute_weight(map: &Vec<Vec<char>>) -> usize {
    map.iter()
        .enumerate()
        .map(|(i, row)| (map.len() - i) * row.iter().filter(|c| **c == 'O').count())
        .sum()
}

pub fn answer() -> usize {
    let mut map = read_lines("src/y2023/d14/input")
        .map(|row| row.chars().collect_vec())
        .collect_vec();

    let mut cache = HashMap::new();

    let mut epochs = 1000000000;

    while epochs > 0 {
        let key = [(-1, 0), (0, -1), (1, 0), (0, 1)].map(|direction| {
            snap_to_direction(&mut map, direction);
            compute_weight(&map)
        });

        if let Some(i) = cache.get(&key) {
            //time skip
            epochs %= i - epochs;
        } else {
            cache.insert(key, epochs);
        }
        epochs -= 1;
    }

    compute_weight(&map)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_compute_solution() {
        assert_eq!(answer(), 0);
    }
}
