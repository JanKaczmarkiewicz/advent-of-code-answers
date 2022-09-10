fn a() -> u32 {
    0
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
    fn first() {}

    #[test]
    fn second() {}
}
