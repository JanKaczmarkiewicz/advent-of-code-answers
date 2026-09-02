use itertools::Itertools;

use crate::y2021::d19::shared::solve;

pub fn answer() -> usize {
    let alligned_scanners = solve();

    return alligned_scanners
        .into_values()
        .map(|(c, _)| c)
        .combinations(2)
        .map(|e| {
            let a = e[0];
            let b = e[1];

            ((a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs()) as usize
        })
        .max()
        .unwrap();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_compute_solution() {
        assert_eq!(answer(), 11860);
    }
}
