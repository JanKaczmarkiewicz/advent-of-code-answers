use crate::utils::read_lines;

fn a() -> u32 {
    let lines = read_lines("src/a3/input")
        .collect::<Vec<_>>();
    let mut result_numbers = vec![0; lines.first().unwrap().len()];

    let lines_count = lines.len();

    for line in lines {
        for (i, char) in line.chars().enumerate() {
            result_numbers[i] += char.to_digit(10).unwrap();
        }
    }

    let get_result = || result_numbers
        .iter()
        .map(|&x| lines_count - (x as usize) < (x as usize))
        .rev()
        .enumerate();

    let it_gamma = get_result()
        .fold(0, |prev, (i, cond)|
            prev + if cond { 2_u32.pow(i as u32) } else { 0 },
        );
    let it_epsilon = get_result()
        .fold(0, |prev, (i, cond)|
            prev + if cond { 0 } else { 2_u32.pow(i as u32) },
        );

    assert_eq!(3969000, it_gamma * it_epsilon);
    return it_gamma * it_epsilon;
}

fn find(matrix: Vec<Vec<u32>>, depth: usize, pick_matrix: fn(Vec<Vec<u32>>, Vec<Vec<u32>>) -> Vec<Vec<u32>>) -> u32 {
    if matrix.len() == 1 {
        matrix[0]
            .clone()
            .iter()
            .rev()
            .enumerate()
            .fold(0, |prev, (i, cond)|
                prev + cond * 2_u32.pow(i as u32),
            )
    } else {
        let (zeros, ones): (Vec<_>, Vec<_>) = matrix
            .into_iter()
            .partition(|e| e[depth] == 0);
        find(pick_matrix(zeros, ones), depth + 1, pick_matrix)
    }
}

fn b() -> u32 {
    let lines = read_lines("src/a3/input")
        .map(|line| line
            .chars()
            .map(|char| char.to_digit(10).unwrap() as u32)
            .collect::<Vec<u32>>())
        .collect::<Vec<_>>();

    // TODO: remove clone
    let oxygen = find(
        lines.clone(),
        0,
        |zeros, ones| if zeros.len() > ones.len() { zeros } else { ones },
    );

    let scrubber = find(
        lines.clone(),
        0,
        |zeros, ones| if zeros.len() > ones.len() { ones } else { zeros },
    );

    assert_eq!(4267809, scrubber * oxygen);
    return scrubber * oxygen;
}


pub fn answer() {
    println!("Answer to problem 3: {}, {}", a(), b());
}