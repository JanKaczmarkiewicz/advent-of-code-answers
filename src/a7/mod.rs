use crate::utils::read;

fn a() -> u128 {
    let mut crabs = read("src/a7/input")
        .split(",")
        .map(|x| x.parse::<i128>().unwrap())
        .collect::<Vec<_>>();
    crabs.sort();

    let median = if  crabs.len() % 2 == 0 {
        (crabs[crabs.len()/2] + crabs[crabs.len()/2-1])  / 2
    } else {
        let center = (crabs.len() / 2) as f64;
        crabs[center.floor() as usize]
    };

    crabs.iter().fold(0, |prev, curr| prev  + (median - curr).abs() as u128)
}

pub fn answer() {
    println!("Answer to problem 7: {}", a());
}