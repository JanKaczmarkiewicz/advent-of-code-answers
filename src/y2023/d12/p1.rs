use crate::utils::read_lines;

fn extract_groups(row: &str) -> Vec<u16> {
    let mut counter = 0;

    let mut row_groups = vec![];

    for ch in row.chars() {
        if ch == '.' {
            if counter > 0 {
                row_groups.push(counter);
                counter = 0;
            }
        } else {
            /* # */
            counter += 1;
        }
    }

    if counter > 0 {
        row_groups.push(counter);
    }

    return row_groups;
}

fn are_vecs_equal<T: PartialEq<T>>(l: &Vec<T>, r: &Vec<T>) -> bool {
    l.len() == r.len() && l.iter().zip(r.iter()).all(|(l, r)| l == r)
}

pub fn answer() -> usize {
    read_lines("src/y2023/d12/input")
        .map(|line| {
            let (row_map, contiguous_groups_raw) = line.split_once(" ").unwrap();

            let contiguous_groups = contiguous_groups_raw
                .split(",")
                .map(|num_raw| num_raw.parse::<u16>().unwrap())
                .collect::<Vec<_>>();

            let mut possibilities = vec!["".to_string()];

            for c in row_map.chars() {
                if c == '?' {
                    possibilities = possibilities
                        .iter()
                        .flat_map(|el| {
                            [format!("{}.", el.clone()), format!("{}#", el.clone())].into_iter()
                        })
                        .collect()
                } else {
                    possibilities.iter_mut().for_each(|el| {
                        el.push(c);
                    });
                }

                possibilities.retain(|el| {
                    let el_groups = extract_groups(el);

                    if el.len() == row_map.len() {
                        are_vecs_equal(&el_groups, &contiguous_groups)
                    } else {
                        if el_groups.len() == 0 {
                            return true;
                        } else {
                            el_groups.len() <= contiguous_groups.len()
                                && el_groups[el_groups.len() - 1]
                                    <= contiguous_groups[el_groups.len() - 1]
                        }
                    }
                })
            }

            possibilities.len()
        })
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_compute_solution() {
        assert_eq!(answer(), 7163);
    }
}
