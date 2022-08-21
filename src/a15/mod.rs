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

fn lowest_risk(
    map: Vec<Vec<i32>>,
    destination: (i32, i32),
    get_cost: fn(&Vec<Vec<i32>>, (i32, i32)) -> Option<i32>,
) -> i32 {
    let mut visited = HashSet::new();
    let mut not_visited = BinaryHeap::new();

    not_visited.push(Node {
        pos: (0, 0),
        cost: 0,
    });

    while let Some(Node { pos: (x, y), cost }) = not_visited.pop() {
        if visited.contains(&(x, y)) {
            continue;
        };
        visited.insert((x, y));

        if (x, y) == destination {
            return cost;
        }

        for (x_mod, y_mod) in [(0, -1), (-1, 0), (1, 0), (0, 1)] {
            let next = (x + x_mod, y + y_mod);
            if visited.contains(&next) {
                continue;
            };

            if let Some(next_cost) = get_cost(&map, next) {
                let sum_cost = cost + next_cost;
                not_visited.push(Node {
                    pos: next,
                    cost: sum_cost,
                });
            }
        }
    }

    0
}

fn a() -> i32 {
    let map = get_map();
    let destination = (map.len() as i32 - 1, map.len() as i32 - 1);
    lowest_risk(map, destination, |map, (x, y)| {
        if x < map.len() as i32 && x >= 0 && y < map.len() as i32 && y >= 0 {
            Some(map[x as usize][y as usize])
        } else {
            None
        }
    })
}

fn b() -> i32 {
    let map = get_map();
    let destination = ((map.len() * 5) as i32 - 1, (map.len() * 5) as i32 - 1);
    lowest_risk(map, destination, |map, (x, y)| {
        let max = (5 * map.len()) as i32;

        if x.is_negative() {
            return None;
        }
        if y.is_negative() {
            return None;
        }
        if x >= max {
            return None;
        }
        if y >= max {
            return None;
        }

        let modificator =
            ((x as f32 / map.len() as f32).floor() + (y as f32 / map.len() as f32).floor()) as i32;

        let mut cost =
            ((modificator + map[x as usize % map.len()][y as usize % map.len()] - 1) % 9) + 1;

        if cost == 0 {
            cost += 1;
        }

        Some(cost)
    })
}

pub fn answer() {
    println!("Answer to problem 15: {} {}", a(), b());
}

#[cfg(test)]
mod tests {
    use super::{a, b};

    #[test]
    fn should_solve_first_problem() {
        assert_eq!(a(), 592);
    }

    #[test]
    fn should_solve_second_problem() {
        assert_eq!(b(), 2897);
    }
}
