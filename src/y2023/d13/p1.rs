use itertools::equal;

use crate::utils::read_lines;

fn get<T>(v: &Vec<T>, i: isize) -> Option<&T> {
    if i.is_negative() {
        None
    } else {
        v.get(i as usize)
    }
}

fn mirror_horizontal(board: &Vec<String>) -> Option<usize> {
    for row_i in 0..board.len() {
        let mut dir_mod = 0;
        while get(board, row_i as isize - dir_mod) == get(board, row_i as isize + dir_mod + 1) {
            dir_mod += 1;
        }

        if dir_mod != 0
            && (get(board, row_i as isize - dir_mod) == None
                || get(board, row_i as isize + dir_mod + 1) == None)
        {
            return Some(row_i + 1);
        }
    }

    None
}

fn mirror_vertical(board: &Vec<String>) -> Option<usize> {
    let col = |i: isize| {
        board.iter().map(move |row| {
            if i.is_negative() {
                None
            } else {
                row.chars().nth(i as usize)
            }
        })
    };

    for col_i in 0..board[0].len() {
        let mut dir_mod = 0;

        while equal(
            col(col_i as isize - dir_mod),
            col(col_i as isize + 1 + dir_mod),
        ) {
            dir_mod += 1;
        }

        if dir_mod != 0
            && (col(col_i as isize - dir_mod).next().unwrap() == None
                || col(col_i as isize + 1 + dir_mod).next().unwrap() == None)
        {
            return Some(col_i + 1);
        }
    }

    None
}

pub fn answer() -> usize {
    let mut curr_board = vec![];
    let mut result = 0;

    for line in read_lines("src/y2023/d13/input") {
        if line.replace(" ", "").is_empty() {
            if let Some(mirror) = mirror_horizontal(&curr_board) {
                result += (mirror) * 100;
            } else if let Some(mirror) = mirror_vertical(&curr_board) {
                result += mirror;
            }

            curr_board.clear();
            continue;
        }
        curr_board.push(line);
    }

    if let Some(mirror) = mirror_horizontal(&curr_board) {
        result += mirror * 100;
    } else if let Some(mirror) = mirror_vertical(&curr_board) {
        result += mirror;
    }

    result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_compute_solution() {
        assert_eq!(answer(), 0);
    }
}
