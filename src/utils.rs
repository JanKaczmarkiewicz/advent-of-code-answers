use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::iter::Map;

pub(crate) fn read_lines(path: &str) -> Map<Lines<BufReader<File>>, fn(std::io::Result<String>) -> String> {
    let file = File::open(path).unwrap();
    BufReader::new(file).lines().map(|line| line.unwrap().to_owned())
}

pub(crate) fn read(path: &str) -> String {
    return std::fs::read_to_string(path).unwrap();
}