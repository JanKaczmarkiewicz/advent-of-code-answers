use crate::utils::read_lines;

fn read_octopuses() -> Vec<Vec<u32>> {
     read_lines("src/a11/input")
        .map(|line|
            line
                .chars()
                .map(|char| {
                    char.to_digit(10).unwrap()
                })
                .collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn flash_rec(x: usize, y: usize, octopuses: &mut Vec<Vec<u32>>) {

    if octopuses[y][x] > 9 {
        octopuses[y][x] = 0;

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
            if x >= 0 && y >= 0 && x < octopuses[0].len() as i32  && y < octopuses.len() as i32 && octopuses[y as usize][x as usize] != 0 {
                octopuses[y as usize ][x as usize] += 1;
                flash_rec(x as usize, y as usize, octopuses);
            }
        }
    }
}

fn a() -> u32 {
    let mut octopuses = read_octopuses();

    let mut sum = 0;

    for _i in 0..100 {
        for y in 0..octopuses.len() {
            for x in 0..octopuses[y].len() {
                octopuses[y][x] += 1;
            }
        }

        for y in 0..octopuses.len() {
            for x in 0..octopuses[y].len() {
                flash_rec(x, y, &mut octopuses);
            }
        }

        sum += octopuses.iter().flatten().fold(0, |prev, curr| if *curr == 0 {1} else {0} + prev )
    }

    sum
}

fn b() -> u32 {
    let mut octopuses = read_octopuses();

    for i in 0..1000 {
        for y in 0..octopuses.len() {
            for x in 0..octopuses[y].len() {
                octopuses[y][x] += 1;
            }
        }

        for y in 0..octopuses.len() {
            for x in 0..octopuses[y].len() {
                flash_rec(x, y, &mut octopuses);
            }
        }

        if octopuses.iter().flatten().all(|x| *x == 0) {
            return i + 1;
        }
    }

    0
}



pub fn answer() {
    println!("Answer to problem 11: {}, {}", a(), b());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_first_problem() {
        assert_eq!(a(), 1713);
    }

    #[test]
    fn should_solve_second_problem() {
        assert_eq!(b(), 502);
    }
}