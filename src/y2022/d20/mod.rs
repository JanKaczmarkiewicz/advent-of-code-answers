use crate::utils::read_lines;

pub fn answer() {
    println!("Answer to day20: {} {}", a1(), a2());
}

fn mix(list: &mut Vec<(usize, i64)>) -> () {
    for i in 0..list.len() {
        let index = list
            .iter()
            .position(|(initial_position, _)| *initial_position == i)
            .unwrap();

        let value = list[index].1;

        let destination = { (index as i64 + value).rem_euclid(list.len() as i64 - 1) as usize };

        let item = list.remove(index);

        list.insert(destination, item);
    }
}

fn compute_message(list: Vec<(usize, i64)>) -> i64 {
    let index_of_0 = list.iter().position(|(_, value)| *value == 0).unwrap();

    [1000, 2000, 3000]
        .map(|ith| list[(ith + index_of_0) % list.len()].1)
        .iter()
        .sum()
}

fn a1() -> i64 {
    let mut list = read_lines("src/y2022/d20/input")
        .map(|l| l.parse::<i64>().unwrap())
        .enumerate()
        .collect::<Vec<_>>();

    mix(&mut list);

    compute_message(list)
}

fn a2() -> i64 {
    let mut list = read_lines("src/y2022/d20/input")
        .map(|l| 811589153 * l.parse::<i64>().unwrap())
        .enumerate()
        .collect::<Vec<_>>();

    for _ in 0..10 {
        mix(&mut list)
    }

    compute_message(list)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_solve_first_problem() {
        assert_eq!(a1(), 15297);
    }

    #[test]
    fn should_solve_second_problem() {
        assert_eq!(a2(), 2897373276210);
    }
}
