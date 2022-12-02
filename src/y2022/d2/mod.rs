use itertools::Itertools;

use crate::utils::read_lines;

pub fn answer() {
    a1();
    a2();
}

pub fn a1() {
    let res = read_lines("src/y2022/d2/input")
        .filter_map(|l| {
            l.chars()
                .filter(|x| *x != ' ')
                .collect_tuple::<(char, char)>()
                .map(|(e, m)| match e {
                    'A' => match m {
                        'X' => 1 + 3,
                        'Y' => 2 + 6,
                        'Z' => 3 + 0,
                        _ => unreachable!(),
                    },
                    'B' => match m {
                        'X' => 1 + 0,
                        'Y' => 2 + 3,
                        'Z' => 3 + 6,
                        _ => unreachable!(),
                    },
                    'C' => match m {
                        'X' => 1 + 6,
                        'Y' => 2 + 0,
                        'Z' => 3 + 3,
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                })
        })
        .sum::<i32>();

    println!("a1: {}", res);
}

pub fn a2() {
    let res = read_lines("src/y2022/d2/input")
        .filter_map(|l| {
            l.chars()
                .filter(|x| *x != ' ')
                .collect_tuple::<(char, char)>()
                .map(|(e, m)| {
                    let m = match e {
                        // x lose
                        // y draw
                        // z win
                        'A' => match m {
                            'X' => 'Z',
                            'Y' => 'X',
                            'Z' => 'Y',
                            _ => unreachable!(),
                        },
                        'B' => match m {
                            'X' => 'X',
                            'Y' => 'Y',
                            'Z' => 'Z',
                            _ => unreachable!(),
                        },
                        'C' => match m {
                            'X' => 'Y',
                            'Y' => 'Z',
                            'Z' => 'X',
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };

                    match e {
                        'A' => match m {
                            'X' => 1 + 3,
                            'Y' => 2 + 6,
                            'Z' => 3 + 0,
                            _ => unreachable!(),
                        },
                        'B' => match m {
                            'X' => 1 + 0,
                            'Y' => 2 + 3,
                            'Z' => 3 + 6,
                            _ => unreachable!(),
                        },
                        'C' => match m {
                            'X' => 1 + 6,
                            'Y' => 2 + 0,
                            'Z' => 3 + 3,
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    }
                })
        })
        .sum::<i32>();

    println!("a2: {}", res);
}
