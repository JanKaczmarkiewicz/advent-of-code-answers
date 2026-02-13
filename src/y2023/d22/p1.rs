use std::{collections::HashSet, iter::zip};

use itertools::Itertools;

use crate::utils::read_lines;

type Brick = ((i32, i32, i32), (i32, i32, i32));

fn space((start, end): Brick) -> HashSet<(i32, i32, i32)> {
    if start.0 < end.0 {
        HashSet::from_iter((start.0..=end.0).map(|i| (start.0 + i, start.1, start.2)))
    } else if start.1 < end.1 {
        HashSet::from_iter((start.1..=end.1).map(|i| (start.0, start.1 + i, start.2)))
    } else if start.2 < end.2 {
        HashSet::from_iter((start.2..=end.2).map(|i| (start.0, start.1, start.2 + i)))
    } else {
        HashSet::from([start])
    }
}

fn check_sum(bricks: &Vec<Brick>) -> i32 {
    bricks
        .iter()
        .map(|((s1, s2, s3), (e1, e2, e3))| s1 + s2 + s3 + e1 + e2 + e3)
        .sum::<i32>()
}

pub fn stabilize(bricks: &mut Vec<Brick>) {
    let mut prev_check_sum = 0;

    loop {
        let current_check_sum = check_sum(&bricks);
        if prev_check_sum == current_check_sum {
            break;
        } else {
            prev_check_sum = current_check_sum;
        }

        // TODO
        for i in 0..bricks.len() {
            if bricks[i].0 .2 == 1 {
                continue;
            }

            bricks[i].0 .2 -= 1;
            bricks[i].1 .2 -= 1;

            for j in 0..bricks.len() {
                if j == i {
                    continue;
                }
                if space(bricks[i]).intersection(&space(bricks[j])).count() != 0 {
                    bricks[i].0 .2 += 1;
                    bricks[i].1 .2 += 1;
                    break;
                }
            }
        }
    }
}

pub fn initial_bricks() -> Vec<Brick> {
    let mut bricks = read_lines("src/y2023/d22/input")
        .map(|str| {
            let (start_raw, end_raw) = str.split_once('~').unwrap();

            let start = start_raw
                .split(',')
                .map(|r| r.parse::<i32>().unwrap())
                .collect_tuple::<(_, _, _)>()
                .unwrap();

            let end = end_raw
                .split(',')
                .map(|r| r.parse::<i32>().unwrap())
                .collect_tuple::<(_, _, _)>()
                .unwrap();

            (start, end)
        })
        .collect_vec();

    stabilize(&mut bricks);

    bricks
}

pub fn answer() -> usize {
    let bricks = initial_bricks();

    (0..bricks.len())
        .filter(|disabled| {
            let mut other_bricks = bricks.clone();
            other_bricks.remove(*disabled);
            let other_bricks_not_stabilized = other_bricks.clone();
            stabilize(&mut other_bricks);

            zip(other_bricks_not_stabilized, other_bricks).all(|(l, r)| l == r)
        })
        .count()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_compute_solution() {
        assert_eq!(answer(), 0);
    }
}
