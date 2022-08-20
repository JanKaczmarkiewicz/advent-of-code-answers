mod data;
mod explode;
use crate::{a18::data::Data, utils::read};
use explode::explode;

fn a() -> i32 {
    let result = read("src/a18/input")
        .lines()
        .flat_map(serde_json::from_str::<Data>)
        .map(explode)
        .collect::<Vec<_>>();

    println!("{:?}", result);
    0
}

fn b() -> usize {
    0
}

pub fn answer() {
    println!("Answer to problem 18: {}, {}", a(), b());
}

#[cfg(test)]
mod tests {
    use super::a;

    #[test]
    fn should_solve_first_problem() {
        a();
    }

    #[test]
    fn should_solve_second_problem() {}
}
