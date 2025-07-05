use core::panic;

use crate::utils::read;

#[derive(PartialEq, Eq, Debug)]
enum Tile {
    Wall,
    Movable,
}

#[derive(PartialEq, Debug)]
struct Rotation(u8);

impl Rotation {
    // TODO take
}

#[derive(PartialEq, Debug)]
struct Face {
    locations: Vec<Vec<Tile>>,
    top: (usize, Rotation),
    bottom: (usize, Rotation),
    left: (usize, Rotation),
    right: (usize, Rotation),
}

fn parse_cube_map(map_raw: &str) -> Vec<Face> {
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
            };

            let top = [
                check_for_face(0, -1).map(|n| (n, Rotation(0))),
                check_for_face(1, -1).map(|n| (n, Rotation(3))),
                check_for_face(-1, -1).map(|n| (n, Rotation(1))),
                check_for_face(0, 3).map(|n| (n, Rotation(2))),
            ]
            .into_iter()
            .find_map(|e| e)
            .unwrap();
            let bottom = [
                check_for_face(0, 1).map(|n| (n, Rotation(0))),
                check_for_face(1, 1).map(|n| (n, Rotation(1))),
                check_for_face(-1, 1).map(|n| (n, Rotation(3))),
                check_for_face(0, -3).map(|n| (n, Rotation(0))),
            ]
            .into_iter()
            .find_map(|e| e)
            .unwrap();
            let left = [
                check_for_face(-1, 0).map(|n| (n, Rotation(0))),
                check_for_face(-1, 1).map(|n| (n, Rotation(1))),
                check_for_face(-1, -1).map(|n| (n, Rotation(3))),
                check_for_face(1, 2).map(|n| (n, Rotation(2))),
                //(-1, -2)
                check_for_face(1, -2).map(|n| (n, Rotation(2))),
            ]
            .into_iter()
            .find_map(|e| e)
            .unwrap();
            let right = [
                check_for_face(1, 0).map(|n| (n, Rotation(0))),
                check_for_face(1, 1).map(|n| (n, Rotation(3))),
                check_for_face(1, -1).map(|n| (n, Rotation(1))),
                check_for_face(-1, 2).map(|n| (n, Rotation(2))),
                check_for_face(1, -2).map(|n| (n, Rotation(2))),
            ]
            .into_iter()
            .find_map(|e| e)
            .unwrap();

            Face {
                locations,
                top,
                bottom,
                left,
                right,
            }
        })
        .collect()
}

pub fn answer() -> i64 {
    let all = read("src/y2022/d22/input");
    let mut iter = all.split("\n\n");

    let cube_map = parse_cube_map(iter.next().unwrap());
    let operation_raw = iter.next().unwrap();

    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_compute_solution() {
        assert_eq!(answer(), 0);
    }
    // 1: missing
    //  - left (2 90deg) done
    //  - right (4 270deg) done
    //  - top (180deg) done
    // 2: missing
    //  - top (1 270deg) done
    //  - left (6 180deg) done
    //  - bottom (5 90deg) done
    // 3 is fine
    // 4: missing
    //  - top (1 90deg)
    //  - bottom (5 270deg) ()
    //  - right (6 180deg)
    // 5: missing
    //  - left (2 270deg)
    //  - right (4 90deg)
    // 6: missing
    //  - left (2 180deg) (-1, -2)
    //  - right (3 180deg) (1, -2)
    //  - bottom (1 0deg) (0, -3)

    // note: order of check does matter: there can be immidiete bottom and bottom-right - both are valid bottom (bottom-right will be bottom if immidiete bottom is not present)

    #[test]
    fn should_parse_simple_cross_cube() {
        let input = "    #...
    ....
    ....
    ....
##..###.####
............
............
............
    ####
    #...
    ....
    ....
    ####
    ##..
    ....
    ....
    ";

        assert_eq!(
            parse_cube_map(input),
            vec![
                Face {
                    locations: vec![
                        vec![Tile::Wall, Tile::Movable, Tile::Movable, Tile::Movable],
                        vec![Tile::Movable, Tile::Movable, Tile::Movable, Tile::Movable],
                        vec![Tile::Movable, Tile::Movable, Tile::Movable, Tile::Movable],
                        vec![Tile::Movable, Tile::Movable, Tile::Movable, Tile::Movable]
                    ],
                    top: (5, Rotation(2)),
                    bottom: (2, Rotation(0)),
                    left: (1, Rotation(1)),
                    right: (3, Rotation(3))
                },
                Face {
                    locations: vec![
                        vec![Tile::Wall, Tile::Wall, Tile::Movable, Tile::Movable],
                        vec![Tile::Movable, Tile::Movable, Tile::Movable, Tile::Movable],
                        vec![Tile::Movable, Tile::Movable, Tile::Movable, Tile::Movable],
                        vec![Tile::Movable, Tile::Movable, Tile::Movable, Tile::Movable]
                    ],
                    top: (0, Rotation(3)),
                    bottom: (4, Rotation(1)),
                    left: (5, Rotation(2)),
                    right: (2, Rotation(0))
                },
                Face {
                    locations: vec![
                        vec![Tile::Wall, Tile::Wall, Tile::Wall, Tile::Movable],
                        vec![Tile::Movable, Tile::Movable, Tile::Movable, Tile::Movable],
                        vec![Tile::Movable, Tile::Movable, Tile::Movable, Tile::Movable],
                        vec![Tile::Movable, Tile::Movable, Tile::Movable, Tile::Movable]
                    ],
                    top: (0, Rotation(0)),
                    bottom: (4, Rotation(0)),
                    left: (1, Rotation(0)),
                    right: (3, Rotation(0))
                },
                Face {
                    locations: vec![
                        vec![Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall],
                        vec![Tile::Movable, Tile::Movable, Tile::Movable, Tile::Movable],
                        vec![Tile::Movable, Tile::Movable, Tile::Movable, Tile::Movable],
                        vec![Tile::Movable, Tile::Movable, Tile::Movable, Tile::Movable]
                    ],
                    top: (0, Rotation(1)),
                    bottom: (4, Rotation(3)),
                    left: (2, Rotation(0)),
                    right: (5, Rotation(2))
                },
                Face {
                    locations: vec![
                        vec![Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall],
                        vec![Tile::Wall, Tile::Movable, Tile::Movable, Tile::Movable],
                        vec![Tile::Movable, Tile::Movable, Tile::Movable, Tile::Movable],
                        vec![Tile::Movable, Tile::Movable, Tile::Movable, Tile::Movable]
                    ],
                    top: (2, Rotation(0)),
                    bottom: (5, Rotation(0)),
                    left: (1, Rotation(3)),
                    right: (3, Rotation(1))
                },
                Face {
                    locations: vec![
                        vec![Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall],
                        vec![Tile::Wall, Tile::Wall, Tile::Movable, Tile::Movable],
                        vec![Tile::Movable, Tile::Movable, Tile::Movable, Tile::Movable],
                        vec![Tile::Movable, Tile::Movable, Tile::Movable, Tile::Movable]
                    ],
                    top: (4, Rotation(0)),
                    bottom: (0, Rotation(0)),
                    left: (3, Rotation(2)),
                    right: (3, Rotation(2))
                }
            ]
        )
    }
}
