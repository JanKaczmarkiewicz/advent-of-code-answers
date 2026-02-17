use itertools::Itertools;

use crate::utils::read_lines;

pub fn answer() -> i64 {
    let map = read_lines("src/y2023/d21/input_example")
        .map(|row| row.chars().collect_vec())
        .collect_vec();

    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_compute_solution() {
        assert_eq!(answer(), 3830);
    }
}
