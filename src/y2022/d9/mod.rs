use std::{
    collections::HashSet,
    io::{stdin, stdout, Read, Write},
    process::Command,
};

use crate::utils::read_lines;

pub fn answer() {
    println!("Answer to day9: {} {}", a1(), a2());
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn parse_instruction(instruction: String) -> impl Iterator<Item = Direction> {
    let mut chars = instruction.chars();
    let direction = match chars.next().unwrap() {
        'U' => Direction::Up,
        'D' => Direction::Down,
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => panic!("Unknown direction"),
    };

    chars.next().unwrap();

    let count = chars.next().unwrap().to_digit(10).unwrap();

    (0..count).map(move |_| direction)
}

fn render(tail_visited: &HashSet<(i32, i32)>, head_point: (i32, i32), tail_point: (i32, i32)) {
    Command::new("clear")
        .spawn()
        .expect("clear command failed to start")
        .wait()
        .expect("failed to wait");

    let mut all = tail_visited.clone();
    all.insert(head_point);

    let min_x = all.iter().map(|(x, _)| x).min().unwrap();
    let max_x = all.iter().map(|(x, _)| x).max().unwrap();
    let min_y = all.iter().map(|(_, y)| y).min().unwrap();
    let max_y = all.iter().map(|(_, y)| y).max().unwrap();

    println!("{:?}", tail_visited);

    println!("{min_x} {max_x} {min_y} {max_y}");

    for y in *min_y..=*max_y {
        for x in *min_x..=*max_x {
            if (x, y) == head_point {
                print!("H");
            } else if (x, y) == tail_point {
                print!("T");
            } else if tail_visited.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!("Head: {:?}, Tail: {:?}", head_point, tail_point);

    pause();
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

                let tail_position: (i32, i32) = {
                    let is_correct_tail_position = (head_position.0 - tail_point.0).abs() <= 1
                        && (head_position.1 - tail_point.1).abs() <= 1;

                    if is_correct_tail_position {
                        tail_point
                    } else {
                        if head_position.0 == tail_point.0 {
                            (
                                tail_point.0,
                                head_position.1 + (tail_point.1 - head_position.1).signum(),
                            )
                        } else if head_position.1 == tail_point.1 {
                            (
                                head_position.0 + (tail_point.0 - head_position.0).signum(),
                                tail_point.1,
                            )
                        } else if (head_position.0 - tail_point.0).abs() == 1 {
                            (
                                head_position.0,
                                head_position.1 + (tail_point.1 - head_position.1).signum(),
                            )
                        } else if (head_position.1 - tail_point.1).abs() == 1 {
                            (
                                head_position.0 + (tail_point.0 - head_position.0).signum(),
                                head_position.1,
                            )
                        } else {
                            panic!("Unknown tail position");
                        }
                    }
                };
                tail_visited.insert(tail_position);

                render(&tail_visited, head_position, tail_position);

                (head_position, tail_position)
            },
        );

    tail_visited.len()
}

pub fn a2() -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_first_problem() {
        assert_eq!(a1(), 1818);
    }

    #[test]
    fn should_solve_second_problem() {
        assert_eq!(a2(), 368368);
    }
}
