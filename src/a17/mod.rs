use crate::utils::read;

struct Config {
    start_x: i32,
    end_x: i32,
    start_y: i32,
    end_y: i32,
}

fn get_config() -> Config {
    let input_raw = read("src/a17/input");

    let result = str::replace(&input_raw, "target area: x=", "");
    let result = str::replace(&result, " y=", "");
    let result = str::replace(&result, "..", ",");

    let mut coordinates = result
        .split(",")
        .map(|num_raw| num_raw.parse::<i32>().unwrap());

    let start_x = coordinates.next().unwrap();
    let end_x = coordinates.next().unwrap();
    let start_y = coordinates.next().unwrap();
    let end_y = coordinates.next().unwrap();

    Config {
        start_x,
        end_x,
        start_y,
        end_y,
    }
}

fn get_possible_x(config: &Config) -> Vec<i32> {
    (1..=config.end_x)
        .filter(|x| {
            let mut current = 0;
            let mut velocity = *x;

            while velocity != 0 && current <= config.end_x {
                if (config.start_x..=config.end_x).contains(&current) {
                    return true;
                }
                current += velocity;
                velocity -= 1;
            }
            return false;
        })
        .collect::<Vec<_>>()
}

fn get_possible_y(config: &Config) -> Vec<i32> {
    (0..(-1 * config.start_y))
        .filter(|y| {
            let mut current = 0;
            let mut velocity = *y;

            while current >= config.start_y {
                if (config.start_y..=config.end_y).contains(&current) {
                    return true;
                }
                current += velocity;
                velocity -= 1;
            }
            return false;
        })
        .rev()
        .collect::<Vec<_>>()
}

fn get_highest_y(config: &Config) -> i32 {
    let possible_x = get_possible_x(config);
    let possible_y = get_possible_y(config);

    possible_y
        .iter()
        .find_map(|y| {
            for x in &possible_x {
                let mut current_x = 0;
                let mut velocity_x = *x;

                let mut current_y = 0;
                let mut velocity_y = *y;

                let mut highest_y = current_y;

                while current_y >= config.start_y && current_x <= config.end_x {
                    if (config.start_y..=config.end_y).contains(&current_y)
                        && (config.start_x..=config.end_x).contains(&current_x)
                    {
                        return Some(highest_y);
                    }
                    current_x += velocity_x;
                    if velocity_x != 0 {
                        velocity_x -= 1;
                    }

                    current_y += velocity_y;
                    velocity_y -= 1;

                    if highest_y < current_y {
                        highest_y = current_y;
                    }
                }
            }
            return None;
        })
        .unwrap()
}

fn a() -> i32 {
    let config = get_config();

    get_highest_y(&config)
}

pub fn answer() {
    println!("Answer to problem 17: {:?}", a());
}

#[cfg(test)]
mod tests {
    use super::a;

    #[test]
    fn should_solve_first_problem() {
        assert_eq!(a(), 6903);
    }
}
