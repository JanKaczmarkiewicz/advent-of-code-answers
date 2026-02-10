use crate::utils::read_lines;
use itertools::Itertools;

fn get<T>(v: &Vec<T>, i: isize) -> Option<&T> {
    i.is_positive().then_some(v.get(i as usize)).flatten()
}

fn find_difference(l: &String, r: &String) -> Option<usize> {
    let mut differance_found = None;
    for (i, (l, r)) in l.chars().zip(r.chars()).enumerate() {
        if l != r {
            if differance_found.is_some() {
                return None;
            }
            differance_found = Some(i);
        }
    }

    return differance_found;
}

pub fn mirror_horizontal(board: &Vec<String>) -> Option<usize> {
    for row_i in 0..board.len() {
        let mut dir_mod = 0;

        let mut was_smudge = false;
        while {
            let up = get(board, row_i as isize - dir_mod);
            let down = get(board, row_i as isize + dir_mod + 1);
            let is_smudge = if let (Some(r), Some(l)) = (up, down) {
                find_difference(r, l).is_some()
            } else {
                false
            };
            if is_smudge && !was_smudge {
                was_smudge = true;
                true
            } else {
                up == down
            }
        } {
            dir_mod += 1;
        }

        if dir_mod != 0
            && was_smudge
            && (get(board, row_i as isize - dir_mod).is_none()
                || get(board, row_i as isize + dir_mod + 1).is_none())
        {
            return Some(row_i + 1);
        }
    }

    None
}

pub fn mirror_vertical(board: &Vec<String>) -> Option<usize> {
    let board_transposed = (0..board[0].len())
        .map(|i| {
            board
                .iter()
                .map(|row| row.chars().nth(i as usize).unwrap())
                .join("")
        })
        .collect::<Vec<_>>();

    mirror_horizontal(&board_transposed)
}

fn find_mirror(board: &Vec<String>) -> usize {
    if let Some(mirror) = mirror_horizontal(&board) {
        (mirror) * 100
    } else if let Some(mirror) = mirror_vertical(&board) {
        mirror
    } else {
        panic!()
    }
}

pub fn answer() -> usize {
    let mut curr_board = vec![];
    let mut result = 0;

    for line in read_lines("src/y2023/d13/input") {
        if line.replace(" ", "").is_empty() {
            result += find_mirror(&curr_board);
            curr_board.clear();
            continue;
        }
        curr_board.push(line);
    }

    result += find_mirror(&curr_board);

    result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_compute_solution() {
        assert_eq!(answer(), 36474);
    }
}
