use crate::y2022::d24::shared::{self, Destination};

pub fn answer() -> i64 {
    shared::answer(&[Destination::End, Destination::Start, Destination::End])
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_compute_solution() {
        assert_eq!(
            shared::answer(&[Destination::End, Destination::Start, Destination::End]),
            0
        );
    }
}
