use crate::utils::read;

// Tetris
// - -> + -> _| -> | -> square -> repeat
// input: list of <,> that pushes shape left or right, repeats
// chamber: 7 units wide
const CHAMBER_LENGTH: i32 = 7;
const Y_SPAWN_OFFSET: i32 = 3;
const X_SPAWN_OFFSET: i32 = 2;
// shape spawns on the left two units from a wall and 3 units above rock/ground
// |11**   |
// |  **   |
// |       1
// |       1
// |       1
// | *     |
// |***    |
// 00*     |
// ---------

// Procedure:
// rock is being pushed to the right or left by gas, bounce check: wall or other rock (prevents movement)
// rock is moved down, if it is not possible then a new rock is spawned

// calculate_shell(tiles: HashSet<Cords>)
// calculate_shell(tiles: HashSet<Cords>)

// 2022 rocks are spawned

// Current plan:
// every time shape is stabalized recalculate surface of tower.
// Data structure: hash set that will contain cords of tiles that are on the boudary
// Update: after placing shape some of cords will be no longer needed. Cords will have to be recalculated (from right to left)
// Update algorithm:

// Visualization:

// Find tiles that are visible from the {top-always, left-when count(tiles with x cord)==0, right-when count(tiles with x cord)==6}
//   |
// ||*|  | |
// |*** |* |
// | * |* ||
// |  ** **|
// ||*  *  |
// ---------

// |       |
// |     * |
// | *  *  |
// |  ** *s|
// | *  *  |
// ---------

pub fn answer() {
    println!("Answer to day17: {} {}", a1(), a2());
}

fn a1() -> usize {
    let o = read("src/y2022/d17/input");
    let mut operators = o.chars().cycle();

    let mut stable_shapes: Vec<(i32, i32)> = vec![];
    let mut highest_y_tile = 0;

    for i in 0_u64..1000000000000 {
        let mut x_pos = X_SPAWN_OFFSET;
        let mut y_pos = highest_y_tile + Y_SPAWN_OFFSET;

        // positions start from the top
        let shape_blocks: &[(i32, i32)] = match i % 5 {
            0 => &[(0, 0), (1, 0), (2, 0), (3, 0)],         // '-'
            1 => &[(1, 0), (1, 1), (1, 2), (0, 1), (2, 1)], // '+'
            2 => &[(2, 0), (2, 1), (2, 2), (1, 0), (0, 0)], // 'L'
            3 => &[(0, 0), (0, 1), (0, 2), (0, 3)],         // '|'
            4 => &[(0, 0), (0, 1), (1, 0), (1, 1)],         // 's'
            _ => panic!("NOT POSSIBLE"),
        };

        loop {
            // operator action
            match operators.next().unwrap() {
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
                stable_shapes.extend(shape_blocks.iter().map(|(x, y)| (x + x_pos, y + y_pos))); // OPTIMALIZTION: filter all tiles that are not as a outside shape
                highest_y_tile = highest_y_tile.max(
                    shape_blocks
                        .iter()
                        .map(|(x, y)| (x + x_pos, y + y_pos))
                        .map(|(_, y)| y)
                        .max()
                        .map_or(0, |x| x + 1),
                );

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

    // PART 2:
    // this algorythm does not work well on higher n
    // Why? because stable_shapes keeps track of all occupied tiles and
    // every time new shape lands its slows down operations dependent on stable_shapes
    //
    // Quick ideas:
    // Would take to much time!  - clean stable_shapes after a while bc some parts are not available for shapes to enter: new floor is formed
    // - the goal is to compute highest point This is just vague idea but is it possible to keep track of just a small window of board?
    // - check for reccurence. At the end of operators it starts again. So if perfect floor is formed, with the same starting shape then it matter of multiplying remaining work;
    #[test]
    fn should_solve_second_problem() {
        assert_eq!(a2(), 0);
    }
}
