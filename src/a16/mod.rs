use crate::utils::read;
use std::cell::Cell;

fn str_binary_to_number(text: &str) -> i32 {
    i32::from_str_radix(text, 2).unwrap()
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

const VERSION_SIZE: usize = 3;
const TYPE_ID_SIZE: usize = 3;
const LITERAL_FRAME_SIZE: usize = 5;
const OPERATION_LENGTH_ID_SIZE: usize = 1;
const OPERATION_LENGTH_SIZE: usize = 15;

fn a() -> i32 {
    let bytes = read("src/a16/input")
        .chars()
        .map(hex_char_to_str_binary)
        .collect::<String>();

    let offset = Cell::new(0);
    let mut result = vec![];

    let read_next_bytes = |amount: usize| {
        let offset_value = offset.get();
        let offset_next_value = offset_value + amount;
        let fragment = &bytes[offset_value..offset_value + offset_next_value];
        offset.set(offset_next_value);
        return fragment;
    };

    let read_header = || {
        let version_id = str_binary_to_number(read_next_bytes(VERSION_SIZE));
        let type_id = str_binary_to_number(read_next_bytes(TYPE_ID_SIZE));

        return (version_id, type_id);
    };

    let read_zeros = || loop {
        let offset_value = offset.get();
        let next_character = &bytes[offset_value..1 + offset_value];

        if next_character == "0" {
            read_next_bytes(1);
        } else {
            break;
        }
    };

    let x = 10;
    let y = &mut x;
    let z = &mut x;

    print!("{}, {}, {}", y, z, x);

    let read_literal = || {
        let mut values = vec![];

        loop {
            let frame = read_next_bytes(LITERAL_FRAME_SIZE);
            values.push(&frame[1..]);
            if frame.starts_with("0") {
                break;
            }
        }

        str_binary_to_number(&values.join(""));
    };

    let mut read_package = || {
        let (version_id, type_id) = read_header();

        match type_id {
            4 => {
                result.push((version_id, type_id, read_literal()));
            }
            _ => {
                let length_type_id = read_next_bytes(OPERATION_LENGTH_ID_SIZE);

                match length_type_id {
                    "0" => {}
                    "1" => {
                        let length = str_binary_to_number(read_next_bytes(OPERATION_LENGTH_SIZE));
                        let target_length = offset.get() + length as usize;

                        while offset.get() != target_length {
                            read_package();
                        }
                    }
                    _ => {}
                }
            }
        };
        read_zeros();
    };

    while bytes.len() != offset.get() {
        read_package();
    }

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
