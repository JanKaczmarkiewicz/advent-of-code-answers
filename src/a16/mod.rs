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

struct PacketsReader {
    offset: Cell<usize>,
    bytes: String,
    result: Vec<(i32, i32, i32)>,
}

impl PacketsReader {
    fn new(bytes: String) -> Self {
        Self {
            offset: Cell::new(0),
            bytes,
            result: vec![],
        }
    }

    fn read_next_bytes(&self, amount: usize) -> &str {
        let offset_value = self.offset.get();
        let offset_next_value = offset_value + amount;
        let fragment = &self.bytes[offset_value..offset_next_value];
        self.offset.set(offset_next_value);
        return fragment;
    }

    fn read_header(&self) -> (i32, i32) {
        let version_id_raw = self.read_next_bytes(VERSION_SIZE);
        let version_id = str_binary_to_number(version_id_raw);

        let type_id_raw = self.read_next_bytes(TYPE_ID_SIZE);
        let type_id = str_binary_to_number(type_id_raw);

        return (version_id, type_id);
    }

    fn read_zeros(&self) {
        loop {
            let offset_value = self.offset.get();
            let next_character = self.bytes.get(offset_value..1 + offset_value);

            if let Some("0") = next_character {
                self.read_next_bytes(1);
            } else {
                break;
            }
        }
    }

    fn read_literal(&self) -> i32 {
        let mut values = vec![];

        loop {
            let frame = self.read_next_bytes(LITERAL_FRAME_SIZE);
            values.push(&frame[1..]);
            if frame.starts_with("0") {
                break;
            }
        }

        str_binary_to_number(&values.join(""))
    }

    fn read_next_package(&mut self) {
        let (version_id, type_id) = self.read_header();

        match type_id {
            4 => {
                self.result.push((version_id, type_id, self.read_literal()));
            }
            _ => {
                let length_type_id = self.read_next_bytes(OPERATION_LENGTH_ID_SIZE);

                match length_type_id {
                    "1" => {}
                    "0" => {
                        let length =
                            str_binary_to_number(self.read_next_bytes(OPERATION_LENGTH_SIZE));
                        let target_length = self.offset.get() + length as usize;

                        while self.offset.get() != target_length {
                            self.read_next_package();
                        }
                    }
                    _ => {}
                }
            }
        };
    }

    pub fn read_all_packages(&mut self) -> &Vec<(i32, i32, i32)> {
        while self.bytes.len() != self.offset.get() {
            self.read_next_package();
            self.read_zeros();
        }
        return &self.result;
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

    let mut packets_reader = PacketsReader::new(bytes);

    let res = packets_reader.read_all_packages();

    res.len() as i32
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
