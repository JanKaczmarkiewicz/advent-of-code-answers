use crate::utils::read;

fn a() -> i32 {
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

    let possible_x = (1..=end_x)
        .filter(|x| {
            let mut current = 0;
            let mut velocity = *x;

            while velocity != 0 && current <= end_x {
                if (start_x..=end_x).contains(&current) {
                    return true;
                }
                current += velocity;
                velocity -= 1;
            }
            return false;
        })
        .collect::<Vec<_>>();

    let mut possible_y = (0..(-1 * start_y))
        .filter(|y| {
            let mut current = 0;
            let mut velocity = *y;

            while current >= start_y {
                if (start_y..=end_y).contains(&current) {
                    return true;
                }
                current += velocity;
                velocity -= 1;
            }
            return false;
        })
        .rev();

    let highest_y = possible_y
        .find_map(|y| {
            for x in &possible_x {
                let mut current_x = 0;
                let mut velocity_x = *x;

                let mut current_y = 0;
                let mut velocity_y = y;

                let mut highest_y = current_y;

                while current_y >= start_y && current_x <= end_x {
                    if (start_y..=end_y).contains(&current_y)
                        && (start_x..=end_x).contains(&current_x)
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
        .unwrap();

    highest_y
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
