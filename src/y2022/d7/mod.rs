use std::collections::HashMap;

use crate::utils::read;
use itertools::{self, Itertools};

pub fn answer() {
    println!("Answer to day7: {} {}", a1(), a2());
}

fn calculate_dir_size(dirs: &HashMap<String, Vec<FileOrDirectory>>, dir: &str) -> usize {
    dirs.get(dir)
        .unwrap()
        .iter()
        .map(|file_or_dir| match file_or_dir {
            FileOrDirectory::Directory { name } => calculate_dir_size(dirs, name),
            FileOrDirectory::File { size, .. } => *size,
        })
        .sum()
}

#[derive(Debug, PartialEq)]
enum FileOrDirectory {
    File { size: usize },
    Directory { name: String },
}

pub fn get_second_word(text: &str) -> &str {
    text.split(" ").collect_vec()[1].trim()
}

fn parse_file_structure(commands_output: &str) -> HashMap<String, Vec<FileOrDirectory>> {
    let mut current_dir = "/".to_string();

    let commands = commands_output.split("$ ").collect_vec();
    let mut dirs = HashMap::new();

    dirs.insert(current_dir.to_string(), vec![]);

    commands.iter().for_each(|command| {
        if command.starts_with("ls") {
            let mut output = command.lines();
            output.next();

            output.for_each(|line| {
                let file_or_dir = if line.starts_with("dir") {
                    let name = get_second_word(line).to_string();

                    let name_with_parrent_suffix = format!("{}/{}", current_dir, name);

                    assert_eq!(dirs.insert(name_with_parrent_suffix.clone(), vec![]), None);

                    FileOrDirectory::Directory {
                        name: name_with_parrent_suffix,
                    }
                } else {
                    let result = line.split(" ").collect_vec();
                    let size = result[0].parse().unwrap();

                    FileOrDirectory::File { size }
                };

                dirs.get_mut(&current_dir)
                    .as_mut()
                    .unwrap()
                    .push(file_or_dir);
            })
        } else if command.starts_with("cd") {
            let name = get_second_word(command);

            if name == current_dir {
                return;
            } else if name == ".." {
                for (key, value) in dirs.iter() {
                    if let Some(_dir) = value.iter().find(|file_or_dir| match file_or_dir {
                        FileOrDirectory::Directory { name } => *name == current_dir,
                        FileOrDirectory::File { .. } => false,
                    }) {
                        current_dir = key.to_owned();
                        break;
                    }
                }
            } else {
                current_dir = format!("{}/{}", current_dir, name);
            }
        }
    });

    dirs
}

pub fn a1() -> usize {
    let raw_commands = read("src/y2022/d7/input");

    let dirs = parse_file_structure(&raw_commands);

    dirs.keys()
        .map(|key| {
            let size = calculate_dir_size(&dirs, key);
            size
        })
        .filter(|size| *size <= 100000)
        .sum()
}

pub fn a2() -> usize {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_first_problem() {
        assert_eq!(a1(), 1086293);
    }

    #[test]
    fn should_solve_second_problem() {
        assert_eq!(a2(), 100);
    }
}
