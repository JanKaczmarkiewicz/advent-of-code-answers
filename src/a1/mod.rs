use std::fs::File;
use std::io::{BufRead, BufReader};

fn count_one_increase(numbers: &Vec<u16>) -> u16{
    let mut previous = numbers.first().unwrap();
    let mut increase_count: u16 = 0;

    for current in numbers {
        if current > &previous {
            increase_count += 1
        };
        previous = current;
    }

    increase_count
}

fn count_three_increase(numbers: &Vec<u16>) -> u16 {
    let group_count = 3;
    let to = numbers.len() - group_count + 1;
    let sum_numbers: Vec<u16> = (0..to).map(|i| numbers[i..i+group_count].iter().sum()).collect();
    count_one_increase(&sum_numbers)
}


pub fn answer() {
    let file = File::open("src/a1/input").unwrap();
    let reader = BufReader::new(file);

    let numbers: Vec<u16> = reader.lines().map(
        | raw_num|
            match raw_num {
                Ok(n) => n.parse().unwrap(),
                _ => 0
            }
    ).collect();

    let first_answer = count_one_increase(&numbers);
    let second_answer = count_three_increase(&numbers);

    println!("Answer to problem 1: {}, {}", first_answer, second_answer);
}