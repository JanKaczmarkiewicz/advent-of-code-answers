use crate::utils::read_lines;
use priority_queue::PriorityQueue;
use std::{cmp::Reverse, vec};

fn get_dirs(last_dir: (i32, i32), consec: u8) -> Vec<(i32, i32)> {
    let mut directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    if last_dir == (0, 0) {
        return directions;
    }

    let opposite_dir = (last_dir.0 * -1, last_dir.1 * -1);
    directions.remove(directions.iter().position(|e| *e == opposite_dir).unwrap());

    if consec == 10 {
        directions.remove(directions.iter().position(|e| *e == last_dir).unwrap());
    }

    return directions;
}

// min 4 in one direction to be able to turn or stop
// max 10
// no back

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

    let is_inside_map = |(x, y)| (start.0..=end.0).contains(&x) && (start.1..=end.1).contains(&y);

    // lets do easy bf search

    let mut paths = PriorityQueue::new();
    paths.push(((0, 0), 0, start), Reverse(0));

    while let Some(((last_dir, consec, curr), cost)) = paths.pop() {
        for dir in get_dirs(last_dir, consec) {
            let (current_cord, next_consec, cost_for_step) = if last_dir == dir {
                let cord = (curr.0 + dir.0, curr.1 + dir.1);
                if !is_inside_map(cord) {
                    continue;
                }

                (cord, consec + 1, map[cord.0 as usize][cord.1 as usize])
            } else {
                const MIN_STEP: i32 = 4;

                let cord = (curr.0 + dir.0 * MIN_STEP, curr.1 + dir.1 * MIN_STEP);
                if !is_inside_map(cord) {
                    continue;
                }

                (
                    cord,
                    MIN_STEP as u8,
                    (1..=MIN_STEP).fold(0, |acc, i| {
                        acc + map[(curr.0 + dir.0 * i) as usize][(curr.1 + dir.1 * i) as usize]
                    }),
                )
            };

            let new_cost = cost.0 + cost_for_step;

            if current_cord == end {
                return new_cost;
            }

            paths.push_increase((dir, next_consec, current_cord), Reverse(new_cost));
        }
    }

    return u32::MAX;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_compute_solution() {
        assert_eq!(answer(), 94);
    }
}
