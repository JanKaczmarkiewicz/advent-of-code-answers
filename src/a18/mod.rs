mod data;
mod reduce;
use crate::{a18::data::Data, utils::read};
use reduce::reduce;

fn magnitude(data: Data) -> u32 {
    match data {
        Data::Pair(pair) => {
            let pair = *pair;
            let (left, right) = pair;

            magnitude(left) * 3 + magnitude(right) * 2
        }
        Data::Integer(n) => n as u32,
    }
}

fn a() -> u32 {
    let result = read("src/a18/input")
        .lines()
        .flat_map(serde_json::from_str::<Data>)
        .map(reduce)
        .fold(Data::Integer(0), |acc, element| {
            if let Data::Pair(_) = acc {
                reduce(Data::Pair(Box::new((acc, element))))
            } else {
                element
            }
        });

    magnitude(result)
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
        assert_eq!(a(), 4140);
    }

    #[test]
    fn should_solve_second_problem() {}
}
