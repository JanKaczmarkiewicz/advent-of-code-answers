use crate::utils::read_lines;

fn a() -> u32 {
    read_lines("src/a10/input").map(|line| {
        let chars = line
            .chars()
            .collect::<Vec<_>>();

        let mut opened = vec![];

        let mut error_score = 0;

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
    }).fold(0, |prev, curr| curr + prev)
}

pub fn answer() {
    println!("Answer to problem 10: {}", a());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_first_problem() {
        assert_eq!(a(), 270);
    }

}