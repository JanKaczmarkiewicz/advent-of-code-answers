use std::collections::HashSet;

use itertools::Itertools;

use crate::utils::read_lines;

fn space((start, end): ((i32, i32, i32), (i32, i32, i32))) -> HashSet<(i32, i32, i32)> {
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

fn check_sum(bricks: &Vec<((i32, i32, i32), (i32, i32, i32))>) -> i32 {
    bricks
        .iter()
        .map(|((s1, s2, s3), (e1, e2, e3))| s1 + s2 + s3 + e1 + e2 + e3)
        .sum::<i32>()
}

pub fn answer() -> usize {
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

    (0..bricks.len())
        .filter(|disabled| {
            let disabled_iter = (0..bricks.len()).filter(|i| i != disabled);

            // check if some brick can be lovered given brick removal
            disabled_iter.clone().all(|i| {
                if bricks[i].0 .2 == 1 {
                    // stable on ground
                    return true;
                }

                let mut current_brick = bricks[i].clone();
                current_brick.0 .2 -= 1;
                current_brick.1 .2 -= 1;

                let current_brick_space = space(current_brick);

                for j in disabled_iter.clone() {
                    if j == i {
                        continue;
                    }

                    if current_brick_space.intersection(&space(bricks[j])).count() != 0 {
                        // collision exist, brick will still be stable
                        return true;
                    }
                }

                // no collision, brick can be moved down
                return false;
            })
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
