use crate::utils::read_lines;

fn perform_folds(arr: &mut Vec<Vec<bool>>, instructions: &[(String, usize)]) {
    for (axis, position) in instructions {
        let position = *position;
        match &axis[..] {
            "x" => {
                for y in 0..arr.len() {
                    for x in 0..position {
                        arr[y][x] |= arr[y][2 * position - x];
                    }
                    arr[y].truncate(position);
                }
            }
            "y" => {
                for y in 0..position {
                    for x in 0..arr[y].len() {
                        arr[y][x] |= arr[2 * position - y][x];
                    }
                }

                arr.truncate(position);
            }
            _ => {}
        }
    }
}

fn get_data() -> (Vec<Vec<bool>>, Vec<(String, usize)>) {
    let data = read_lines("src/a13/input").collect::<Vec<_>>();

    let separator_position = data.iter().position(|line| line.is_empty()).unwrap();

    let points = &data[0..separator_position]
        .iter()
        .map(|line| {
            let coordinates = line
                .split(',')
                .map(|raw| raw.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            (coordinates[0], coordinates[1])
        })
        .collect::<Vec<_>>();

    let max_x = *points.iter().map(|(x, _y)| x).max().unwrap();

    let max_y = *points.iter().map(|(_x, y)| y).max().unwrap();

    let instructions = data[separator_position + 1..data.len()]
        .iter()
        .map(|line| {
            let fold_instruction = line
                .split("fold along ")
                .nth(1)
                .unwrap()
                .split('=')
                .collect::<Vec<_>>();
            (
                String::from(fold_instruction[0]),
                fold_instruction[1].parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut arr = vec![vec![false; max_x + 1 + 3]; max_y + 1 + 3];

    for (x, y) in points {
        arr[*y][*x] = true;
    }

    (arr, instructions)
}

fn a() -> u32 {
    let (mut arr, instructions) = get_data();

    perform_folds(&mut arr, &instructions[0..1]);

    arr.iter()
        .map(|row| row.iter().map(|x| *x as u32).sum::<u32>())
        .sum()
}

fn b() -> String {
    let (mut arr, instructions) = get_data();

    perform_folds(&mut arr, &instructions[..]);

    let mut code = String::new();

    for row in &arr {
        for cell in row {
            code.push_str(if *cell { "*" } else { " " });
        }
        code.push('\n');
    }

    code
}

pub fn answer() {
    println!("Answer to problem 13: {} \n{}", a(), b());
}

#[cfg(test)]
mod tests {
    use super::{a, b};

    #[test]
    fn should_solve_first_problem() {
        assert_eq!(a(), 675);
    }

    #[test]
    fn should_solve_second_problem() {
        let code = "*  * **** *  * *  * **** ****   ** **** \n*  *    * * *  *  * *    *       *    * \n****   *  **   **** ***  ***     *   *  \n*  *  *   * *  *  * *    *       *  *   \n*  * *    * *  *  * *    *    *  * *    \n*  * **** *  * *  * *    ****  **  **** \n";
        assert_eq!(b(), code);
    }
}
