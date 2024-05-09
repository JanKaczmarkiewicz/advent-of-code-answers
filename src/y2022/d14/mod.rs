use itertools::Itertools;

use crate::utils::read_lines;

pub fn answer() {
    println!("Answer to day14: {} {}", a1(), a2());
}

fn a1() -> usize {
    let mut lines = vec![];

    read_lines("src/y2022/d14/input").for_each(|line| {
        line.split(" -> ")
            .map(|cords_raw| {
                let mut cords_iter = cords_raw.split(",");
                (
                    cords_iter.next().unwrap().parse::<i32>().unwrap(),
                    cords_iter.next().unwrap().parse::<i32>().unwrap(),
                )
            })
            .reduce(|acc, curr| {
                lines.push((acc, curr));
                curr
            });
    });

    let all_points = lines.iter().map(|i| i.0).chain(lines.iter().map(|i| i.1));

    let max_y = all_points.clone().map(|i| i.1).max().unwrap();

    let (min_x, max_x) = all_points.map(|i| i.0).minmax().into_option().unwrap();

    let map_width = max_x - min_x;

    let mut map = (0..=max_y)
        .map(|_| (0..=map_width).map(|_| '.').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let cords_to_map_indexes = |pos: &(i32, i32)| ((pos.0 - min_x) as usize, pos.1 as usize);

    for (start, end) in lines
        .iter()
        .map(|(s, e)| (cords_to_map_indexes(s), cords_to_map_indexes(e)))
    {
        let start_x = start.0.min(end.0);
        let end_x = start.0.max(end.0);
        let start_y = start.1.min(end.1);
        let end_y = start.1.max(end.1);

        for x in start_x..=end_x {
            for y in start_y..=end_y {
                map[y][x] = '#';
            }
        }
    }

    let mut sand_pieces: Vec<Option<(usize, usize)>> = vec![];

    loop {
        for option in sand_pieces.iter_mut() {
            if let Some((x, y)) = option {
                let is_in_map = |(_, y): (i32, i32)| y < map.len() as i32 && y >= 0;

                let is_collision = |(x, y)| {
                    let row: &Vec<_> = &map[y];
                    if let Some(&state) = row.get(x) {
                        state == '#' || state == 'o'
                    } else {
                        false
                    }
                };

                // down
                {
                    let next = (*x as i32, *y as i32 + 1);
                    if !is_in_map(next) {
                        return map
                            .iter()
                            .map(|row| row.iter().filter(|c| **c == 'o').count())
                            .sum();
                    }

                    let next = (next.0 as usize, next.1 as usize);
                    if !is_collision(next) {
                        *option = Some(next);
                        continue;
                    }
                }

                // left
                {
                    let next = (*x as i32 - 1, *y as i32 + 1);
                    if !is_in_map(next) {
                        return map
                            .iter()
                            .map(|row| row.iter().filter(|c| **c == 'o').count())
                            .sum();
                    }

                    let next = (next.0 as usize, next.1 as usize);
                    if !is_collision(next) {
                        *option = Some(next);
                        continue;
                    }
                }

                // right
                {
                    let next = (*x as i32 + 1, *y as i32 + 1);
                    if !is_in_map(next) {
                        return map
                            .iter()
                            .map(|row| row.iter().filter(|c| **c == 'o').count())
                            .sum();
                    }

                    let next = (next.0 as usize, next.1 as usize);
                    if !is_collision(next) {
                        *option = Some(next);
                        continue;
                    }
                }

                map[*y][*x] = 'o';
                *option = None
            }
        }

        sand_pieces.push(Some(cords_to_map_indexes(&(500, 0))))
    }
}

const SPAWN_POSITION: (i32, i32) = (500, 0);

fn a2() -> usize {
    let mut lines: Vec<((i32, i32), (i32, i32))> = vec![];

    read_lines("src/y2022/d14/input").for_each(|line| {
        line.split(" -> ")
            .map(|cords_raw| {
                let mut cords_iter = cords_raw.split(",");
                (
                    cords_iter.next().unwrap().parse::<i32>().unwrap(),
                    cords_iter.next().unwrap().parse::<i32>().unwrap(),
                )
            })
            .reduce(|acc, curr| {
                lines.push((acc, curr));
                curr
            });
    });
    let all_points = lines.iter().map(|i| i.0).chain(lines.iter().map(|i| i.1));

    let max_y = 2 + all_points.clone().map(|i| i.1).max().unwrap();

    let pyramid_side_width = max_y; // - 1 removed because last sand can still check for side moves

    let min_x_reqiured_for_pyramide = SPAWN_POSITION.0 - pyramid_side_width;
    let max_x_reqiured_for_pyramide = SPAWN_POSITION.0 + pyramid_side_width;

    let (min_x_for_lines, max_x_for_lines) =
        all_points.map(|i| i.0).minmax().into_option().unwrap();

    let min_x = min_x_reqiured_for_pyramide.min(min_x_for_lines);
    let max_x = max_x_reqiured_for_pyramide.max(max_x_for_lines);

    let map_width = max_x - min_x;

    let floor_line = ((min_x, max_y), (max_x, max_y));

    lines.push(floor_line);

    let mut map = (0..=max_y)
        .map(|_| (0..=map_width).map(|_| '.').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let cords_to_map_indexes = |pos: &(i32, i32)| ((pos.0 - min_x) as usize, pos.1 as usize);

    for (start, end) in lines
        .iter()
        .map(|(s, e)| (cords_to_map_indexes(s), cords_to_map_indexes(e)))
    {
        let start_x = start.0.min(end.0);
        let end_x = start.0.max(end.0);
        let start_y = start.1.min(end.1);
        let end_y = start.1.max(end.1);

        for x in start_x..=end_x {
            for y in start_y..=end_y {
                map[y][x] = '#';
            }
        }
    }

    let mut sand_pieces: Vec<Option<(usize, usize)>> = vec![];

    loop {
        for option in sand_pieces.iter_mut() {
            if let Some((x, y)) = option {
                let is_collision = |(x, y)| {
                    let row: &Vec<_> = &map[y];
                    if let Some(&state) = row.get(x) {
                        state == '#' || state == 'o'
                    } else {
                        false
                    }
                };

                // down
                {
                    let next = (*x, *y + 1);
                    if !is_collision(next) {
                        *option = Some(next);
                        continue;
                    }
                }

                // left
                {
                    let next = (*x - 1, *y + 1);

                    if !is_collision(next) {
                        *option = Some(next);
                        continue;
                    }
                }

                // right
                {
                    let next = (*x + 1, *y + 1);

                    if !is_collision(next) {
                        *option = Some(next);
                        continue;
                    }
                }

                map[*y][*x] = 'o';
                *option = None
            }
        }

        let spawn_cell = cords_to_map_indexes(&SPAWN_POSITION);

        if map[spawn_cell.1][spawn_cell.0] == 'o' {
            return map
                .iter()
                .map(|row| row.iter().filter(|c| **c == 'o').count())
                .sum();
        }

        sand_pieces.push(Some(spawn_cell));
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_solve_first_problem() {
        assert_eq!(a1(), 610);
    }

    #[test]
    fn should_solve_second_problem() {
        assert_eq!(a2(), 0);
    }
}
