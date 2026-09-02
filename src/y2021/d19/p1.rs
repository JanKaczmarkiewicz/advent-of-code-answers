use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::utils::read_lines;

type Vec3 = (i32, i32, i32);

fn apply_orientations(
    scanners: HashMap<usize, HashSet<Vec3>>,
) -> HashMap<usize, [HashSet<(i32, i32, i32)>; 24]> {
    const ALL_ROTATIONS: [(Vec3, Vec3, Vec3); 24] = [
        ((1, 0, 0), (0, 1, 0), (0, 0, 1)),
        ((1, 0, 0), (0, 0, -1), (0, 1, 0)),
        ((1, 0, 0), (0, -1, 0), (0, 0, -1)),
        ((1, 0, 0), (0, 0, 1), (0, -1, 0)),
        ((0, -1, 0), (1, 0, 0), (0, 0, 1)),
        ((0, 0, 1), (1, 0, 0), (0, 1, 0)),
        ((0, 1, 0), (1, 0, 0), (0, 0, -1)),
        ((0, 0, -1), (1, 0, 0), (0, -1, 0)),
        ((-1, 0, 0), (0, -1, 0), (0, 0, 1)),
        ((-1, 0, 0), (0, 0, -1), (0, -1, 0)),
        ((-1, 0, 0), (0, 1, 0), (0, 0, -1)),
        ((-1, 0, 0), (0, 0, 1), (0, 1, 0)),
        ((0, 1, 0), (-1, 0, 0), (0, 0, 1)),
        ((0, 0, 1), (-1, 0, 0), (0, -1, 0)),
        ((0, -1, 0), (-1, 0, 0), (0, 0, -1)),
        ((0, 0, -1), (-1, 0, 0), (0, 1, 0)),
        ((0, 0, -1), (0, 1, 0), (1, 0, 0)),
        ((0, 1, 0), (0, 0, 1), (1, 0, 0)),
        ((0, 0, 1), (0, -1, 0), (1, 0, 0)),
        ((0, -1, 0), (0, 0, -1), (1, 0, 0)),
        ((0, 0, -1), (0, -1, 0), (-1, 0, 0)),
        ((0, -1, 0), (0, 0, 1), (-1, 0, 0)),
        ((0, 0, 1), (0, 1, 0), (-1, 0, 0)),
        ((0, 1, 0), (0, 0, -1), (-1, 0, 0)),
    ];

    scanners
        .into_iter()
        .map(|(i, scanner)| {
            (
                i,
                ALL_ROTATIONS.map(|o| {
                    scanner
                        .iter()
                        .map(|c| {
                            (
                                c.0 * o.0 .0 + c.1 * o.0 .1 + c.2 * o.0 .2,
                                c.0 * o.1 .0 + c.1 * o.1 .1 + c.2 * o.1 .2,
                                c.0 * o.2 .0 + c.1 * o.2 .1 + c.2 * o.2 .2,
                            )
                        })
                        .collect::<HashSet<_>>()
                }),
            )
        })
        .collect::<HashMap<_, _>>()
}

fn parse_cord(s: String) -> Option<Vec3> {
    s.split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .next_tuple::<Vec3>()
}

fn get_scanners() -> HashMap<usize, HashSet<Vec3>> {
    let mut scanners = HashMap::new();

    let mut i: i32 = -1;
    for l in read_lines("src/y2021/d19/input") {
        if l == "" || l.starts_with(" ") {
            continue;
        } else if l.starts_with("---") {
            i += 1;
            scanners.insert(i as usize, HashSet::new());
        } else {
            scanners
                .get_mut(&(i as usize))
                .unwrap()
                .insert(parse_cord(l).unwrap());
        }
    }

    scanners
}

pub fn answer() -> usize {
    let mut scanners = get_scanners();

    const BASE: usize = 0;

    let mut alligned_scanners = HashMap::from([(BASE, scanners.remove(&BASE).unwrap())]);
    let mut unalligned_scanners = apply_orientations(scanners);

    let mut order = vec![BASE];

    while let Some(current_base) = order.pop() {
        let result = unalligned_scanners
            .iter()
            .filter_map(|(i, scanner_orientations)| {
                for scanner in scanner_orientations {
                    for base_cord in &alligned_scanners[&current_base] {
                        for cord in scanner {
                            let diff = (
                                base_cord.0 - cord.0,
                                base_cord.1 - cord.1,
                                base_cord.2 - cord.2,
                            );

                            let transformed = scanner
                                .iter()
                                .map(|cord| (diff.0 + cord.0, diff.1 + cord.1, diff.2 + cord.2))
                                .collect::<HashSet<_>>();

                            if transformed
                                .intersection(&alligned_scanners[&current_base])
                                .count()
                                >= 12
                            {
                                return Some((*i, transformed));
                            }
                        }
                    }
                }
                None
            })
            .collect::<Vec<_>>();

        for (i, alligned_scanner) in result {
            alligned_scanners.insert(i, alligned_scanner);
            unalligned_scanners.remove(&i);
            order.push(i);
        }
    }

    return alligned_scanners.into_values().flatten().unique().count();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_compute_solution() {
        assert_eq!(answer(), 82);
    }
}
