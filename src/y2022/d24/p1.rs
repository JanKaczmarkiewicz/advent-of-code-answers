use std::collections::HashSet;

use crate::utils::read_lines;

pub fn answer() -> i64 {
    // The map moves statically, independently to user actions
    // Say pointer is at (0,0) and goal it to move right down to opposite corner
    // There are 5 actions, awsd+pause the actions that lead to
    // each of those actions is potential branch in the search space for the shortest path.
    // Potential because if it leads to pointer cords to be the same as blizzard then this possiblility is invalid

    // Step 1 parse input

    let mut max_x = 0;
    let mut max_y = 0;
    let mut top_storms = HashSet::new();
    let mut down_storms = HashSet::new();
    let mut left_storms = HashSet::new();
    let mut right_storms = HashSet::new();

    for (y, row) in read_lines("src/y2022/d24/input").enumerate() {
        max_x = row.len();
        max_y = y;

        for (x, tile) in row.chars().enumerate() {
            match tile {
                '>' => {
                    right_storms.insert((x, y));
                }
                '<' => {
                    left_storms.insert((x, y));
                }
                'v' => {
                    down_storms.insert((x, y));
                }
                '^' => {
                    top_storms.insert((x, y));
                }
                _ => {}
            }
        }
    }

    // Step 2 initialize start location

    let start = (1, 0);
    let end = (max_x - 1, max_y);

    let right_direction = (1, 0);
    let left_direction = (-1, 0);
    let up_direction = (0, 1);
    let right_direction = (0, -1);

    // Step 3 simulate map, turns and generate graph (prioritize those branches that are making most progress)

    // Step 4 return shortest path length

    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_compute_solution() {
        assert_eq!(answer(), 0);
    }
}
