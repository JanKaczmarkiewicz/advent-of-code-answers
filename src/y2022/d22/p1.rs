use crate::utils::read_lines;

pub fn answer() {
    read_lines("src/y2022/dXX/input");

    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_compute_solution() {
        assert_eq!(answer(), 0);
    }
}
