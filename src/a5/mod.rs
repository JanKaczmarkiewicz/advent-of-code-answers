use std::cmp;
use crate::utils::read_lines;
use std::collections::HashMap;

pub fn count_overlaps(point_criteria: fn ((i32, i32, i32, i32)) -> Vec<(i32, i32)>) -> usize {
    let points = read_lines("src/a5/input")
        .flat_map(|line| {
            let vector = line
                .split(" -> ")
                .flat_map(|cord_raw| cord_raw
                    .split(",")
                    .map(|num_raw| num_raw.parse::<i32>().unwrap())
                )
                .collect::<Vec<_>>();


            return point_criteria((vector[0], vector[1], vector[2], vector[3]));
        });

    let mut contacts: HashMap<(i32, i32), i32> = HashMap::new();

    for point in points {
        let new_count = contacts.get(&point).unwrap_or(&0) + 1;

        contacts.insert(point, new_count);
    }

    contacts
        .iter()
        .map(|(_key, value)| value)
        .filter(|&&value| value > 1)
        .count()
}


pub fn answer() {
    let answer_a = count_overlaps(|(x1, y1, x2, y2)| {
        if x1 == x2 { // horizontal
            let (start, end) = (cmp::min(y1, y2), cmp::max(y1, y2));
            (start..=end).map(move |cord| (x1, cord)).collect::<Vec<_>>()
        }
        else if y1 == y2 { // vertical
            let (start, end) = (cmp::min(x1, x2), cmp::max(x1, x2));
            (start..=end).map(move |cord| (cord, y1)).collect::<Vec<_>>()
        }
        else {
            let vec:Vec<(i32, i32)> = vec![];
            vec
        }
    });

    let answer_b = count_overlaps(|(x1, y1, x2, y2)| {
        if x1 == x2 { // horizontal
            let (start, end) = (cmp::min(y1, y2), cmp::max(y1, y2));
            (start..=end).map(move |cord| (x1, cord)).collect::<Vec<_>>()
        }
        else if y1 == y2 { // vertical
            let (start, end) = (cmp::min(x1, x2), cmp::max(x1, x2));
            (start..=end).map(move |cord| (cord, y1)).collect::<Vec<_>>()
        }
        else if x1 - y1 == x2 - y2 { // diagonal down
            let min_x = cmp::min(x1, x2);
            let min_y = cmp::min(y1, y2);
            let end = (x1 - x2).abs();
            (0..=end).map(move |i| (min_x + i, min_y + i)).collect::<Vec<_>>()
        }
        else if x1 - x2 == y2 - y1 { // diagonal up
            let min_x = cmp::min(x1, x2);
            let max_y = cmp::max(y1, y2);
            let end = (x1 - x2).abs();
            (0..=end).map(move |i| (min_x + i, max_y - i)).collect::<Vec<_>>()
        }
        else {
            let vec:Vec<(i32, i32)> = vec![];
            vec
        }
    });

    println!("Answer to problem 5: {} {}", answer_a, answer_b);
}