use crate::utils::read_lines;

pub fn answer() {
    println!("Answer to problem 12: {} {}", a1(), a2());
}

pub fn a1() -> usize {
    read_lines("src/y2022/d4/input")
        .filter(|line| {
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

            let left = left_and_right[0];
            let right = left_and_right[1];

            (left.0 <= right.0 && left.1 >= right.1) || (right.0 <= left.0 && right.1 >= left.1)
        })
        .count()
}

pub fn a2() -> &'static str {
    "not implemented"
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
        // assert_eq!(b(), 118242);
    }
}
