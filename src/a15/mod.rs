use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

use crate::utils::read_lines;

fn get_map() -> Vec<Vec<i32>> {
    read_lines("src/a15/input")
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn get_tile_cost(map: &Vec<Vec<i32>>, (x, y): (i32, i32)) -> Option<i32> {
    if x < map.len() as i32 && x >= 0 && y < map.len() as i32 && y >= 0 {
        Some(map[x as usize][y as usize])
    } else {
        None
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    cost: i32,
    pos: (i32, i32),
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

// TODO: optimize - it takes 2minutes
fn lowest_risk() -> i32 {
    let map = get_map();

    let destination = (map.len() as i32 - 1, map.len() as i32 - 1);

    let mut visited = HashSet::new();

    let mut not_visited = BinaryHeap::new();

    not_visited.push(Node {
        pos: (0, 0),
        cost: 0,
    });

    while let Some(Node { pos: (x, y), cost }) = not_visited.pop() {
        let x = x;
        let y = y;
        let cost = cost;

        visited.insert((x, y));
        if (x, y) == destination {
            return cost;
        }

        for (x_mod, y_mod) in [(0, -1), (-1, 0), (1, 0), (0, 1)] {
            let next = (x + x_mod, y + y_mod);

            if visited.contains(&next) {
                continue;
            };

            if let Some(next_cost) = get_tile_cost(&map, next) {
                let sum_cost = cost + next_cost;
                not_visited.push(Node {
                    pos: next,
                    cost: sum_cost,
                });
            }
        }
    }

    return 0;
}

pub fn answer() {
    println!("{}", lowest_risk());
}

#[cfg(test)]
mod tests {
    use super::lowest_risk;

    #[test]
    fn should_solve_first_problem() {
        assert_eq!(lowest_risk(), 592);
    }
}
