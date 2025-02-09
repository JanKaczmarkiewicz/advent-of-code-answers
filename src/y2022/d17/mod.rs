use std::collections::HashSet;

use ::array_init::array_init;

use crate::utils::read;

const CHAMBER_LENGTH: i32 = 7;
const INPUT_SIZE: usize = 10091;
const NR_OF_SHAPES: usize = 5;
const X_SPAWN_OFFSET: i32 = 2;
const Y_SPAWN_OFFSET: i32 = 3;

pub fn answer() {
    println!("Answer to day17: {} {}", a1(), a2());
}

// State:
//      shapes: [5] + operator index [10091] + stage [unbounded] : hashed as u128
// Problem how to hash a stage - say I compute external top layer of structure and subtract highest y
// it needs to be unique:
// ######*
// #*####*
// #####** -> ######* #*####* #####** ******* ##*##** -> 0000001 0100001 0100001 ... -> lets start with fixed -> u128 (or Vec[u128])
// *******
// ##*##**
/*

        |0000001|
        |0000001|
   20 > |0000001| < 14
   13 > |1111111| < 7
    6 > |1010101| < 0
*/

fn hash_space(space: &HashSet<(i32, i32)>, highest_y_tile: i32) -> u128 {
    const CUT_AT: i32 = 17;
    let lowest_y = (highest_y_tile - CUT_AT).max(0);
    let mut hash = 0;
    for (x, y) in space.iter().filter(|(_, y)| *y >= lowest_y) {
        let bit_position = CHAMBER_LENGTH - 1 + (CHAMBER_LENGTH) * (y - lowest_y) - x;
        hash |= 1 << bit_position;
    }
    return hash;
}

fn a1() -> usize {
    let o = read("src/y2022/d17/input");
    let mut operators = o.chars().enumerate().cycle().peekable();
    let mut state: [HashSet<u128>; INPUT_SIZE * NR_OF_SHAPES] = array_init(|_| HashSet::new());
    // if for the same shape, operator and stage
    let mut stable_shapes = HashSet::new();
    let mut highest_y_tile = 0;

    for i in 0_u64..1000000000000 {
        let mut x_pos = X_SPAWN_OFFSET;
        let mut y_pos = highest_y_tile + Y_SPAWN_OFFSET;

        // positions start from the top
        let shape_blocks: &[(i32, i32)] = match i % NR_OF_SHAPES as u64 {
            0 => &[(0, 0), (1, 0), (2, 0), (3, 0)],         // '-'
            1 => &[(1, 0), (1, 1), (1, 2), (0, 1), (2, 1)], // '+'
            2 => &[(2, 0), (2, 1), (2, 2), (1, 0), (0, 0)], // 'L'
            3 => &[(0, 0), (0, 1), (0, 2), (0, 3)],         // '|'
            4 => &[(0, 0), (0, 1), (1, 0), (1, 1)],         // 's'
            _ => panic!("NOT POSSIBLE"),
        };

        let stable_shapes_hash = hash_space(&stable_shapes, highest_y_tile);

        let is_present =
            !state[i as usize * operators.peek().unwrap().0].insert(stable_shapes_hash);

        if is_present {
            // instead of going throught all the iterations I can advance the i hub s
        }
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

                // TODO: sanitize hash_space
                // Why do I need to sanitize? Sanitizations strips inreachable destinations so
                // it is then ready to hash for uniques, and efficient in future computation
                // OPTIMALIZTION: filter all tiles that are not as a outside shape

                /*
                Is this actually nessesery? I dont think so (a bit sketchy but it should work)
                Imagine following situation

                |#      |
                |#      |
                |##     |
                |###    |
                |###    |
                ---------

                and

                |#      |
                |#      |
                |##     |
                |###    |
                |###    |
                |  #####|
                |#      |
                |#      |
                |##     |
                |###    |
                |###    |
                ---------

                In first situation there is pattern that repeats in the second one.
                Now I will pick some n which will indicate nr of lines considered eg 18 because 128 / 7 = 18.28

                The quickest way is to just implement it
                 */

                highest_y_tile = highest_y_tile.max(
                    shape_blocks
                        .iter()
                        .map(|(_, y)| y + y_pos)
                        .max()
                        .map_or(0, |x| x + 1),
                );

                // pick only those tiles that are

                break;
            } else {
                y_pos -= 1;
            }
        }
    }

    return highest_y_tile as usize;
}

fn a2() -> usize {
    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_solve_first_problem() {
        assert_eq!(a1(), 3106);
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

        // space.insert((0, 3));

        // space.insert((0, 2));
        // space.insert((1, 2));

        space.insert((0, 1));

        space.insert((CHAMBER_LENGTH - 1, 0));

        // assert_eq!(hash_space(&space, 3), 0b1000000_1100000_10000000_0000001);
        assert_eq!(hash_space(&space, 3), 0b0000000_0000000_1000000_0000001);
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

    // PART 2:
    // this algorythm does not work well on higher n
    // Why? because stable_shapes keeps track of all occupied tiles and
    // every time new shape lands its slows down operations dependent on stable_shapes
    //
    // Quick ideas:
    // Would take to much time!  - clean stable_shapes after a while bc some parts are not available for shapes to enter: new floor is formed
    // - the goal is to compute highest point This is just vague idea but is it possible to keep track of just a small window of board?
    // WORKING ON THIS - check for reccurence. At the end of operators it starts again. So if perfect floor is formed, with the same starting shape then it matter of multiplying remaining work;
    // Break at iterator end and display first 10 ish y tiles from the top
    #[test]
    fn should_solve_second_problem() {
        assert_eq!(a2(), 0);
    }

    // Supposed goal: If I detect some cyclic behavior I can reduce its complexity by caching the result.

    // Current flow is like this:

    // for n:
    //      spawn shape
    //      loop:
    //          move shape horizontaly
    //          if try move shape verticaly {} else { break }

    // additionaly keep track of history and based on the shape and moves detect ciclicity? No, there is external state
    // What does it mean to pattern to repeat? What is the pattern?
}
