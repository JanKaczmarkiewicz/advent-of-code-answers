use crate::utils::read_lines;

fn a() -> u32 {
    let mut octo = read_lines("src/a11/input")
        .map(|line|
            line
                .chars()
                .map(|char| {
                    char.to_digit(10).unwrap()
                })
                .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    fn flash_rec(x: usize, y: usize, octo: &mut Vec<Vec<u32>>) {

        if octo[y][x] > 9 {
            octo[y][x] = 0;

            let x = x as i32;
            let y = y as i32;

            let directions = vec![
                (x, y - 1),
                (x, y + 1),
                (x + 1, y),
                (x - 1, y),
                (x - 1, y - 1),
                (x + 1, y + 1),
                (x - 1, y + 1),
                (x + 1, y - 1),
            ];

            for (x, y) in directions {
                if x >= 0 && y >= 0 && x < octo[0].len() as i32  && y < octo.len() as i32 && octo[y as usize][x as usize] != 0 {
                    octo[y as usize ][x as usize] += 1;
                    flash_rec(x as usize, y as usize, octo);
                }
            }
        }
    }

    let mut sum = 0;

    for _i in 0..100 {
        for y in 0..octo.len() {
            for x in 0..octo[y].len() {
                octo[y][x] += 1;
            }
        }

        for y in 0..octo.len() {
            for x in 0..octo[y].len() {
                flash_rec(x, y, &mut octo);
            }
        }

        sum += octo.iter().flatten().fold(0, |prev, curr| if *curr == 0 {1} else {0} + prev )
    }

    sum
}


pub fn answer() {
    println!("Answer to problem 11: {}", a());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_first_problem() {
        assert_eq!(a(), 1713);
    }
}