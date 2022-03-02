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
}

impl PacketsReader {
    pub fn new(bytes: String) -> Self {
        Self {
            offset: Cell::new(0),
            bytes,
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

    fn read_next_package(&mut self) -> (u64, u64) {
        let (version_id, type_id) = self.read_header();

        if 4 == type_id {
            return (version_id, self.read_literal());
        }

        let length_type_id = self.read_next_bytes(OPERATION_LENGTH_ID_SIZE);

        let mut sub_packages = vec![];

        match length_type_id {
            "1" => {
                let length = str_binary_to_number(self.read_next_bytes(SUBPACKETS_NUMBER_SIZE));

                for _ in 0..length {
                    sub_packages.push(self.read_next_package());
                }
            }
            "0" => {
                let length = str_binary_to_number(self.read_next_bytes(OPERATION_LENGTH_SIZE));
                let target_length = self.offset.get() + length as usize;

                while self.offset.get() != target_length {
                    sub_packages.push(self.read_next_package());
                }
            }
            _ => {
                panic!("Not possible")
            }
        };

        let sub_packages_values = sub_packages
            .iter()
            .map(|(_, value)| *value)
            .collect::<Vec<_>>();
        let sub_packages_versions_sum: u64 = sub_packages.iter().map(|(version, _)| *version).sum();

        let value = match type_id {
            0 => sub_packages_values.iter().sum(),
            1 => sub_packages_values.iter().fold(1, |acc, curr| acc * curr),
            2 => sub_packages_values.iter().min().unwrap().to_owned(),
            3 => sub_packages_values.iter().max().unwrap().to_owned(),
            5 => {
                if sub_packages_values[0] > sub_packages_values[1] {
                    1
                } else {
                    0
                }
            }
            6 => {
                if sub_packages_values[1] > sub_packages_values[0] {
                    1
                } else {
                    0
                }
            }
            7 => {
                if sub_packages_values[1] == sub_packages_values[0] {
                    1
                } else {
                    0
                }
            }
            _ => panic!("Operation not possible"),
        };

        (sub_packages_versions_sum + version_id, value)
    }

    pub fn read_all_packages(&mut self) -> (u64, u64) {
        let mut sum = 0;
        let mut version_sum = 0;
        while self.bytes.len() != self.offset.get() {
            let (version, value) = self.read_next_package();
            version_sum += version;
            sum += value;
            self.read_zeros();
        }
        return (version_sum, sum);
    }
}
