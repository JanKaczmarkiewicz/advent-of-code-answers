use itertools::Itertools;

use crate::utils::read_lines;

pub fn answer() {
    println!("Answer to day12: {}", a1());
}

fn a1() -> usize {
    let mut start = (0, 0);
    let mut end = (0, 0);

    let map = read_lines("src/y2022/d12/input")
        .enumerate()
        .map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    'S' => {
                        start = (x, y);
                        'a' as u32
                    }
                    'E' => {
                        end = (x, y);
                        'z' as u32
                    }
                    n => n as u32,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("XD");

    let mut paths_meta = vec![vec![(start, None::<usize>)]];

    loop {
        let possible_moves = paths_meta
            .last()
            .unwrap()
            .iter()
            .enumerate()
            .flat_map(|(i, ((x, y), _))| {
                let mut valid_moves = vec![];

                for pos in
                    [(0, -1), (0, 1), (1, 0), (-1, 0)].map(|m| (m.0 + *x as i32, m.1 + *y as i32))
                {
                    if pos.0 < 0
                        || pos.1 < 0
                        || pos.0 >= map[0].len() as i32
                        || pos.1 >= map.len() as i32
                    {
                        continue;
                    }

                    // terrain
                    if map[pos.1 as usize][pos.0 as usize] as i32 - map[*y][*x] as i32 > 1 {
                        continue;
                    }

                    // already in prev layers
                    if paths_meta.len() > 1
                        && paths_meta[paths_meta.len() - 2]
                            .iter()
                            .any(|el| el.0 == (pos.0 as usize, pos.1 as usize))
                    {
                        continue;
                    }

                    valid_moves.push(((pos.0 as usize, pos.1 as usize), Some(i)))
                }

                valid_moves
            })
            .unique_by(|a| a.0)
            .collect::<Vec<_>>();

        println!("{}", paths_meta.len());

        if possible_moves
            .iter()
            .any(|pos| ((pos.0).0, (pos.0).1) == end)
        {
            return paths_meta.len();
        }

        paths_meta.push(possible_moves);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_solve_first_problem() {
        assert_eq!(a1(), 0);
    }

    #[test]
    fn should_solve_second_problem() {
        // assert_eq!(a2(), 0);
    }
}
