use itertools::Itertools;

use crate::y2021::d19::shared::solve;

pub fn answer() -> usize {
    let alligned_scanners = solve();

    return alligned_scanners
        .into_values()
        .map(|(_, v)| v)
        .flatten()
        .unique()
        .count();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_compute_solution() {
        assert_eq!(answer(), 430);
    }
}
