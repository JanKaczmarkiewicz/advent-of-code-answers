use std::collections::HashSet;

use crate::utils::read_lines;

pub fn answer() {
    println!("Answer to day9: {} {}", a1(), a2());
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_instruction(instruction: String) -> impl Iterator<Item = Direction> {
    let mut chars = instruction.split(" ");

    let dir = chars.next().unwrap();
    let count = chars.next().unwrap().parse().unwrap();

    let direction = match dir {
        "U" => Direction::Up,
        "D" => Direction::Down,
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => panic!("Unknown direction"),
    };

    (0..count).map(move |_| direction)
}

// fn render(tail_visited: &HashSet<(i32, i32)>, knots: &Vec<(i32, i32)>) {
//     let mut all = tail_visited.clone();
//     all.extend(knots.iter());

//     let min_x = all.iter().map(|(x, _)| x).min().unwrap();
//     let max_x = all.iter().map(|(x, _)| x).max().unwrap();
//     let min_y = all.iter().map(|(_, y)| y).min().unwrap();
//     let max_y = all.iter().map(|(_, y)| y).max().unwrap();

//     for y in *min_y..=*max_y {
//         for x in *min_x..=*max_x {
//             if let Some(index) = knots.iter().position(|knot| knot == &(x, y)) {
//                 if knots.len() - 1 == index {
//                     print!("T")
//                 } else if index == 0 {
//                     print!("H")
//                 } else {
//                     print!("{index}")
//                 }
//             } else if tail_visited.contains(&(x, y)) {
//                 print!("#");
//             } else {
//                 print!(".");
//             }
//         }
//         println!();
//     }
//     println!("---------");
// }

fn fix_prev_knot_position(head_position: (i32, i32), prev_knot_pos: (i32, i32)) -> (i32, i32) {
    let x_distance: i32 = prev_knot_pos.0 - head_position.0;
    let y_distance = prev_knot_pos.1 - head_position.1;

    let new_distance_relative_to_head = if x_distance.abs() > y_distance.abs() {
        (x_distance.signum(), 0)
    } else if x_distance.abs() < y_distance.abs() {
        (0, y_distance.signum())
    } else {
        (x_distance.signum(), y_distance.signum())
    };

    (
        head_position.0 + new_distance_relative_to_head.0,
        head_position.1 + new_distance_relative_to_head.1,
    )
}

pub fn a1() -> usize {
    let mut tail_visited = HashSet::new();
    let starting_point: (i32, i32) = (0, 0);

    tail_visited.insert(starting_point);

    read_lines("src/y2022/d9/input")
        .flat_map(parse_instruction)
        .fold(
            (starting_point, starting_point),
            |(head_point, tail_point), direction| {
                let head_position = match direction {
                    Direction::Up => (head_point.0, head_point.1 - 1),
                    Direction::Down => (head_point.0, head_point.1 + 1),
                    Direction::Left => (head_point.0 - 1, head_point.1),
                    Direction::Right => (head_point.0 + 1, head_point.1),
                };

                // render(&tail_visited, &vec![head_point, tail_point]);

                let tail_point = fix_prev_knot_position(head_position, tail_point);

                tail_visited.insert(tail_point);
                (head_position, tail_point)
            },
        );

    tail_visited.len()
}

pub fn a2() -> usize {
    let mut tail_visited = HashSet::new();
    let starting_point: (i32, i32) = (0, 0);

    let knots: Vec<(i32, i32)> = (0..10).map(|_| (0, 0)).collect::<Vec<_>>();

    tail_visited.insert(starting_point);

    read_lines("src/y2022/d9/input")
        .flat_map(parse_instruction)
        .fold(knots, |mut knots, direction| {
            let mut knots_iter = knots.iter_mut();

            let head = knots_iter.next().unwrap();

            *head = match direction {
                Direction::Up => (head.0, head.1 - 1),
                Direction::Down => (head.0, head.1 + 1),
                Direction::Left => (head.0 - 1, head.1),
                Direction::Right => (head.0 + 1, head.1),
            };

            knots_iter.fold(head, |prev, knot| {
                *knot = fix_prev_knot_position(*prev, *knot);
                knot
            });

            tail_visited.insert(*knots.last().unwrap());

            // render(&tail_visited, &knots);

            knots
        });

    tail_visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_first_problem() {
        assert_eq!(a1(), 6236);
    }

    #[test]
    fn parse_instruction_test() {
        assert_eq!(
            parse_instruction("R 12".to_string()).collect::<Vec<_>>(),
            (0..12).map(|_| Direction::Right).collect::<Vec<_>>()
        );
    }

    #[test]
    fn fix_prev_knot_position_distance_2() {
        assert_eq!(fix_prev_knot_position((2, -2), (0, 0)), (1, -1));
    }

    #[test]
    fn should_solve_second_problem() {
        assert_eq!(a2(), 2449);
    }
}
