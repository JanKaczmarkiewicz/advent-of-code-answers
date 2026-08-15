use std::{collections::HashSet, hash::RandomState, print, println, vec};

use itertools::Itertools;

use crate::utils::read_lines;

pub fn answer() -> i32 {
    let mut gx = vec![];

    read_lines("src/y2023/d11/input")
        .enumerate()
        .for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                if c == '#' {
                    gx.push((x, y))
                }
            })
        });

    let (min_x, max_x) = gx.iter().map(|(x, _)| x).minmax().into_option().unwrap();
    let (min_y, max_y) = gx.iter().map(|(_, x)| x).minmax().into_option().unwrap();

    for x in (*min_x..*max_x) {
        for y in (*min_y..*max_y) {
            print!(
                "{}",
                if let Some(_) = gx.iter().find(|c| **c == (x, y)) {
                    "#"
                } else {
                    "."
                }
            )
        }
        println!("")
    }

    let x_space =
        HashSet::<_, RandomState>::from_iter(0..gx.iter().map(|(x, _)| *x).max().unwrap())
            .difference(&HashSet::<_, RandomState>::from_iter(
                gx.iter().map(|(x, _)| *x),
            ))
            .map(|i| *i)
            .collect::<Vec<_>>();

    // n = epochs
    // how mych space is from the left? answer:c
    // -> c * 2^n

    let y_space =
        HashSet::<_, RandomState>::from_iter(0..gx.iter().map(|(_, y)| *y).max().unwrap())
            .difference(&HashSet::<_, RandomState>::from_iter(
                gx.iter().map(|(_, y)| *y),
            ))
            .map(|i| *i)
            .collect::<Vec<_>>();

    let n = 1;

    for (x, y) in gx.iter_mut() {
        *x += x_space.iter().filter(|i| **i < *x).count() * 2 ^ n;
        *y += y_space.iter().filter(|i| **i < *y).count() * 2 ^ n;
    }

    let mut sum_distance = 0;

    for (i, (i_x, i_y)) in gx.iter().enumerate() {
        for (j_x, j_y) in gx[i + 1..].iter() {
            sum_distance += (*j_x as i32 - *i_x as i32).abs() + (*j_y as i32 - *i_y as i32).abs()
        }
    }

    let (min_x, max_x) = gx.iter().map(|(x, _)| x).minmax().into_option().unwrap();
    let (min_y, max_y) = gx.iter().map(|(_, x)| x).minmax().into_option().unwrap();

    for x in (*min_x..*max_x) {
        for y in (*min_y..*max_y) {
            print!(
                "{}",
                if let Some(_) = gx.iter().find(|c| **c == (x, y)) {
                    "#"
                } else {
                    "."
                }
            )
        }
        println!("")
    }
    // find h & v spaces between galaxies
    // every one of those will expand exponentially (1 2 4 8 16...)
    // causing all galaxies on the left and right to change their cords
    // no need to alter galaxies more than one time
    // compute shortest paths between pairs

    sum_distance
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_compute_solution() {
        assert_eq!(answer(), 0);
    }
}
