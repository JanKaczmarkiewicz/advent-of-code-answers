use crate::{
    utils::read_lines,
    y2023::d13::p1::{get, mirror_vertical},
};

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
            if is_smudge {
                was_smudge = true
            };

            up == down || is_smudge
        } {
            dir_mod += 1;
        }

        if dir_mod != 0
            && was_smudge
            && (get(board, row_i as isize - dir_mod) == None
                || get(board, row_i as isize + dir_mod + 1) == None)
        {
            return Some(row_i + 1);
        }
    }

    None
}

pub fn answer() -> usize {
    let mut curr_board = vec![];
    let mut result = 0;

    for line in read_lines("src/y2023/d13/input_example") {
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
