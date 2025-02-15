use std::collections::HashSet;

use itertools::Itertools;

use crate::utils::read_lines;

pub fn answer() {
    println!("Answer to day18: {} {}", a1(), a2());
}

fn a1() -> usize {
    let mut faces = HashSet::new();

    let cords_iter = read_lines("src/y2022/d18/input").map(|str| {
        str.split(',')
            .map(|s| s.parse::<i16>().unwrap())
            .collect_tuple::<(_, _, _)>()
            .unwrap()
    });

    for (x, y, z) in cords_iter {
        let new_faces = [
            (x, y, z, 'x'),
            (x, y, z, 'y'),
            (x, y, z, 'z'),
            (x - 1, y, z, 'x'),
            (x, y - 1, z, 'y'),
            (x, y, z - 1, 'z'),
        ];

        for face in new_faces {
            if !faces.insert(face) {
                faces.remove(&face);
            }
        }
    }

    faces.len()
}
fn a2() -> usize {
    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_solve_first_problem() {
        assert_eq!(a1(), 4456);
    }

    #[test]
    fn should_solve_second_problem() {
        assert_eq!(a2(), 0);
    }
}
