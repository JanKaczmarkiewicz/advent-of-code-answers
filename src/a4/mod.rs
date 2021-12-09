use crate::utils::read_lines;

const BOARD_SIZE: usize = 5;

fn check_bingo(current_numbers: &[u8], board: &Vec<u8>) -> bool {
    let traverse = |get_index: fn(usize, usize) -> usize|
        (0..BOARD_SIZE).any(
            |i|
                (0..BOARD_SIZE)
                    .all(|j| {
                        let index = get_index(i, j);
                        current_numbers.contains(&board[index])
                    })
        );


    if traverse(|i, j| (BOARD_SIZE * i + j) as usize) {
        return true;
    }

    if traverse(|i, j| (i + BOARD_SIZE * j) as usize) {
        return true;
    }

    false
}

fn a() -> u32 {
    let lines = read_lines("src/a4/input")
        .collect::<Vec<_>>();
    let board_vertical_size = 5+1;

    let boards_count = (lines.len() - 1) / board_vertical_size;

    let all_bingo_numbers = lines.first().unwrap().split(",").map(|x| x.parse::<u8>().unwrap()).collect::<Vec<_>>();

    let boards = (0..boards_count).map(|i| {
        (0..BOARD_SIZE).flat_map(|j| {
            let index =(board_vertical_size * i + j + 2) as usize;
            let current_line = &lines[index];
            (0..BOARD_SIZE).map(|z| {
                let str_fragment = &current_line[3 * z..3 * z + 2];
                str_fragment.trim().parse::<u8>().unwrap()
            })
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let mut i = BOARD_SIZE;
    let mut result_points = 0;
    'find_bingo: loop {
        let current_bingo_numbers = &all_bingo_numbers[0..i];
        for board in &boards {
            if check_bingo(current_bingo_numbers, board) {
                result_points = all_bingo_numbers[i-1] as u32 * board
                    .iter()
                    .filter(|x| !current_bingo_numbers.contains(x))
                    .fold(0_u32, |prev, x| {
                        prev + *x as u32
                    });
                break 'find_bingo;
            };
        }
        i += 1;
    }


    return result_points;
}



pub fn answer() {
    println!("Answer to problem 4: {}", a());
}