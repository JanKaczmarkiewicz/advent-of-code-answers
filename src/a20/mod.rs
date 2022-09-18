use itertools::Itertools;

use crate::utils::read;

fn char_to_bool(c: char) -> bool {
    match c {
        '.' => false,
        '#' => true,
        _ => panic!("Invalid character {c}"),
    }
}

fn resize_image(image: &mut Vec<Vec<bool>>) {
    let first_row = (0..image[0].len()).map(|_| false).collect::<Vec<_>>();
    let last_row = (0..image[0].len()).map(|_| false).collect::<Vec<_>>();

    image.insert(0, first_row);
    image.push(last_row);

    for row in image.iter_mut() {
        row.insert(0, false);
        row.push(false);
    }
}

fn debug(image: &Vec<Vec<bool>>) {
    for row in image {
        for c in row {
            print!("{}", if *c { '#' } else { '.' });
        }
        println!();
    }
    println!();
}

fn a() -> usize {
    let raw = read("src/a20/input");

    let mut lines = raw.lines();

    let algorithm = lines
        .next()
        .unwrap()
        .chars()
        .map(char_to_bool)
        .collect::<Vec<_>>();

    let mut image = lines
        .skip(1)
        .map(|l| l.chars().map(char_to_bool).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for _ in 0..=2 {
        resize_image(&mut image);
        let prev_image = image.clone();

        debug(&image);

        for j in 0..image.len() {
            for i in 0..image[j].len() {
                if i == 2 && j == 4 {
                    print!("");
                }

                let bstr = [
                    (-1, -1),
                    (0, -1),
                    (1, -1),
                    (-1, 0),
                    (0, 0),
                    (1, 0),
                    (-1, 1),
                    (0, 1),
                    (1, 1),
                ]
                .iter()
                .map(|(x, y)| {
                    let x = (i as i32) + x;
                    let y = (j as i32) + y;

                    if prev_image
                        .get(y as usize)
                        .and_then(|row| row.get(x as usize))
                        .copied()
                        .unwrap_or(false)
                    {
                        '1'
                    } else {
                        '0'
                    }
                })
                .join("");

                let algo_index = usize::from_str_radix(&bstr, 2).unwrap();

                image[j][i] = algorithm[algo_index];
            }
        }
    }

    image.into_iter().flatten().filter(|b| *b).count()
}

fn b() -> u32 {
    0
}

pub fn answer() {
    println!("Answer to problem 20: {}, {}", a(), b());
}

#[cfg(test)]
mod tests {
    use super::{a, b};

    #[test]
    fn first() {
        print!("Answer to problem 20: {}, {}", a(), b());
    }

    #[test]
    fn second() {}
}
