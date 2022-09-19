use crate::utils::read_lines;

fn a() -> usize {
    let mut times_roled: usize = 0;

    let mut roll_dice = || {
        let r = (times_roled % 100) + 1;
        times_roled += 1;
        r
    };

    let mut players_position: Vec<usize> = read_lines("src/a21/input")
        .map(|l| l.split(": ").nth(1).unwrap().parse::<usize>().unwrap())
        .collect();

    let mut players_score = [0; 2];

    loop {
        for index in [0, 1] {
            let r1 = roll_dice();
            let r2 = roll_dice();
            let r3 = roll_dice();

            let player_position = ((r1 + r2 + r3 + players_position[index] - 1) % 10) + 1;

            players_position[index] = player_position;
            players_score[index] += player_position;

            if players_score.into_iter().max().unwrap() >= 1000 {
                let result = players_score.into_iter().min().unwrap() * times_roled;

                println!("Result: {times_roled}, {:?}", players_score);
                return result;
            }
        }
    }
}

pub fn answer() {
    println!("Answer to problem 21: {}", a());
}

#[cfg(test)]
mod tests {
    use super::a;

    #[test]
    fn should_solve_first_problem() {
        assert_eq!(a(), 506466);
    }
}
