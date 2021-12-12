use crate::utils::read;

pub fn answer() {
    let content = read("src/a6/input");
    let mut fishes = content
        .split(",")
        .map(|x|
            x.parse::<u16>().unwrap())
        .collect::<Vec<_>>();

    for i in 0..256 {
        let mut new_spawns: i32 = 0;

        fishes = fishes.iter().map(|&x|
            if x == 0 {
                new_spawns += 1;
                6
            } else {
                x - 1
            }
        ).collect::<Vec<_>>();

        let mut new_fishes = (0..new_spawns).map(|x| 8).collect::<Vec<_>>();

        fishes.append(&mut new_fishes);
        println!("{}", i);
    }


    println!("Answer to problem 6: {}", fishes.len());
}