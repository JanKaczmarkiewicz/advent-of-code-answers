use std::cmp;
use crate::utils::read_lines;
use std::collections::HashMap;
use std::iter;

pub fn answer() {
    let points = read_lines("src/a5/input")
        .flat_map(|line| {
            let vector = line
                .split(" -> ")
                .flat_map(|cord_raw| cord_raw
                    .split(",")
                    .map(|num_raw| num_raw.parse().unwrap())
                )
                .collect::<Vec<_>>();

            let x1 = vector[0];
            let y1 = vector[1];
            let x2 = vector[2];
            let y2 = vector[3];

            if x1 == x2 {
                let (start, end) = (cmp::min(y1, y2), cmp::max(y1, y2));
                (start..=end).map(move |cord: u32| (x1, cord)).collect::<Vec<_>>()
            } else if y1 == y2 {
                let (start, end) = (cmp::min(x1, x2), cmp::max(x1, x2));
                (start..=end).map(move |cord: u32| (cord, y1)).collect::<Vec<_>>()
            }
            else {
                let vec:Vec<(u32, u32)> = vec![];
                vec
            }
        });

    let mut contacts: HashMap<(u32, u32), u32> = HashMap::new();

    for point in points {
        let new_count = contacts.get(&point).unwrap_or(&0) + 1;

        contacts.insert(point, new_count);
    }

    let sum = contacts
        .iter()
        .map(|(key, value)| value)
        .filter(|&&value| value > 1)
        .count();

    println!("Answer to problem 5: {}", sum);
}