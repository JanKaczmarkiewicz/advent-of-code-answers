use std::collections::{HashMap, HashSet};

use crate::utils::read;

const CHAMBER_LENGTH: i128 = 7;

const NR_OF_SHAPES: i128 = 5;
const X_SPAWN_OFFSET: i128 = 2;
const Y_SPAWN_OFFSET: i128 = 3;
const EPOCHS_P1: i128 = 2022;
const EPOCHS_P2: i128 = 1000000000000;

pub fn answer() {
    println!("Answer to day17: {} {}", solve(EPOCHS_P1), solve(EPOCHS_P2));
}

fn hash_space(space: &HashSet<(i128, i128)>, highest_y_tile: i128) -> u128 {
    const CUT_AT: i128 = 17;
    let lowest_y = (highest_y_tile as i128 - CUT_AT as i128).max(0) as i128;
    let mut hash: u128 = 0;
    for (x, y) in space.iter().filter(|(_, y)| *y >= lowest_y) {
        let bit_position = CHAMBER_LENGTH - 1 + (CHAMBER_LENGTH) * (y - lowest_y) - x;
        hash |= 1 << bit_position;
    }
    return hash;
}

fn solve(epochs: i128) -> usize {
    let o = read("src/y2022/d17/input");
    let input_size = o.len();
    let mut operators = o.chars().enumerate().cycle().peekable();

    let mut hashes =
        Vec::<HashMap<u128, (i128, i128)>>::with_capacity(input_size * NR_OF_SHAPES as usize);
    for _ in 0..input_size * NR_OF_SHAPES as usize {
        hashes.push(HashMap::new());
    }

    // if for the same shape, operator and stage
    let mut stable_shapes = HashSet::new();
    let mut highest_y_tile = 0;

    let mut is_ciclicity_detected = false;
    let mut i = 0;
    while i < epochs {
        let mut x_pos: i128 = X_SPAWN_OFFSET;
        let mut y_pos: i128 = highest_y_tile + Y_SPAWN_OFFSET;
        let shape_nr = (i % NR_OF_SHAPES) as usize;

        // positions start from the top
        let shape_blocks: &[(i128, i128)] = match shape_nr {
            0 => &[(0, 0), (1, 0), (2, 0), (3, 0)],         // '-'
            1 => &[(1, 0), (1, 1), (1, 2), (0, 1), (2, 1)], // '+'
            2 => &[(2, 0), (2, 1), (2, 2), (1, 0), (0, 0)], // 'L'
            3 => &[(0, 0), (0, 1), (0, 2), (0, 3)],         // '|'
            4 => &[(0, 0), (0, 1), (1, 0), (1, 1)],         // 's'
            _ => panic!("NOT POSSIBLE"),
        };

        loop {
            // operator action
            match operators.next().unwrap().1 {
                '<' => {
                    let is_collision_after_move = shape_blocks
                        .iter()
                        .map(|(x, y)| (x + x_pos, y + y_pos))
                        .any(|(x, y)| x - 1 < 0 || stable_shapes.contains(&(x - 1, y)));

                    if !is_collision_after_move {
                        x_pos -= 1;
                    }
                }

                '>' => {
                    let is_collision_after_move = shape_blocks
                        .iter()
                        .map(|(x, y)| (x + x_pos, y + y_pos))
                        .any(|(x, y)| {
                            x + 1 >= CHAMBER_LENGTH || stable_shapes.contains(&(x + 1, y))
                        });

                    if !is_collision_after_move {
                        x_pos += 1;
                    }
                }

                _ => panic!("Not implemented!"),
            }

            // dawn fall action
            let is_collision_after_move = shape_blocks
                .iter()
                .map(|(x, y)| (x + x_pos, y + y_pos))
                .any(|(x, y)| y - 1 < 0 || stable_shapes.contains(&(x, y - 1)));

            if is_collision_after_move {
                stable_shapes.extend(shape_blocks.iter().map(|(x, y)| (x + x_pos, y + y_pos)));

                highest_y_tile = highest_y_tile.max(
                    shape_blocks
                        .iter()
                        .map(|(_, y)| y + y_pos)
                        .max()
                        .map_or(0, |x| x + 1),
                );
                break;
            } else {
                y_pos -= 1;
            }
        }

        if !is_ciclicity_detected {
            let stable_shapes_hash = hash_space(&stable_shapes, highest_y_tile);

            let operator_index = operators.peek().unwrap().0;

            if let Some((prev_i, prev_highest_y_tile)) = hashes
                [shape_nr * input_size + operator_index]
                .insert(stable_shapes_hash, (i, highest_y_tile))
            {
                let i_difference = i - prev_i; // how many shapes had been placed since last state like this
                let height_difference = highest_y_tile - prev_highest_y_tile;
                let nr_of_possible_skips =
                    ((epochs - i) as f64 / i_difference as f64).floor() as i128;

                // update height of points
                let mut new_stable_shapes = HashSet::with_capacity(stable_shapes.capacity());
                for (x, y) in stable_shapes {
                    new_stable_shapes.insert((x, y + height_difference * nr_of_possible_skips));
                }
                stable_shapes = new_stable_shapes;

                // update interation point
                i += i_difference * nr_of_possible_skips;
                highest_y_tile += height_difference * nr_of_possible_skips;

                is_ciclicity_detected = true;
            }
        }

        i += 1;
    }

    return highest_y_tile as usize;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_solve_first_problem() {
        assert_eq!(solve(EPOCHS_P1), 3106);
    }

    #[test]
    fn hash_space_should_return_a_number_that_binary_representation_have_1_where_space_has_solid_object(
    ) {
        let mut space = HashSet::new();

        /*
        |1000000|
        |1100000| -> 1000000 1100000 1000000 0000001
        |1000000|
        |0000001|
         */

        space.insert((0, 3));

        space.insert((0, 2));
        space.insert((1, 2));

        space.insert((0, 1));

        space.insert((CHAMBER_LENGTH - 1, 0));

        assert_eq!(hash_space(&space, 3), 0b1000000_1100000_10000000_0000001);
    }

    #[test]
    fn hash_space_should_return_striped_lower_part() {
        let mut space = HashSet::new();

        /*
             |1000000|
             |1100000| -> 1000000 1100000 1000000 0000001
             |1000000|
             |0000001|
             |0000001|16
             |0000001|
             |0000001|
             |0000001|
             |0000001|
             |0000001|10
             |0000001|
             |0000001|
             |0000001|
             |0000001|
             |0000001|
             |0000001|
             |0000001|
        20 > |0000001| < 14
        13 > |1111111| < 7
         6 > |1010101| < 0
              */

        space.insert((0, 20));

        space.insert((0, 19));
        space.insert((1, 19));

        space.insert((0, 18));

        space.insert((6, 17));
        space.insert((6, 16));
        space.insert((6, 15));
        space.insert((6, 14));
        space.insert((6, 13));
        space.insert((6, 12));
        space.insert((6, 11));
        space.insert((6, 10));
        space.insert((6, 9));
        space.insert((6, 8));
        space.insert((6, 7));
        space.insert((6, 6));
        space.insert((6, 5));
        space.insert((6, 4));
        space.insert((6, 3));
        space.insert((6, 2));

        space.insert((0, 1));
        space.insert((1, 1));
        space.insert((2, 1));
        space.insert((3, 1));
        space.insert((4, 1));
        space.insert((5, 1));
        space.insert((6, 1));

        space.insert((0, 0));
        space.insert((2, 0));
        space.insert((4, 0));
        space.insert((6, 0));

        let highest_y = space.iter().map(|(_, y)| y).max().unwrap();

        assert_eq!(
            hash_space(&space, *highest_y),
            0b1000000_1100000_1000000_0000001_0000001_0000001_0000001_0000001_0000001_0000001_0000001_0000001_0000001_0000001_0000001_0000001_0000001_0000001
        );
    }

    #[test]
    fn should_solve_second_problem() {
        assert_eq!(solve(EPOCHS_P2), 1537175792495);
    }
}
