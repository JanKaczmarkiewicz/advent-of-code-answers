use std::{collections::HashMap, ops::RangeFrom};

use itertools::Itertools;

use crate::utils::read_lines;

fn substr(s: &str, r: RangeFrom<usize>) -> &str {
    if r.start >= s.len() {
        return "";
    } else {
        &s[r]
    }
}

struct CachedCount<'a> {
    cache: HashMap<(Option<char>, &'a str, &'a [usize]), usize>,
}

impl<'a> CachedCount<'a> {
    fn count(&mut self, curr: Option<char>, row: &'a str, cfg: &'a [usize]) -> usize {
        // println!("{} {row} {cfg:?}", curr.unwrap_or(' '));
        if let Some(c) = curr {
            if c == '.' {
                if row.len() == 0 {
                    self.cached_count(None, "", cfg)
                } else {
                    self.cached_count(row.chars().nth(0), &row[1..], cfg)
                }
            } else if c == '#' {
                if cfg.len() == 0 {
                    0
                } else if row.len() < cfg[0] - 1 {
                    0
                } else if row[..cfg[0] - 1].contains(".") {
                    0
                } else if row
                    .chars()
                    .nth(cfg[0] - 1)
                    .map(|nc| nc == '#')
                    .unwrap_or(false)
                {
                    0
                } else {
                    // this means that the next one is either ? or . or <end>
                    self.cached_count(
                        row.chars().nth(cfg[0]),
                        substr(row, cfg[0] + 1..),
                        &cfg[1..],
                    )
                }
            } else {
                self.cached_count(Some('#'), row, cfg) + self.cached_count(Some('.'), row, cfg)
            }
        } else {
            if cfg.is_empty() {
                1
            } else {
                0
            }
        }
    }

    fn cached_count(&mut self, curr: Option<char>, row: &'a str, cfg: &'a [usize]) -> usize {
        let key = (curr, row, cfg);
        if let Some(result) = self.cache.get(&key) {
            return *result;
        } else {
            let result = self.count(curr, row, cfg);
            self.cache.insert(key, result);
            return result;
        }
    }
}

pub fn answer() -> usize {
    read_lines("src/y2023/d12/input")
        .map(|line| {
            let (row_map, contiguous_groups_raw) = line.split_once(" ").unwrap();

            let row_map = (0..5).map(|_| row_map).join("?");
            let contiguous_groups_raw = (0..5).map(|_| contiguous_groups_raw).join(",");

            let contiguous_groups = contiguous_groups_raw
                .split(",")
                .map(|num_raw| num_raw.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            let mut cache = CachedCount {
                cache: HashMap::new(),
            };

            let result =
                cache.cached_count(row_map.chars().nth(0), &row_map[1..], &contiguous_groups);

            println!("{result}");
            result
        })
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_compute_solution() {
        assert_eq!(answer(), 51456609952403);
    }

    #[test]
    fn should_compute_solution_dev() {
        assert_eq!(answer(), 0);
    }
}
