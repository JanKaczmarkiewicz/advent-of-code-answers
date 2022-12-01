use itertools::Itertools;

use crate::utils::read_lines;

fn get_elves_load() -> impl Iterator<Item = u32> {
    read_lines("src/y2022/d1/input").batching(|it| {
        match it.map_while(|line| line.parse::<u32>().ok()).sum::<u32>() {
            0 => None,
            sum => Some(sum),
        }
    })
}

pub fn answer() {
    a1();
    a2();
}

pub fn a1() {
    println!("a1: {}", get_elves_load().max().unwrap());
}

pub fn a2() {
    println!(
        "a2: {}",
        get_elves_load().sorted().rev().take(3).sum::<u32>()
    );
}
