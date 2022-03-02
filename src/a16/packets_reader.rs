use std::cell::Cell;

const VERSION_SIZE: usize = 3;
const TYPE_ID_SIZE: usize = 3;
const LITERAL_FRAME_SIZE: usize = 5;
const OPERATION_LENGTH_ID_SIZE: usize = 1;
const OPERATION_LENGTH_SIZE: usize = 15;
const SUBPACKETS_NUMBER_SIZE: usize = 11;

fn str_binary_to_number(text: &str) -> u64 {
    u64::from_str_radix(text, 2).unwrap()
}

pub struct PacketsReader {
    offset: Cell<usize>,
    bytes: String,
    result: Vec<(u64, u64, u64)>,
}

impl PacketsReader {
    pub fn new(bytes: String) -> Self {
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

    fn read_header(&self) -> (u64, u64) {
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

    fn read_literal(&self) -> u64 {
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
                    "1" => {
                        let length =
                            str_binary_to_number(self.read_next_bytes(SUBPACKETS_NUMBER_SIZE));

                        for _ in 0..length {
                            self.read_next_package();
                        }
                    }
                    "0" => {
                        let length =
                            str_binary_to_number(self.read_next_bytes(OPERATION_LENGTH_SIZE));
                        let target_length = self.offset.get() + length as usize;

                        while self.offset.get() != target_length {
                            self.read_next_package();
                        }
                    }
                    _ => {
                        panic!("Not possible")
                    }
                }

                self.result.push((version_id, type_id, 0));
            }
        };
    }

    pub fn read_all_packages(&mut self) -> &Vec<(u64, u64, u64)> {
        while self.bytes.len() != self.offset.get() {
            self.read_next_package();
            self.read_zeros();
        }
        return &self.result;
    }
}
