use itertools::Itertools;

use crate::utils::read_lines;

pub fn answer() -> usize {
    let mut die_iter = (1..=100).cycle();

    let (mut p1, mut p2) = read_lines("src/y2021/d21/input")
        .map(|s| s.chars().last().unwrap().to_digit(10).unwrap() as usize)
        .next_tuple()
        .unwrap();

    let mut total = 0;
    let mut die = || {
        let (d1, d2, d3) = die_iter.next_tuple().unwrap();
        total += 3;
        d1 + d2 + d3
    };

    let mut p1_score = 0;
    let mut p2_score = 0;

    fn mod_1_10(num: usize) -> usize {
        ((num - 1) % 10) + 1
    }

    loop {
        p1 = mod_1_10(p1 + die());
        p1_score += p1;

        if p1_score >= 1000 {
            break;
        }

        p2 = mod_1_10(p2 + die());
        p2_score += p2;

        if p2_score >= 1000 {
            break;
        }
    }

    p1_score.min(p2_score) * total
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_compute_solution() {
        assert_eq!(answer(), 757770);
    }
}
