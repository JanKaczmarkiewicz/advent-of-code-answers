mod packets_reader;

use crate::utils::read;
use packets_reader::PacketsReader;

fn hex_char_to_str_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}

fn a() -> u64 {
    let bytes = read("src/a16/input")
        .chars()
        .map(hex_char_to_str_binary)
        .collect::<String>();

    let mut pr = PacketsReader::new(bytes);

    let res = pr.read_all_packages();

    res.iter().map(|pack| pack.0).sum()
}

pub fn answer() {
    println!("Answer to problem 15: {}", a());
}

#[cfg(test)]
mod tests {
    use super::a;

    #[test]
    fn should_solve_first_problem() {
        assert_eq!(a(), 908);
    }
}
