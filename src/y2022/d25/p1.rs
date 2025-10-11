use crate::utils::read_lines;

pub fn answer() -> String {
    let total_sum = read_lines("src/y2022/d25/input")
        .map(|line| {
            line.chars()
                .rev()
                .enumerate()
                .map(|(i, c)| {
                    (match c {
                        '=' => -2,
                        '-' => -1,
                        '1' => 1,
                        '2' => 2,
                        '0' => 0,

                        n => panic!("Other: {n}"),
                    }) * 5_i32.pow(i as u32)
                })
                .sum::<i32>()
        })
        .sum::<i32>();

    return to_snafu(total_sum);
}

fn to_snafu(mut nr: i32) -> String {
    let mut result = vec![];
    while nr != 0 {
        let mut n = 0;

        'outer: loop {
            for a in [1 * nr.signum(), 2 * nr.signum()] {
                let num = a * 5_i32.pow(n);
                if (nr as f32 / num as f32).round() == 1.0 {
                    if result.capacity() == 0 {
                        result = vec!["0"; n as usize + 1];
                    }

                    result[n as usize] = match a {
                        1 => "1",
                        2 => "2",
                        -2 => "=",
                        -1 => "-",
                        _ => panic!(),
                    };
                    nr -= num;
                    break 'outer;
                }
            }

            n += 1;
        }
    }

    result.reverse();
    return result.join("");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_compute_solution() {
        assert_eq!(answer(), "2=-1=0".to_string());
    }
}
