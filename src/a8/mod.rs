use crate::utils::{read_lines};

fn a() -> u32 {
    read_lines("src/a8/input")
        .map(|line| line
            .split(" | ")
            .nth(1)
            .unwrap()
            .split(" ")
            .map( String::from)
            .collect::<Vec<_>>()
        )
        .fold(0, |prev, curr| {
            let curr_count = curr
                .iter()
                .fold(0, |prev, curr|
                    prev + if curr.len() == 2 || curr.len() == 4 || curr.len() == 7 || curr.len() == 3 { 1 } else { 0 }
                );

            prev + curr_count
        })
}

pub fn answer() {
    println!("Answer to problem 8: {}", a());
}