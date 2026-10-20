use std::collections::HashMap;

use itertools::Itertools;

use crate::utils::read_lines;

fn mod_1_10(num: usize) -> usize {
    ((num - 1) % 10) + 1
}

pub fn answer() -> u128 {
    let (p1, p2) = read_lines("src/y2021/d21/input")
        .map(|s| s.chars().last().unwrap().to_digit(10).unwrap() as usize)
        .next_tuple()
        .unwrap();

    let mut queue = HashMap::from([(((p1, 0), (p2, 0)), 1)]);
    let dice_distribution = [1, 2, 3]
        .into_iter()
        .cartesian_product([1, 2, 3])
        .cartesian_product([1, 2, 3])
        .map(|((a, b), c)| a + b + c)
        .counts();

    // so for every dice roll the computation separates

    let mut p1_wins: u128 = 0;
    let mut p2_wins: u128 = 0;

    // TODO: hmm there seems to be explosion of universes.
    // There might be a solution that will memorize, so it will effectiely cut recursive braches
    dbg!(&dice_distribution);

    while let Some((((p1, p1_score), (p2, p2_score)), count)) = &queue
        .keys()
        .next()
        .map(|k| k.clone())
        .and_then(|k| queue.remove_entry(&k))
    {
        for (dice_outcome_p1, dice_outcome_p1_times) in &dice_distribution {
            let t_p1 = mod_1_10(p1 + dice_outcome_p1);
            let t_p1_score = p1_score + t_p1;

            if t_p1_score >= 21 {
                p1_wins += count * *dice_outcome_p1_times as u128;
                continue;
            }

            for (dice_outcome_p2, dice_outcome_p2_times) in &dice_distribution {
                let t_p2 = mod_1_10(p2 + dice_outcome_p2);
                let t_p2_score = p2_score + t_p2;

                if t_p2_score >= 21 {
                    p2_wins += count * (dice_outcome_p1_times * dice_outcome_p2_times) as u128;
                    continue;
                } else {
                    let x = queue
                        .entry(((t_p1, t_p1_score), (t_p2, t_p2_score)))
                        .or_insert(0);
                    *x += count * (dice_outcome_p1_times * dice_outcome_p2_times) as u128;
                }
            }
        }
    }

    p1_wins.max(p2_wins)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_compute_solution() {
        assert_eq!(answer(), 712381680443927);
    }
}
