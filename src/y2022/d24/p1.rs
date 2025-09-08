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
    let mut up_storms = vec![];
    let mut down_storms = vec![];
    let mut left_storms = vec![];
    let mut right_storms = vec![];

    for (y, row) in read_lines("src/y2022/d24/input").enumerate() {
        max_x = row.len() as i32 - 1;
        max_y = y as i32;

        for (x, tile) in row.chars().enumerate() {
            let cords = (x as i32, y as i32);

            match tile {
                '>' => {
                    right_storms.push(cords);
                }
                '<' => {
                    left_storms.push(cords);
                }
                'v' => {
                    down_storms.push(cords);
                }
                '^' => {
                    up_storms.push(cords);
                }
                _ => {}
            }
        }
    }

    // Step 2 initialize start location

    let start = (1, 0);
    let end = (max_x as i32 - 1, max_y as i32);

    let right_direction = (1, 0);
    let left_direction = (-1, 0);
    let up_direction = (0, -1);
    let down_direction = (0, 1);
    let wait_direction = (0, 0);

    let actions = [
        (wait_direction),
        (right_direction),
        (down_direction),
        (up_direction),
        (left_direction),
    ];

    // Step 3 simulate map, turns and generate graph (prioritize those branches that are making most progress)
    let mut i = 0;
    let mut valid_paths = HashSet::with_capacity(2_usize.pow(10));
    valid_paths.insert(start);

    loop {
        for pos in up_storms.iter_mut() {
            let new_pos = (pos.0 + up_direction.0, pos.1 + up_direction.1);
            *pos = if new_pos.1 == 0 {
                (new_pos.0, max_y - 1)
            } else {
                new_pos
            }
        }

        for pos in down_storms.iter_mut() {
            let new_pos = (pos.0 + down_direction.0, pos.1 + down_direction.1);
            *pos = if new_pos.1 == max_y {
                (new_pos.0, 1)
            } else {
                new_pos
            }
        }

        for pos in left_storms.iter_mut() {
            let new_pos = (pos.0 + left_direction.0, pos.1 + left_direction.1);
            *pos = if new_pos.0 == 0 {
                (max_x - 1, new_pos.1)
            } else {
                new_pos
            }
        }

        for pos in right_storms.iter_mut() {
            let new_pos = (pos.0 + right_direction.0, pos.1 + right_direction.1);
            *pos = if new_pos.0 == max_x {
                (1, new_pos.1)
            } else {
                new_pos
            }
        }

        valid_paths = valid_paths
            .iter()
            .map(|(curr_x, curr_y)| actions.iter().map(move |(x, y)| ((x + curr_x, y + curr_y))))
            .flatten()
            .filter(|cords| {
                !(up_storms.contains(cords)
                    || down_storms.contains(cords)
                    || right_storms.contains(cords)
                    || left_storms.contains(cords))
            })
            .filter(|pos| {
                (end == *pos || start == *pos)
                    || !(pos.1 <= 0 || pos.0 <= 0 || pos.1 >= max_y || pos.0 >= max_x)
            })
            .collect();

        println!("{}", valid_paths.len());

        if let Some((_path, _)) = valid_paths.get(&end) {
            // println!("{path}");
            return i + 1;
        }

        i += 1;
    }

    // Step 4 return shortest path length
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_compute_solution() {
        assert_eq!(answer(), 221);
    }
}
