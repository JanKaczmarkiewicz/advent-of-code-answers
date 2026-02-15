use std::collections::HashSet;

use itertools::Itertools;

use crate::utils::read_lines;

pub fn get_cell(map: &Vec<Vec<char>>, (row, col): (i32, i32)) -> Option<&char> {
    if row >= 0 && col >= 0 {
        map.get(row as usize).and_then(|r| r.get(col as usize))
    } else {
        None
    }
}

fn find_start(map: &Vec<Vec<char>>) -> (i32, i32) {
    for (row_i, row) in map.iter().enumerate() {
        for (col_i, cell) in row.iter().enumerate() {
            if *cell == 'S' {
                return (row_i as i32, col_i as i32);
            }
        }
    }
    panic!()
}

pub fn find_loop(map: &Vec<Vec<char>>) -> HashSet<(i32, i32)> {
    let start = find_start(map);

    let mut paths = vec![(HashSet::new(), start)];

    while let Some((mut history, current)) = paths.pop() {
        if history.contains(&current) {
            if current == start && history.len() > 2 {
                return history;
            } else {
                continue;
            }
        }

        history.insert(current);

        let c = get_cell(map, current).unwrap();

        //up
        if let '|' | 'L' | 'J' | 'S' = c {
            let next = (current.0 - 1, current.1);

            if let Some('|' | 'F' | '7' | 'S') = get_cell(map, next) {
                paths.push((history.clone(), next))
            }
        }

        //down
        if let '|' | 'F' | '7' | 'S' = c {
            let next = (current.0 + 1, current.1);

            if let Some('|' | 'L' | 'J' | 'S') = get_cell(map, next) {
                paths.push((history.clone(), next))
            }
        }

        //right
        if let '-' | 'L' | 'F' | 'S' = c {
            let next = (current.0, current.1 + 1);

            if let Some('-' | 'J' | '7' | 'S') = get_cell(map, next) {
                paths.push((history.clone(), next))
            }
        }

        //left
        if let '-' | 'J' | '7' | 'S' = c {
            let next = (current.0, current.1 - 1);

            if let Some('-' | 'L' | 'F' | 'S') = get_cell(map, next) {
                paths.push((history.clone(), next))
            }
        }
    }

    panic!()
}

pub fn answer() -> f64 {
    let map = read_lines("src/y2023/d10/input")
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    find_loop(&map).len() as f64 / 2.0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_compute_solution() {
        assert_eq!(answer(), 0.0);
    }
}
