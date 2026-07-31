use crate::utils::read_lines;
use priority_queue::PriorityQueue;
use std::{cmp::Reverse, vec};

fn is_all_same1(arr: &[(i32, i32)]) -> bool {
    if arr.is_empty() {
        return true;
    }
    let first = arr[0];
    arr.iter().all(|&item| item == first)
}

fn get_dirs(history: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let mut directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    if history.first().unwrap() == &(0, 0) {
        return directions;
    }

    if is_all_same1(history) {
        directions.remove(directions.iter().position(|e| *e == history[0]).unwrap());
    }

    let opposite_dir = (
        history.last().unwrap().0 * -1,
        history.last().unwrap().1 * -1,
    );
    directions.remove(directions.iter().position(|e| *e == opposite_dir).unwrap());

    return directions;
}

pub fn answer() -> u32 {
    let map = read_lines("src/y2023/d17/input")
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let start = (0, 0);
    let end = ((map.len() - 1) as i32, (map[0].len() - 1) as i32);

    // lets do easy bf search

    let mut paths = PriorityQueue::new();
    paths.push(([(0, 0), (0, 0), (0, 0)], start), Reverse(0));

    while let Some(((last_steps, curr), cost)) = paths.pop() {
        for dir in get_dirs(&last_steps) {
            let mut steps_copy = last_steps.clone();
            steps_copy[0] = last_steps[1];
            steps_copy[1] = last_steps[2];
            steps_copy[2] = dir;
            let current_cord = (curr.0 + dir.0, curr.1 + dir.1);

            // bounds
            if !(start.0..=end.0).contains(&current_cord.0)
                || !(start.1..=end.1).contains(&current_cord.1)
            {
                continue;
            }

            let new_cost = cost.0 + map[current_cord.0 as usize][current_cord.1 as usize];

            if current_cord == end {
                return new_cost;
            }

            paths.push_increase((steps_copy, current_cord), Reverse(new_cost));
        }
    }

    return best_cost;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_compute_solution() {
        assert_eq!(answer(), 102);
    }
}
