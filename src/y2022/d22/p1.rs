use std::collections::HashMap;

use itertools::Itertools;

use crate::utils::read_lines;

#[derive(PartialEq, Eq)]
enum Tile {
    Wall,
    Movable,
}

pub fn answer() -> i64 {
    let mut operation = String::new();
    let mut map = HashMap::new();

    for (row, line) in read_lines("src/y2022/d22/input").enumerate() {
        if line.is_empty() {
            continue;
        }

        if line.chars().next().unwrap().is_numeric() {
            operation = line;
            break;
        }

        map.extend(line.char_indices().flat_map(|(column, char)| {
            if char == '.' {
                Some(((row as i64, column as i64), Tile::Movable))
            } else if char == '#' {
                Some(((row as i64, column as i64), Tile::Wall))
            } else {
                None
            }
        }));
    }

    let perspective_up = (-1, 0);
    let perspective_down = (1, 0);
    let perspective_left = (0, -1);
    let perspective_right = (0, 1);

    let mut perspective = perspective_right;
    let mut position = (
        0_i64,
        *map.iter()
            .filter(|((row, _), v)| *row == 0 && **v == Tile::Movable)
            .map(|((_, col), _)| col)
            .min()
            .unwrap(),
    );

    let mut chars = operation.chars();

    while let Ok(n) = chars
        .peeking_take_while(|x| x.is_numeric())
        .collect::<String>()
        .parse::<u64>()
    {
        // add position go times (check for colision)
        for _ in 0..n {
            let next_pos_row = position.0 + perspective.0;
            let next_pos_col = position.1 + perspective.1;

            let obvious_next_position = (position.0 + perspective.0, position.1 + perspective.1);

            let next_position = if map.get(&obvious_next_position).is_some() {
                obvious_next_position
            } else {
                if perspective == perspective_up {
                    *map.keys()
                        .filter(|(_, col)| *col == next_pos_col)
                        .max_by(|(l_row, _), (r_row, _)| l_row.cmp(r_row))
                        .unwrap()
                } else if perspective == perspective_left {
                    *map.keys()
                        .filter(|(row, _)| *row == next_pos_row)
                        .max_by(|(_, l_col), (_, r_col)| l_col.cmp(r_col))
                        .unwrap()
                } else if perspective == perspective_down {
                    *map.keys()
                        .filter(|(_, col)| *col == next_pos_col)
                        .min_by(|(l_row, _), (r_row, _)| l_row.cmp(r_row))
                        .unwrap()
                } else if perspective == perspective_right {
                    *map.keys()
                        .filter(|(row, _)| *row == next_pos_row)
                        .min_by(|(_, l_col), (_, r_col)| l_col.cmp(r_col))
                        .unwrap()
                } else {
                    panic!()
                }
            };

            match map.get(&next_position).unwrap() {
                Tile::Movable => position = next_position,
                Tile::Wall => break,
            }
        }

        if let Some(rotation) = chars.next() {
            perspective = match rotation {
                'L' => {
                    if perspective == perspective_up {
                        perspective_left
                    } else if perspective == perspective_left {
                        perspective_down
                    } else if perspective == perspective_down {
                        perspective_right
                    } else {
                        perspective_up
                    }
                }
                'R' => {
                    if perspective == perspective_up {
                        perspective_right
                    } else if perspective == perspective_right {
                        perspective_down
                    } else if perspective == perspective_down {
                        perspective_left
                    } else {
                        perspective_up
                    }
                }
                _ => panic!(),
            }
        }
    }

    1000 * (position.0 + 1)
        + 4 * (position.1 + 1)
        + if perspective == perspective_up {
            3
        } else if perspective == perspective_right {
            0
        } else if perspective == perspective_down {
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
        assert_eq!(answer(), 11464);
    }
}
