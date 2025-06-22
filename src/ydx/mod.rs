use crate::utils::read_lines;

pub fn answer() {
    println!("Answer to dayxb: {} {}", a1(), a2());
}

fn a1() -> i64 {
    let mut list = read_lines("src/y2022/dx/input");
    0
}

fn a2() -> i64 {
    let mut list = read_lines("src/y2022/dx/input");
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
