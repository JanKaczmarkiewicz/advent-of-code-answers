use std::{collections::HashSet, ops::RangeInclusive};

use itertools::Itertools;

use crate::utils::read_lines;

pub fn answer() {
    println!("Answer to day15: {} {}", a1(), a2());
}

fn parse_line(s: String) -> ((i32, i32), (i32, i32)) {
    s.replace("Sensor at x=", "")
        .replace(" closest beacon is at x=", "")
        .replace(" y=", "")
        .split(":")
        .map(|s| {
            s.split(",")
                .map(|n| n.parse::<i32>().unwrap())
                .next_tuple::<(i32, i32)>()
                .unwrap()
        })
        .next_tuple()
        .unwrap()
}

fn _visualize() {
    // the easiest way to check would be to draw a map
    let pairs = read_lines("src/y2022/d15/input")
        .map(parse_line)
        .map(|(s, b)| ((s.0, s.1, (s.0 - b.0).abs() + (s.1 - b.1).abs()), b))
        .collect::<Vec<_>>();

    let circles = pairs.iter().map(|(c, _)| c);

    let max_x = circles.clone().map(|s| s.0 + s.2).max().unwrap();
    let min_x = circles.clone().map(|s| s.0 - s.2).min().unwrap();

    let min_y = circles.clone().map(|s| s.1 - s.2).min().unwrap();
    let max_y = circles.clone().map(|s| s.1 + s.2).max().unwrap();

    let mut screen = (min_y..=max_y)
        .map(|_| (min_x..=max_x).map(|_| '.').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut draw_ch = |(x, y): (i32, i32), ch: char| {
        screen[(min_y - y).abs() as usize][(min_x - x).abs() as usize] = ch;
    };

    for (s, b) in pairs {
        let r = s.2;
        let c = ((s.0 + s.1) % (90 - 65) + 65) as u8 as char;

        for i in -r..=r {
            draw_ch((s.0, s.1 - i), c);
        }

        for i in 1..=r {
            for j in -r + i..=r - i {
                draw_ch((s.0 - i, s.1 + j), c);
                draw_ch((s.0 + i, s.1 + j), c);
            }
        }
        draw_ch((s.0, s.1), 'S');
        draw_ch(b, 'b');
    }

    for row in screen {
        for c in row {
            print!("{c}");
        }
        println!("");
    }
}

fn a1() -> usize {
    const TESTED_ROW: i32 = 2000000;
    let mut coords_in_row_where_beacon_cannot_be = HashSet::new();

    read_lines("src/y2022/d15/input")
        .map(parse_line)
        .for_each(|(s, b)| {
            let radius = (s.0 - b.0).abs() + (s.1 - b.1).abs();

            let how_far_from_sensor_border_is_tested_row = radius - (s.1 - TESTED_ROW).abs(); // < 0 if line is outside. = radius if sensor.y is eq TESTED_ROW

            for x in (-how_far_from_sensor_border_is_tested_row
                ..=how_far_from_sensor_border_is_tested_row)
                .map(|i| i + s.0)
            {
                if b != (x, TESTED_ROW) {
                    coords_in_row_where_beacon_cannot_be.insert(x);
                }
            }
        });

    coords_in_row_where_beacon_cannot_be.len()
}

fn range_len(range: &RangeInclusive<i32>) -> i32 {
    *range.end() - *range.start()
}

fn a2() -> usize {
    let map = read_lines("src/y2022/d15/input")
        .map(parse_line)
        .map(|((sx, sy), b)| ((sx, sy, (sx - b.0).abs() + (sy - b.1).abs()), b))
        .collect::<Vec<_>>();

    for tested_row in 0..4000000 {
        let mut sorted_sensor_ranges = map
            .iter()
            .filter_map(|((sx, sy, sr), _)| {
                let how_far_from_sensor_border_is_tested_row = sr - (sy - tested_row).abs(); // < 0 if line is outside. = radius if sensor.y is eq TESTED_ROW

                if how_far_from_sensor_border_is_tested_row > 0 {
                    Some(
                        -how_far_from_sensor_border_is_tested_row + sx
                            ..=how_far_from_sensor_border_is_tested_row + sx,
                    )
                } else {
                    None
                }
            })
            .sorted_by(|l, r| (*l.start() as f64).total_cmp(&(*r.start() as f64)))
            .collect::<Vec<_>>();

        let mut merged_range = sorted_sensor_ranges.remove(0);

        for range in sorted_sensor_ranges {
            let new =
                *merged_range.start().min(range.start())..=*merged_range.end().max(range.end());

            if range_len(&new) <= range_len(&merged_range) + range_len(&range) {
                merged_range = new;
            } else {
                // ranges cannot be added then next x is empty
                return (*merged_range.end() as usize + 1) * 4000000 + tested_row as usize;
            }
        }
    }

    return 0;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn ps_t() {
        assert_eq!(
            parse_line("Sensor at x=2, y=18: closest beacon is at x=-2, y=15".into()),
            ((2, 18), (-2, 15))
        )
    }

    #[test]
    fn should_solve_first_problem() {
        assert_eq!(a1(), 6124805);
    }

    #[test]
    fn should_solve_second_problem() {
        assert_eq!(a2(), 12555527364986);
    }
}
