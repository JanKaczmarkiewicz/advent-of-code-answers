use crate::utils::read_lines;

pub fn answer() {
    println!("Answer to problem 12: {} {}", a1(), a2());
}

fn read_pairs() -> impl Iterator<Item = ((i16, i16), (i16, i16))> {
    read_lines("src/y2022/d4/input").map(|line| {
        let left_and_right = line
            .split(',')
            .map(|e| {
                let range = e
                    .split('-')
                    .map(|e| e.parse::<_>().unwrap())
                    .collect::<Vec<i16>>();
                (range[0], range[1])
            })
            .collect::<Vec<_>>();
        (left_and_right[0], left_and_right[1])
    })
}

pub fn a1() -> usize {
    read_pairs()
        .filter(|(left, right)| {
            (left.0 <= right.0 && left.1 >= right.1) || (right.0 <= left.0 && right.1 >= left.1)
        })
        .count()
}

pub fn a2() -> usize {
    read_pairs()
        .filter(|(left, right)| {
            (left.0..=left.1).contains(&right.0)
                || (left.0..=left.1).contains(&right.1)
                || (right.0..=right.1).contains(&left.0)
                || (right.0..=right.1).contains(&left.1)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_first_problem() {
        assert_eq!(a1(), 424);
    }

    #[test]
    fn should_solve_second_problem() {
        assert_eq!(a2(), 804);
    }
}
