use std::iter::zip;

use crate::y2023::d22::p1::{initial_bricks, stabilize};

pub fn answer() -> usize {
    let bricks = initial_bricks();

    (0..bricks.len())
        .map(|disabled| {
            let mut other_bricks = bricks.clone();
            other_bricks.remove(disabled);
            let other_bricks_not_stabilized = other_bricks.clone();
            stabilize(&mut other_bricks);

            zip(other_bricks_not_stabilized, other_bricks)
                .filter(|(l, r)| l != r)
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_compute_solution() {
        assert_eq!(answer(), 0);
    }
}
