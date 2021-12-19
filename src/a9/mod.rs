use crate::utils::{read_lines};
use itertools::Itertools;

fn a() -> i32 {
    let matrix = read_lines("src/a9/input").map(|line| {
        line
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let get_cell_safe = |x: i32, y: i32| -> i32 {
        if x >= 0 && y >= 0{
            if let Some(row) = matrix.iter().nth(y as usize) {
                if let Some(&cell) = row.iter().nth(x as usize) {
                    return cell as i32;
                }
            }
        }
        10
    };

    let mut sum = 0;

    for (y, row) in matrix.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            let cell = get_cell_safe(x as i32, y as i32);

            let top_cell = get_cell_safe(x as i32, y as i32 - 1);
            let bottom_cell = get_cell_safe(x as i32, y as i32 + 1);
            let left_cell = get_cell_safe(x as i32 + 1, y as i32);
            let right_cell = get_cell_safe(x as i32 - 1, y as i32);

            let is_depression = cell < top_cell && cell < bottom_cell && cell < left_cell && cell < right_cell;

            if is_depression {
                sum += 1 + cell;
            }
        }
    }

    sum
}

pub fn answer() {
    println!("Answer to problem 9: {}", a());
}