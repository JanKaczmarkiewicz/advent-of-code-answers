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
            if space(bricks[i]).difference(&space(bricks[j])).count() != 0 {
                bricks[i].0 .2 += 1;
                bricks[i].1 .2 += 1;
                break;
            }
        }
    }

    (0..bricks.len())
        .filter(|disabled| {
            // disable loop
            let disabled_iter = (0..bricks.len()).filter(|i| i != disabled);

            // check if some brick can be lovered given brick removal
            let r = disabled_iter.clone().all(|i| {
                if bricks[i].0 .2 == 1 {
                    return true;
                }

                bricks[i].0 .2 -= 1;
                bricks[i].1 .2 -= 1;

                for j in disabled_iter.clone() {
                    if j == i {
                        continue;
                    }

                    if space(bricks[i]).difference(&space(bricks[j])).count() != 0 {
                        // collision exist
                        bricks[i].0 .2 += 1;
                        bricks[i].1 .2 += 1;

                        return true;
                    }
                }

                // no collision, brick can be moved down
                bricks[i].0 .2 += 1;
                bricks[i].1 .2 += 1;

                return false;
            });
            println!("{disabled} {r}");
            r
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
