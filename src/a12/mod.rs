use std::collections::HashMap;
use itertools::Itertools;
use crate::utils::read_lines;

fn read_caves() -> HashMap<String, Vec<String>> {
    let mut caves: HashMap<String, Vec<String>> = HashMap::new();

    for line in read_lines("src/a12/input") {
        let params = line.split("-").collect::<Vec<&str>>();
        let key = String::from(params[0]);
        let path = String::from(params[1]);

        for (key, path) in [(path.clone(), key.clone()), (key, path)] {
            if let Some(paths) = caves.get_mut(&key) {
                paths.push(path);
            } else {
                caves.insert(key, vec![path]);
            };
        }
    }

    caves
}

fn a() -> u64 {
    fn get_paths<'a>(caves: &'a HashMap<String, Vec<String>>, key: &'a str, current_path: &mut Vec<&'a str>, result: &mut u64) {
        if key.to_lowercase().eq(key) && current_path.contains(&key) {
            return;
        }

        let mut current_path = current_path.clone();

        current_path.push(key);

        if String::from(key).eq("end") {
            *result += 1;
        }

        if let Some(children) = caves.get(key) {
            for child in children {
                get_paths(caves, child, current_path.as_mut(), result);
            }
        }
    }


    let caves = read_caves();
    let mut result = 0;

    get_paths(&caves, "start",&mut vec![], &mut result);

    result
}

fn is_lowercase(key: &str) -> bool {
    key.to_lowercase().eq(key)
}

// FIXME: reduce execution time (3s)
fn b() -> u64 {
    fn get_paths<'a>(caves: &'a HashMap<String, Vec<String>>, key: &'a str, current_path: &mut Vec<&'a str>, result: &mut u64) {
        if is_lowercase(key) {
            if key == "start" && current_path.contains(&"start") {
                return;
            }

            let lowercase_keys = current_path
                .iter()
                .map(|&key| key)
                .filter(|key| is_lowercase(&key))
                .collect::<Vec<_>>();
            let is_any_lowercase_cave_visited_twice = lowercase_keys.iter().unique().count() != lowercase_keys.len();

            if is_any_lowercase_cave_visited_twice && lowercase_keys.contains(&key) {
                return;
            }
        }

        let mut current_path = current_path.clone();

        current_path.push(key);

        if String::from(key).eq("end") {
            *result += 1;
            return;
        }

        if let Some(children) = caves.get(key) {
            for child in children {
                get_paths(caves, child, &mut current_path, result);
            }
        }
    }


    let caves = read_caves();
    let mut result = 0;

    get_paths(&caves, "start",&mut vec![], &mut result);

    result
}


pub fn answer() {
    println!("Answer to problem 12: {} {}", a(), b());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_first_problem() {
        assert_eq!(a(), 4304);
    }

    #[test]
    fn should_solve_second_problem() {
        assert_eq!(b(), 118242);
    }

}