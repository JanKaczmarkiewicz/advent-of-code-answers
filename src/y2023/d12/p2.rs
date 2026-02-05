use itertools::Itertools;

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

            let row_map = (0..5).map(|_| row_map).join(",");
            let contiguous_groups_raw = (0..5).map(|_| contiguous_groups_raw).join(",");

            let contiguous_groups = contiguous_groups_raw
                .split(",")
                .map(|num_raw| num_raw.parse::<u16>().unwrap())
                .collect::<Vec<_>>();

            let mut possibilities = vec!["".to_string()];

            let is_possible_row = |el: &String| {
                if el.len() == row_map.len() {
                    are_vecs_equal(&extract_groups(el), &contiguous_groups)
                } else {
                    let mut counter = 0;
                    let mut last = 0;
                    let mut nr_of_groups = 0;

                    for ch in el.chars() {
                        if ch == '.' {
                            if counter > 0 {
                                last = counter;
                                nr_of_groups += 1;
                                counter = 0;
                            }
                        } else {
                            /* # */
                            counter += 1;
                        }
                    }

                    if counter > 0 {
                        last = counter;
                        nr_of_groups += 1;
                    };

                    nr_of_groups == 0
                        || (nr_of_groups <= contiguous_groups.len()
                            && last <= contiguous_groups[nr_of_groups - 1])
                            && (contiguous_groups[nr_of_groups..].into_iter().sum::<u16>() as usize)
                                < (row_map.len() - el.len())
                }
            };

            // let mut result = 0;
            // // idea: make this recursive instead of keeping possibilities. Why this will help?
            // for c in row_map.chars() {}

            // The information that the row pattern is repeated 5 times can be utilized. I can compute all basic combination that will pass check and then mix and match slices of those.
            // eg ##???#??#?????????#? 11,6
            // say from p1 pow there are 3 possible configurations. This means that in part two there will be at least 3^5 possible final configurations.
            // Not true, since the first pattern can finish with # at the end this will affect the second part of the row, there are not isolated.
            // eg ##########....###### -> ##########....################....######
            // this not only affect 2nd row but also the first: now this is possible:
            // eg                         ##########.......######.################ end continues da da da
            // so I have to include configurations that does finish with # just in case
            // ???????#?? 1,1,2 This illustrates problem well. .......#.. #.##.#.#.# #(etc). This repetition allowes extra patterns in the first part (dependent on the second)

            // so maybe entirely different approach first find place for the biggest group
            // ##???#??#?????????#? 11,6
            // has to be at the start
            // [###########???????#? 11,6]
            // Then second
            // [###########???????#? 11,6]

            // DaC recursive approach

            for c in row_map.chars() {
                if c == '?' {
                    for i in 0..possibilities.len() {
                        let mut cp = possibilities[i].clone();
                        cp.push('.');
                        possibilities.push(cp);
                        possibilities[i].push('#');
                    }
                } else {
                    possibilities.iter_mut().for_each(|el| {
                        el.push(c);
                    });
                }

                println!("{}", possibilities.len());

                //  idea: look at the number of groups
                possibilities.retain(is_possible_row)
            }

            println!("res: {}", possibilities.len());
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

    #[test]
    fn should_compute_solution_dev() {
        assert_eq!(answer(), 0);
    }
}
