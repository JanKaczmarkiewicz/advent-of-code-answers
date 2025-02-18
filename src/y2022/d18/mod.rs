use std::collections::HashSet;

use itertools::Itertools;

use crate::utils::read_lines;

pub fn answer() {
    println!("Answer to day18: {} {}", a1(), a2());
}

fn find_outside_of_cords(
    space: &HashSet<(i32, i32, i32)>,
    boundries: (i32, i32, i32),
) -> HashSet<(i32, i32, i32)> {
    let directions = [
        (0, 1, 0),
        (1, 0, 0),
        (-1, 0, 0),
        (0, 0, 1),
        (0, 0, -1),
        (0, -1, 0),
    ];

    let mut qiue = vec![(0, 0, 0)];
    let mut visited = HashSet::new();

    while let Some(cord) = qiue.pop() {
        visited.insert(cord);
        for dir in directions {
            let destination = (cord.0 + dir.0, cord.1 + dir.1, cord.2 + dir.2);
            if destination.0 == -2
                || destination.1 == -2
                || destination.2 == -2
                || destination.0 == boundries.0 + 2
                || destination.1 == boundries.1 + 2
                || destination.2 == boundries.2 + 2
            {
                continue;
            }
            if space.contains(&destination) {
                continue;
            }
            if visited.contains(&destination) {
                continue;
            }
            qiue.push(destination)
        }
    }

    return visited;
}

fn a2() -> usize {
    let mut faces = HashSet::new();

    let cords = read_lines("src/y2022/d18/input")
        .map(|str| {
            str.split(',')
                .map(|s| s.parse::<i32>().unwrap())
                .collect_tuple::<(_, _, _)>()
                .unwrap()
        })
        .collect::<HashSet<_>>();

    let get_faces = |(x, y, z)| {
        [
            (x, y, z, 'x'),
            (x, y, z, 'y'),
            (x, y, z, 'z'),
            (x - 1, y, z, 'x'),
            (x, y - 1, z, 'y'),
            (x, y, z - 1, 'z'),
        ]
    };

    let mut x_max = 0;
    let mut y_max = 0;
    let mut z_max = 0;

    cords.iter().for_each(|(mut x, mut y, mut z)| {
        x_max = x_max.max(x);
        y_max = y_max.max(y);
        z_max = z_max.max(z);

        let new_faces = get_faces((x, y, z));

        for face in new_faces {
            if !faces.insert(face) {
                faces.remove(&face);
            }
        }
    });

    let outside_cords = find_outside_of_cords(&cords, (x_max, y_max, z_max));

    for x in 0..=x_max {
        for y in 0..=y_max {
            for z in 0..=z_max {
                let cord = (x, y, z);
                if !outside_cords.contains(&cord) && !cords.contains(&cord) {
                    // part of bubble detected
                    get_faces(cord).iter().for_each(|face| {
                        faces.remove(face);
                    })
                }
            }
        }
    }

    faces.len()
}
fn a1() -> usize {
    let mut faces = HashSet::new();

    let cords = read_lines("src/y2022/d18/input")
        .map(|str| {
            str.split(',')
                .map(|s| s.parse::<i32>().unwrap())
                .collect_tuple::<(_, _, _)>()
                .unwrap()
        })
        .collect::<HashSet<_>>();

    let get_faces = |(x, y, z)| {
        [
            (x, y, z, 'x'),
            (x, y, z, 'y'),
            (x, y, z, 'z'),
            (x - 1, y, z, 'x'),
            (x, y - 1, z, 'y'),
            (x, y, z - 1, 'z'),
        ]
    };

    cords.iter().for_each(|(mut x, mut y, mut z)| {
        let new_faces = get_faces((x, y, z));

        for face in new_faces {
            if !faces.insert(face) {
                faces.remove(&face);
            }
        }
    });

    return faces.len();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_solve_first_problem() {
        assert_eq!(a1(), 4456);
    }

    #[test]
    fn should_solve_second_problem() {
        assert_eq!(a2(), 2510);
    }
}
