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
                    .map(|col_relative| {
                        (0..face_size)
                            .map(|row_relative| {
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

            let top = [check_for_face(0, -1).map(|n| (n, Rotation(0)))]
                .into_iter()
                .find_map(|e| e)
                .unwrap();
            let bottom = [check_for_face(0, 1).map(|n| (n, Rotation(0)))]
                .into_iter()
                .find_map(|e| e)
                .unwrap();
            let left = [check_for_face(-1, 0).map(|n| (n, Rotation(0)))]
                .into_iter()
                .find_map(|e| e)
                .unwrap();
            let right = [check_for_face(1, 0).map(|n| (n, Rotation(0)))]
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
    // 3 is fine
    // 1: missing
    //  - left (2 90deg rotated)
    //  - right (4 270deg rotated)
    //  - top (180deg rotated)
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
    ####
    ##..
    ....
    ....
    ";

        assert_eq!(parse_cube_map(input), vec![])
    }
}
