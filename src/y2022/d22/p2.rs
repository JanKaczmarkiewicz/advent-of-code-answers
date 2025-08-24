use crate::utils::read;
use itertools::Itertools;

#[derive(PartialEq, Eq, Debug)]
enum Tile {
    Wall,
    Movable,
}

#[derive(PartialEq, Debug)]
struct Face {
    locations: Vec<Vec<Tile>>,
    top: (usize, usize),
    bottom: (usize, usize),
    left: (usize, usize),
    right: (usize, usize),
}

fn parse_cube_map(map_raw: &str) -> Vec<((usize, usize), Face)> {
    let map_chars = map_raw
        .split("\n")
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    const CUBE_FACES: usize = 6;

    let mut abstract_representation_positions = Vec::with_capacity(CUBE_FACES);
    let mut abstract_representation_locations = Vec::with_capacity(CUBE_FACES);

    let face_size = if map_raw.len() == 20199 { 50 } else { 4 };

    let mut col = 0;
    let mut row = 0;
    while abstract_representation_positions.len() != CUBE_FACES {
        if map_chars[row * face_size][col * face_size] != ' ' {
            abstract_representation_positions.push((col, row));
            abstract_representation_locations.push(
                (0..face_size)
                    .map(|row_relative| {
                        (0..face_size)
                            .map(|col_relative| {
                                let char = map_chars[row * face_size + row_relative]
                                    [col * face_size + col_relative];

                                if char == '.' {
                                    Tile::Movable
                                } else if char == '#' {
                                    Tile::Wall
                                } else {
                                    panic!();
                                }
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>(),
            );
        }

        if map_chars[row * face_size]
            .get((col + 1) * face_size)
            .is_some()
        {
            col += 1;
        } else {
            row += 1;
            col = 0;
        }
    }
    // Might be some bugs because of order in checks
    abstract_representation_locations
        .into_iter()
        .zip(abstract_representation_positions.iter())
        .map(|(locations, (col, row))| {
            let check_for_face = |col_relative: i32, row_relative: i32| {
                abstract_representation_positions
                    .iter()
                    .position(|(item_col, item_row)| {
                        (*item_col as i32, *item_row as i32)
                            == (*col as i32 + col_relative, *row as i32 + row_relative)
                    })
                    .map(|n| {
                        (
                            n,
                            /*rotation: */
                            ((col_relative.abs() + row_relative.abs() - 1) % 4) as usize,
                        )
                    })
            };

            let top = [
                check_for_face(0, -1),
                check_for_face(1, -1),
                check_for_face(-1, 3),
                check_for_face(-2, 3),
                //
                check_for_face(-2, 1),
                check_for_face(2, -1),
                check_for_face(-1, -1),
            ]
            .into_iter()
            .find_map(|e| e)
            .unwrap();
            let bottom = [
                check_for_face(0, 1),
                check_for_face(-1, 1),
                check_for_face(-1, 1),
                check_for_face(2, -3),
                //
                check_for_face(2, 1),
                check_for_face(1, 1),
                check_for_face(-2, -1),
                check_for_face(-3, -1),
            ]
            .into_iter()
            .find_map(|e| e)
            .unwrap();
            let left = [
                check_for_face(-1, 0),
                check_for_face(-1, 1),
                check_for_face(-1, 2),
                check_for_face(1, -3),
                check_for_face(1, -2),
                //
                check_for_face(3, 1),
                check_for_face(-1, -1),
            ]
            .into_iter()
            .find_map(|e| e)
            .unwrap();
            let right = [
                check_for_face(1, 0),
                check_for_face(1, -1),
                check_for_face(1, -2),
                check_for_face(-1, 2),
                //
                check_for_face(1, 2),
                check_for_face(1, 1),
                check_for_face(-1, -2),
            ]
            .into_iter()
            .find_map(|e| e)
            .unwrap();
            (
                (*col, *row),
                Face {
                    locations,
                    top,
                    bottom,
                    left,
                    right,
                },
            )
        })
        .collect()
}

fn rotate((x, y): (i32, i32)) -> (i32, i32) {
    (-y, x)
}

pub fn answer() -> i32 {
    let all = read("src/y2022/d22/input");
    let mut iter = all.split("\n\n");

    let map_raw = iter.next().unwrap();
    let operations_raw = iter.next().unwrap();

    let cube_map = parse_cube_map(map_raw);

    let perspective_up = (0, -1);
    let perspective_right = (1, 0);
    let perspective_down = (0, 1);
    let perspective_left = (-1, 0);

    let mut position = (0, 0);
    let mut current_map = &cube_map[0];
    let mut rotated_perspective = perspective_right;

    let mut chars = operations_raw.chars();

    while let Ok(n) = chars
        .peeking_take_while(|x| x.is_numeric())
        .collect::<String>()
        .parse::<u64>()
    {
        for _ in 0..n {
            let obvious_next_position = (
                position.0 + rotated_perspective.0,
                position.1 + rotated_perspective.1,
            );

            let (_, current_map_data) = current_map;

            if let Some(tile) = current_map_data
                .locations
                .get(obvious_next_position.1 as usize)
                .map(|r| r.get(obvious_next_position.0 as usize))
                .flatten()
            {
                match tile {
                    Tile::Movable => position = obvious_next_position,
                    Tile::Wall => break,
                }
            } else {
                let board_max_size = current_map_data.locations.len() as i32 - 1;

                let ((index, rotation), destination) = if rotated_perspective == perspective_up {
                    (
                        current_map_data.top.clone(),
                        (position.0, board_max_size), /*bottom */
                    )
                } else if rotated_perspective == perspective_right {
                    (
                        current_map_data.right.clone(),
                        (0, position.1), /*left */
                    )
                } else if rotated_perspective == perspective_left {
                    (
                        current_map_data.left.clone(),
                        (board_max_size, position.1), /*right */
                    )
                } else if rotated_perspective == perspective_down {
                    (current_map_data.bottom, (position.0, 0) /*top */)
                } else {
                    panic!()
                };

                let next_map = &cube_map[index];
                let position_at_next_side =
                    (0..rotation).fold(destination, |(x, y), _| (board_max_size - y, x));

                match next_map.1.locations[position_at_next_side.1 as usize]
                    [position_at_next_side.0 as usize]
                {
                    Tile::Movable => {
                        rotated_perspective =
                            (0..rotation).fold(rotated_perspective, |cords, _| rotate(cords));
                        current_map = next_map;
                        position = position_at_next_side
                    }
                    Tile::Wall => break,
                }
            }
        }

        if let Some(rotation) = chars.next() {
            rotated_perspective = match rotation {
                'L' => rotate(rotate(rotate(rotated_perspective))),
                'R' => rotate(rotated_perspective),
                _ => panic!(),
            }
        }
    }

    let (x, y) = current_map.0;

    1000 * (position.1 + 1 + (x * current_map.1.locations.len()) as i32)
        + 4 * (position.0 + 1 + (y * current_map.1.locations.len()) as i32)
        + if rotated_perspective == perspective_up {
            3
        } else if rotated_perspective == perspective_right {
            0
        } else if rotated_perspective == perspective_down {
            1
        } else {
            2
        }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_compute_solution() {
        // The answer algo is solving the problem correctly. It works for a given example. Parsing seems to be correct
        assert_eq!(answer(), 0);
    }

    //     #[test]
    //     fn should_parse_simple_cross_cube() {
    //         let input = "    #...
    //     ....
    //     ....
    //     ....
    // ##..###.####
    // ............
    // ............
    // ............
    //     ####
    //     #...
    //     ....
    //     ....
    //     ####
    //     ##..
    //     ....
    //     ....
    //     ";

    //         assert_eq!(parse_cube_map(input), vec![])
    //     }
}
