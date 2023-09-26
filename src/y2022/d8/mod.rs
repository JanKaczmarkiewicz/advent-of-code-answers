use crate::utils::read_lines;

pub fn answer() {
    println!("Answer to day8: {} {}", a1(), a2());
}

pub fn a1() -> usize {
    let map_of_trees = read_lines("src/y2022/d8/input")
        .map(|row| {
            row.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut count = 2 * map_of_trees.len() + 2 * (map_of_trees[0].len() - 2);

    let is_visible = |i: usize, j: usize| {
        let current_tree_height = map_of_trees[i][j];

        // top
        if (0..i).all(|k| map_of_trees[k][j] < current_tree_height) {
            return true;
        }
        // bottom
        if (i + 1..map_of_trees.len()).all(|k| map_of_trees[k][j] < current_tree_height) {
            return true;
        }
        // left
        if (0..j).all(|k| map_of_trees[i][k] < current_tree_height) {
            return true;
        }
        // right
        if (j + 1..map_of_trees[0].len()).all(|k| map_of_trees[i][k] < current_tree_height) {
            return true;
        }

        false
    };

    for i in 1..map_of_trees.len() - 1 {
        for j in 1..map_of_trees[i].len() - 1 {
            if is_visible(i, j) {
                count += 1;
            }
        }
    }

    count
}

pub fn a2() -> usize {
    let map_of_trees = read_lines("src/y2022/d8/input")
        .map(|row| {
            row.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut max_score = 0;

    for i in 0..map_of_trees.len() {
        for j in 0..map_of_trees[i].len() {
            let mut top_count = 0;
            for k in (0..i).rev() {
                top_count += 1;
                if map_of_trees[i][j] <= map_of_trees[k][j] {
                    break;
                }
            }

            let mut bottom_count = 0;
            for k in i + 1..map_of_trees.len() {
                bottom_count += 1;
                if map_of_trees[i][j] <= map_of_trees[k][j] {
                    break;
                }
            }

            let mut left_count = 0;
            for k in (0..j).rev() {
                left_count += 1;
                if map_of_trees[i][j] <= map_of_trees[i][k] {
                    break;
                }
            }

            let mut right_count = 0;
            for k in j + 1..map_of_trees[0].len() {
                right_count += 1;
                if map_of_trees[i][j] <= map_of_trees[i][k] {
                    break;
                }
            }

            max_score = max_score.max(top_count * bottom_count * left_count * right_count);
        }
    }

    max_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_first_problem() {
        assert_eq!(a1(), 1818);
    }

    #[test]
    fn should_solve_second_problem() {
        assert_eq!(a2(), 368368);
    }
}
