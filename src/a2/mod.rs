use crate::utils::read_lines;

pub fn answer() {
    let get_operations = || {
        read_lines("src/a2/input").map(|line| {
            let res: Vec<&str> = line.split(' ').collect();
            let action = String::from(res[0]);
            let arg: u64 = res[1].parse().unwrap();

            (action, arg)
        })
    };

    let (depth, horizontal_position) = get_operations().fold(
        (0, 0),
        |(depth, horizontal_position), (action, arg)| match &action[..] {
            "forward" => (depth, horizontal_position + arg),
            "down" => (depth + arg, horizontal_position),
            "up" => (depth - arg, horizontal_position),
            _ => (depth, horizontal_position),
        },
    );

    let (depth_with_aim, horizontal_position_with_aim, _aim) = get_operations().fold(
        (0, 0, 0),
        |(depth, horizontal_position, aim), (action, arg)| match &action[..] {
            "forward" => (depth + aim * arg, horizontal_position + arg, aim),
            "down" => (depth, horizontal_position, aim + arg),
            "up" => (depth, horizontal_position, aim - arg),
            _ => (depth, horizontal_position, aim),
        },
    );

    println!(
        "Answer to problem 2: {}, {}",
        depth * horizontal_position,
        depth_with_aim * horizontal_position_with_aim
    );
}
