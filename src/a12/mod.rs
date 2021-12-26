use std::collections::HashMap;
use crate::utils::read_lines;


fn get_paths<'a>(caves: &'a HashMap<String, Vec<String>>, key: &'a str, current_path: &mut Vec<&'a str>, result: &mut Vec<Vec<&'a str>>) {
    if key.to_lowercase().eq(key) && current_path.contains(&key) {
        return;
    }

    let mut current_path = current_path.clone();

    current_path.push(key);

    if String::from(key).eq("end") {
        return result.push(current_path.clone());
    }

    if let Some(children) = caves.get(key) {
        for child in children {
            get_paths(caves, child, current_path.as_mut(), result);
        }
    }
}

fn a() -> usize {
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

    let mut result = vec![];
    get_paths(&caves, "start",&mut vec![], &mut result);

    result.len()
}

pub fn answer() {
    println!("Answer to problem 12: {}", a());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_first_problem() {
        assert_eq!(a(), 4304);
    }

}