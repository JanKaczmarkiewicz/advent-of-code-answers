use std::collections::HashSet;

use itertools::Itertools as _;

use crate::{
    utils::read_lines,
    y2023::d10::p1::{find_loop, get_cell},
};

pub fn answer() -> usize {
    let map = read_lines("src/y2023/d10/input_example")
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    let found_loop = find_loop(&map);

    let (min_col, max_col) = found_loop
        .iter()
        .map(|(_, col)| *col)
        .minmax()
        .into_option()
        .map(|(min, max)| (min - 1, max + 1))
        .unwrap();

    let (min_row, max_row) = found_loop
        .iter()
        .map(|(row, _)| *row)
        .minmax()
        .into_option()
        .map(|(min, max)| (min - 1, max + 1))
        .unwrap();

    println!("{:?}", ((min_col, max_col), (min_row, max_row)));

    let mut visited = HashSet::new();
    let mut to_visit = vec![(min_row, min_col)];

    while let Some(current) = to_visit.pop() {
        visited.insert(current);

        to_visit.extend(
            [
                (current.0 - 1, current.1),
                (current.0 + 1, current.1),
                (current.0, current.1 + 1),
                (current.0, current.1 - 1),
            ]
            .iter()
            .filter(|(row, col)| {
                (min_col..=max_col).contains(col)
                    && (min_row..=max_row).contains(row)
                    && !found_loop.contains(&(*row, *col))
                    && !visited.contains(&(*row, *col))
            }),
        );
    }

    let tiles = (min_col..=max_col)
        .flat_map(|col| (min_row..=max_row).map(move |row| (row, col)))
        .collect::<HashSet<_>>();

    let to_exclude = visited
        .union(&found_loop)
        .map(|a| *a)
        .collect::<HashSet<_>>();

    tiles
        .difference(&to_exclude)
        .filter_map(|cord| get_cell(&map, *cord).map(|cell| *cell == '.').map(|_| cord))
        .count()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_compute_solution() {
        assert_eq!(answer(), 10);
    }
}
