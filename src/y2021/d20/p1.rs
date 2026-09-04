use std::collections::HashSet;

use itertools::Itertools;

use crate::utils::read_lines;

const SCANNED_AREA: [(i32, i32); 9] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (0, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

pub fn answer() -> usize {
    let mut iter = read_lines("src/y2021/d20/input");

    let alg = iter.next().unwrap();

    iter.next();

    let mut img = HashSet::new();

    for (y, line) in iter.enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                img.insert((x as i32, y as i32));
            }
        }
    }

    // dbg!(&img, &alg);
    for _ in 0..2 {
        let (min_x, max_x) = img.iter().map(|(x, _)| *x).minmax().into_option().unwrap();
        let (min_y, max_y) = img.iter().map(|(_, y)| *y).minmax().into_option().unwrap();

        let mut new_img = HashSet::new();
        for x in min_x - 1..=max_x + 1 {
            for y in min_y - 1..=max_y + 1 {
                let index = SCANNED_AREA
                    .map(|(x_mod, y_mod)| {
                        img.get(&(x + x_mod, y + y_mod))
                            .map(|_| 1)
                            .unwrap_or_default()
                    })
                    .iter()
                    .fold(0, |acc, digit| (acc << 1) + digit);

                if let Some('#') = alg.chars().nth(index) {
                    new_img.insert((x, y));
                }
            }
        }
        img = new_img;
    }

    img.len()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_compute_solution() {
        assert_eq!(answer(), 5402);
    }
}
