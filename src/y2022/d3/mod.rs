use std::collections::HashSet;

use itertools::Itertools;

use crate::utils::read_lines;

pub fn answer() {
    a1();
    a2();
}

pub fn a1() {
    let res = read_lines("src/y2022/d3/input")
        .map(|line| {
            let l = &line[0..line.len() / 2];
            let r = &line[line.len() / 2..line.len()];

            let left_content = HashSet::<_>::from_iter(l.chars());
            let right_content = HashSet::from_iter(r.chars());

            let _union: Vec<_> = left_content.intersection(&right_content).collect();

            left_content
                .intersection(&right_content)
                .map(|c| (*c as u8 - if c.is_lowercase() { 96 } else { 65 - 27 }) as u32)
                .sum::<u32>()
        })
        .sum::<u32>();

    println!("a1: {}", res);
}

pub fn a2() {
    let res = read_lines("src/y2022/d3/input")
        .chunks(3)
        .into_iter()
        .map(|group| {
            group
                .map(|gnome| HashSet::<_>::from_iter(gnome.chars()))
                .reduce(|accum, item| accum.intersection(&item).copied().collect::<HashSet<_>>())
                .unwrap()
                .iter()
                .map(|c| (*c as u8 - if c.is_lowercase() { 96 } else { 65 - 27 }) as u32)
                .sum::<u32>()
        })
        .sum::<u32>();

    println!("a2: {}", res);
}
