use crate::utils::read_lines;

pub fn answer() {
    println!("Answer to day10: {}", a1());
}

enum Command {
    Noop,
    Addx(i32),
}

pub fn a1() -> i32 {
    let instructions = read_lines("src/y2022/d10/input").map(|instruction| {
        let mut chars = instruction.split(" ");

        let command = chars.next().unwrap();
        if command == "addx" {
            Command::Addx(chars.next().unwrap().parse().unwrap())
        } else {
            Command::Noop
        }
    });

    let mut cycle = 0;
    let mut register = 1;

    let mut checks_at = vec![20, 60, 100, 140, 180, 220];

    let mut result = 0;

    for instruction in instructions {
        if checks_at.get(0).is_none() {
            break;
        }

        match instruction {
            Command::Addx(x) => {
                cycle += 2;

                if checks_at[0] <= cycle {
                    result += checks_at.remove(0) * register;
                }

                register += x;
            }
            Command::Noop => {
                cycle += 1;
                if checks_at[0] == cycle {
                    result += checks_at.remove(0) * register;
                }
            }
        }
    }

    result
}

fn draw_pixel(cycle: i32, register: i32) -> () {
    let cursor_x_position = cycle % 40;
    if cursor_x_position == 0 {
        println!();
    }

    let should_light_pixel = (register - 1..=register + 1).contains(&cursor_x_position);

    print!("{}", if should_light_pixel { '#' } else { '.' });
}

pub fn a2() -> () {
    let instructions = read_lines("src/y2022/d10/input").map(|instruction| {
        let mut chars = instruction.split(" ");

        let command = chars.next().unwrap();
        if command == "addx" {
            Command::Addx(chars.next().unwrap().parse().unwrap())
        } else {
            Command::Noop
        }
    });

    // TODO: I can translate instructions into ticks by ("addx" -> Command::Noop, Command::Addx)

    let mut cycle = 0;
    let mut register = 1;

    for instruction in instructions {
        match instruction {
            Command::Addx(x) => {
                // first cycle
                draw_pixel(cycle, register);
                cycle += 1;

                // second cycle
                draw_pixel(cycle, register);
                cycle += 1;

                register += x;
            }
            Command::Noop => {
                draw_pixel(cycle, register);
                cycle += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_first_problem() {
        assert_eq!(a1(), 15140);
    }

    #[test]
    fn should_solve_second_problem() {
        assert_eq!(a2(), ());
    }
}
