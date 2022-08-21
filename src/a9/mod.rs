use itertools::Itertools;

use crate::utils::read_lines;

fn read_matrix() -> Vec<Vec<u32>> {
    read_lines("src/a9/input")
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn get_cell_safe(x: i32, y: i32, matrix: &[Vec<u32>]) -> i32 {
    if x >= 0 && y >= 0 {
        if let Some(row) = matrix.get(y as usize) {
            if let Some(&cell) = row.get(x as usize) {
                return cell as i32;
            }
        }
    }
    9
}

fn find_critical_points(matrix: &[Vec<u32>]) -> Vec<(i32, i32)> {
    let mut critical_points = vec![];

    for (i, row) in matrix.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            let y = i as i32;
            let x = j as i32;

            let cell = cell as i32;

            let top_cell = get_cell_safe(x, y - 1, matrix);
            let bottom_cell = get_cell_safe(x, y + 1, matrix);
            let left_cell = get_cell_safe(x + 1, y, matrix);
            let right_cell = get_cell_safe(x - 1, y, matrix);

            let is_depression =
                cell < top_cell && cell < bottom_cell && cell < left_cell && cell < right_cell;

            if is_depression {
                critical_points.push((x, y));
            }
        }
    }

    critical_points
}

fn a() -> i32 {
    let matrix = read_matrix();

    find_critical_points(&matrix)
        .iter()
        .fold(0, |prev, (x, y)| get_cell_safe(*x, *y, &matrix) + prev)
}

fn b() -> usize {
    let matrix = read_matrix();

    fn traverse_basin(x: i32, y: i32, matrix: &Vec<Vec<u32>>, points: &mut Vec<(i32, i32)>) {
        points.push((x, y));

        let directions = vec![(x, y - 1), (x, y + 1), (x + 1, y), (x - 1, y)];

        for (x, y) in directions {
            if get_cell_safe(x, y, matrix) != 9
                && !points
                    .iter()
                    .any(|(curr_x, curr_y)| *curr_x == x && *curr_y == y)
            {
                traverse_basin(x, y, matrix, points);
            }
        }
    }

    let basin_sizes = find_critical_points(&matrix)
        .iter()
        .map(|(x, y)| {
            let mut points = vec![];
            traverse_basin(*x, *y, &matrix, &mut points);
            points.len()
        })
        .sorted()
        .collect::<Vec<_>>();

    basin_sizes[basin_sizes.len() - 1]
        * basin_sizes[basin_sizes.len() - 2]
        * basin_sizes[basin_sizes.len() - 3]
}

pub fn answer() {
    println!("Answer to problem 9: {} {}", a(), b());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_first_problem() {
        assert_eq!(a(), 270);
    }

    #[test]
    fn should_solve_second_problem() {
        assert_eq!(b(), 1092012);
    }
}
