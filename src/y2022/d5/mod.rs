use crate::utils::read;

pub fn answer() {
    println!("Answer to day5: {} {}", a1(), a2());
}

struct Instriction {
    from: usize,
    to: usize,
    number: usize,
}

fn parse_instriction(raw: &str) -> Instriction {
    let res = raw.split(" ").collect::<Vec<_>>();

    Instriction {
        number: res[1].parse().unwrap(),
        from: res[3].parse::<usize>().unwrap() - 1,
        to: res[5].parse::<usize>().unwrap() - 1,
    }
}

fn parse_columns(raw: &str) -> Vec<Vec<char>> {
    let number_of_columns = (raw.lines().next().unwrap().chars().count() + 1) / 4;

    (0..number_of_columns)
        .map(|i| {
            let index = 4 * i + 1;
            raw.lines()
                .filter_map(|line| {
                    let char = line.chars().nth(index).unwrap();
                    char.is_alphabetic().then_some(char)
                })
                .rev()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn a1() -> String {
    let binding = read("src/y2022/d5/input");

    let mut iter = binding.split("\n\n");

    let state_raw = iter.next().unwrap();

    let mut columns = parse_columns(state_raw);

    let instructions = iter.next().unwrap().lines().map(parse_instriction);

    instructions.for_each(|Instriction { from, to, number }| {
        let col_len = columns[from].len();       
        let items_to_move = columns[from].drain(col_len-number..col_len).rev().collect::<Vec<_>>();
        columns[to].extend(items_to_move);
    });

    columns
        .iter()
        .map(|column| column.last().unwrap())
        .collect::<String>()
}

pub fn a2() -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_first_problem() {
        assert_eq!(a1(), "ZRLJGSCTR");
    }

    #[test]
    fn should_solve_second_problem() {}
}
