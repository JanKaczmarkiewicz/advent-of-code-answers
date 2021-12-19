use crate::utils::{read_lines};
use itertools::Itertools;

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

fn words_difference(left_word: &String, right_word: &String) -> u8 {
    right_word.chars().fold(0, |sum, curr| {
        sum + if left_word.contains(curr) { 0 } else { 1 }
    })
}

fn b() -> u32 {
    read_lines("src/a8/input")
        .map(|line| line
            .split(" | ")
            .flat_map(|subline| subline.split(" "))
            .map(String::from)
            .collect::<Vec<_>>()
        ).fold(0, |acc, line| {
            let mut found_numbers: Vec<Option<&String>> = vec![None, None, None, None, None, None, None, None, None, None];
            for word in &line {
                let index_to_write_word = match word.len() {
                    2 => 1,
                    4 => 4,
                    3 => 7,
                    7 => 8,
                    _ => -1,
                };

                if index_to_write_word > -1 {
                    found_numbers[index_to_write_word as usize] = Some(word);
                }
            }

            for word in &line {
                let index_to_write_word: i32 = match word.len() {
                    6 => {
                        if found_numbers[4] != None && words_difference(found_numbers[4].unwrap(), &word) == 2 {
                            9
                        }
                        else if found_numbers[1] != None && words_difference(found_numbers[1].unwrap(), &word) == 5 {
                            6
                        }
                        else {
                            0
                        }
                    }
                    5 => {
                        if found_numbers[4] != None && words_difference(found_numbers[4].unwrap(), &word) == 3 {
                            2
                        }
                        else if found_numbers[1] != None && words_difference(found_numbers[1].unwrap(), &word) == 3 {
                            3
                        } else {
                            5
                        }
                    }
                    _ => -1
                };

                if index_to_write_word > -1 {
                    found_numbers[index_to_write_word as usize] = Some(word);
                }
            }

            let sum = (&line[10..14])
                .into_iter()
                .rev()
                .enumerate()
                .fold(0_u32,|prev, (i, curr)|{
                    prev + 10_u32.pow(i as u32) * found_numbers
                        .iter()
                        .position(|&number|
                            {
                                let left =  number
                                    .unwrap()
                                    .chars()
                                    .sorted()
                                    .collect::<String>();

                                let right =  curr
                                    .chars()
                                    .sorted()
                                    .collect::<String>();

                                right.eq(&left)
                            }
                        )
                        .unwrap() as u32
                });

            acc + sum
        })
}

pub fn answer() {
    println!("Answer to problem 8: {}, {}", a(), b());
}