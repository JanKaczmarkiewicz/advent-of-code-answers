use crate::utils::read;
use itertools::{self, Itertools};

pub fn answer() {
    println!("Answer to day6: {} {}", a1(), a2());
}

pub fn a1() -> usize {
    let communication: Vec<_> = read("src/y2022/d6/input").chars().collect();

    for i in 0..communication.len() {

        const NUMBER_OF_UNIQUE_CHARS: usize = 4; 

        if communication[i..i+NUMBER_OF_UNIQUE_CHARS].iter().unique().count() == NUMBER_OF_UNIQUE_CHARS {
            return i + NUMBER_OF_UNIQUE_CHARS;
        }
    }
    
    return 0
}

pub fn a2() -> usize {
    let communication: Vec<_> = read("src/y2022/d6/input").chars().collect();

    for i in 0..communication.len() {

        const NUMBER_OF_UNIQUE_CHARS: usize = 14; 

        if communication[i..i+NUMBER_OF_UNIQUE_CHARS].iter().unique().count() == NUMBER_OF_UNIQUE_CHARS {
            return i + NUMBER_OF_UNIQUE_CHARS;
        }
    }
    
    return 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_first_problem() {
        assert_eq!(a1(), 1480);
    }

    #[test]
    fn should_solve_second_problem() {
        assert_eq!(a2(), 2746);
    }
}
