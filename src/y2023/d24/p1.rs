use itertools::Itertools;

use crate::utils::read_lines;

pub fn answer() -> i64 {
    let hail_stones = read_lines("src/y2023/d24/input")
        .map(|line| {
            let (px, py, _, vx, vy, _) = line
                .replace(',', "")
                .split_ascii_whitespace()
                .flat_map(|s| s.parse::<f64>())
                .collect_tuple()
                .unwrap();

            ((px, py), (vx, vy))
        })
        .collect_vec();

    let mut intersections = 0;

    for (i, a) in hail_stones.iter().enumerate() {
        for (iter2, b) in hail_stones[i + 1..].iter().enumerate() {
            let res = [b.0 .0 - a.0 .0, b.0 .1 - a.0 .1];

            let det = 1.0 / (a.1 .0 * b.1 .1 - a.1 .1 * b.1 .0);
            let A_inv = [[b.1 .1, -b.1 .0], [-a.1 .0, -a.1 .0]];

            let a_scalar = (res[0] * A_inv[0][0] + res[1] * A_inv[0][1]) * det;
            let b_scalar = -(res[0] * A_inv[1][0] + res[1] * A_inv[1][1]) * det;

            let x = a.0 .0 + a_scalar * a.1 .0;
            let y = a.0 .1 + a_scalar * a.1 .1;

            let b_x = b.0 .0 + b_scalar * b.1 .0;
            let b_y = b.0 .1 + b_scalar * b.1 .1;

            assert_eq!((x, y), (b_x, b_y));

            // println!("Hailstone A: {a:?}");
            // println!("Hailstone B: {b:?}");
            // const FROM: f64 = 7.0;
            // const TO: f64 = 27.0;
            const FROM: f64 = 200000000000000.0;
            const TO: f64 = FROM * 2.0;

            if a_scalar >= 0.0 && b_scalar >= 0.0 && x >= FROM && x <= TO && y >= FROM && y <= TO {
                println!("{i}, {iter2}");
                intersections += 1;
            }

            println!("{x}, {y}, {a_scalar}, {b_scalar}");
            println!();
        }
    }

    intersections
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_compute_solution() {
        assert_eq!(answer(), 0);
    }
}
