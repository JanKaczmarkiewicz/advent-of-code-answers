use crate::utils::read;
use std::num::ParseIntError;

fn str_binary_to_number(text: &str) -> Result<i32, ParseIntError> {
    i32::from_str_radix(text, 2)
}

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

fn a() -> i32 {
    let bytes = read("src/a16/input")
        .chars()
        .map(hex_char_to_str_binary)
        .collect::<String>();

    let mut offset: usize = 0;
    let mut result = vec![];

    let mut read_next_bytes = |amount: usize| {
        let fragment = &bytes[offset..offset + amount];
        offset += amount;
        return fragment;
    };

    const VERSION_SIZE: usize = 3;
    const TYPE_ID_SIZE: usize = 3;
    const LITERAL_FRAME_SIZE: usize = 5;
    const LITERAL_FRAME_MULTIPLE_SIZE: usize = 4;

    let version_id = str_binary_to_number(read_next_bytes(VERSION_SIZE)).unwrap();
    let type_id = str_binary_to_number(read_next_bytes(TYPE_ID_SIZE)).unwrap();

    match type_id {
        0 | 1 => {}
        4 => {
            let mut values = vec![];

            loop {
                let frame = read_next_bytes(LITERAL_FRAME_SIZE);
                values.push(&frame[1..]);
                if frame.starts_with("0") {
                    break;
                }
            }

            // literal data frame can contain empty bits at the end (LITERAL_FRAME_MULTIPLE_SIZE alignment)
            let read_bits = values.len() * LITERAL_FRAME_SIZE + VERSION_SIZE + TYPE_ID_SIZE;
            let align = LITERAL_FRAME_MULTIPLE_SIZE - (read_bits % LITERAL_FRAME_MULTIPLE_SIZE);
            read_next_bytes(align);

            result.push((
                version_id,
                type_id,
                str_binary_to_number(&values.join("")).unwrap(),
            ));
        }
        n => panic!("Unsupported type {}", n),
    };

    0
}

pub fn answer() {
    println!("Answer to problem 15: {}", a());
}

#[cfg(test)]
mod tests {
    use super::a;

    #[test]
    fn should_solve_first_problem() {
        assert_eq!(a(), 592);
    }
}
