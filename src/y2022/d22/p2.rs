use core::panic;

use crate::utils::read;

#[derive(PartialEq, Eq)]
enum Tile {
    Wall,
    Movable,
}

pub fn answer() -> i64 {
    let all = read("src/y2022/d22/input");
    let mut iter = all.split("\n\n");
    let map_raw = iter
        .next()
        .unwrap()
        .split("\n")
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let operation_raw = iter.next().unwrap();

    let face_size = if all.len() == 25823 { 50 } else { 4 };

    let mut abstract_representation = Vec::with_capacity(6);

    let mut col = 0;
    let mut row = 0;
    loop {
        if abstract_representation.len() == 6 {
            break;
        }
        if map_raw[row * face_size][col * face_size] != ' ' {
            abstract_representation.push((
                (col, row),
                (0..face_size)
                    .map(|col_relative| {
                        (0..face_size)
                            .map(|row_relative| {
                                let char = map_raw[row * face_size + row_relative]
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
            ));
        }

        if map_raw[row * face_size]
            .get((col + 1) * face_size)
            .is_some()
        {
            col += 1;
        } else {
            row += 1;
            col = 0;
        }
    }

    println!("LENGTH: {}", all.len());
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
