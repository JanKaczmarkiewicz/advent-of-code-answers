use crate::utils::read_lines;

pub fn answer() {
    println!("Answer to day19: {} {}", a1(), a2());
}

fn a1() -> usize {
    read_lines("src/y2022/d19/input");

    0
}

fn a2() -> usize {
    read_lines("src/y2022/d19/input");

    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_solve_first_problem() {
        assert_eq!(a1(), 0);
    }

    #[test]
    fn should_solve_second_problem() {
        assert_eq!(a2(), 0);
    }
}
