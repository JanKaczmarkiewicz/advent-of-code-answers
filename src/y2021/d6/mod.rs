use crate::utils::read;

fn simulate_lanternfish_population(days: u16) -> u128 {
    let content = read("src/a6/input");
    let fish_iter = content.split(',').map(|x| x.parse::<u128>().unwrap());

    let mut fish = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
    for fish_value in fish_iter {
        fish[fish_value as usize] += 1;
    }

    for _ in 0..days {
        let removed = fish.remove(0);

        fish[6] += removed;
        fish.push(removed);
    }

    fish.iter().sum()
}

pub fn answer() {
    println!(
        "Answer to problem 6: {}, {}",
        simulate_lanternfish_population(80),
        simulate_lanternfish_population(256)
    );
}
