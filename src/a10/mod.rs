use itertools::Itertools;
use crate::utils::read_lines;

fn get_line_error_score(line: &String) -> u32 {
    let chars = line
        .chars()
        .collect::<Vec<_>>();

    let mut error_score = 0;
    let mut opened = vec![];

    for char in chars {
        match char {
            '{' | '<' | '(' | '[' => {
                opened.push(char)
            }
            ']' => {
                if opened.pop().unwrap() != '[' {
                    error_score += 57;
                }
            }
            '}' => {
                if opened.pop().unwrap() != '{' {
                    error_score += 1197;
                }
            }
            '>' => {
                if opened.pop().unwrap() != '<' {
                    error_score += 25137;
                }
            }
            ')' => {
                if opened.pop().unwrap() != '(' {
                    error_score += 3;
                }
            }
            _ => {}
        }
    }
    error_score
}

fn a() -> u32 {
    read_lines("src/a10/input")
        .map(|line| get_line_error_score(&line))
        .fold(0, |prev, curr| curr + prev)
}

fn b() -> u64 {
    let sorted_result = read_lines("src/a10/input")
        .filter(|line| get_line_error_score(line) == 0)
        .map(|line| {
            let chars = line
                .chars()
                .collect::<Vec<_>>();

            let mut opened = vec![];

            for char in chars {
                match char {
                    '{' | '<' | '(' | '[' => { opened.push(char); },
                    ']' | '}' | '>' | ')' => { opened.pop(); },
                    _ => {}
                }
            }

            opened.iter().rev().fold(0, |prev, curr| {
                let modifier: u64 = match curr {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => 0,
                };
                5 * prev + modifier
            })
    }).sorted().collect::<Vec<_>>();

    sorted_result[sorted_result.len() / 2]
}





pub fn answer() {
    println!("Answer to problem 10: {} {}", a(), b());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_first_problem() {
        assert_eq!(a(), 370407);
    }

}