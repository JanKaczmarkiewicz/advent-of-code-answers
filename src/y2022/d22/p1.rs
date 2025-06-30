use crate::utils::read_lines;

pub fn answer() -> i64 {
    read_lines("src/y2022/d22/input");

    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_compute_solution() {
        assert_eq!(answer(), 6032);
    }
}
