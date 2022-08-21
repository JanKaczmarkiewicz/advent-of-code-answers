use crate::utils::read_lines;

fn count_one_increase(numbers: &Vec<u16>) -> u16 {
    let mut previous = numbers.first().unwrap();
    let mut increase_count: u16 = 0;

    for current in numbers {
        if current > previous {
            increase_count += 1
        };
        previous = current;
    }

    increase_count
}

fn count_three_increase(numbers: &Vec<u16>) -> u16 {
    let group_count = 3;
    let to = numbers.len() - group_count + 1;
    let sum_numbers: Vec<u16> = (0..to)
        .map(|i| numbers[i..i + group_count].iter().sum())
        .collect();
    count_one_increase(&sum_numbers)
}

pub fn answer() {
    let numbers: Vec<u16> = read_lines("src/a1/input")
        .map(|line| line.parse().unwrap())
        .collect();

    let first_answer = count_one_increase(&numbers);
    let second_answer = count_three_increase(&numbers);

    println!("Answer to problem 1: {}, {}", first_answer, second_answer);
}
