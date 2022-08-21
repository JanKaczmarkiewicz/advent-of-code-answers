use crate::a18::data::Data;

pub fn reset_at(d: &mut Data, index: usize) {
    let mut current_index = 0;

    reset_at_inner(d, index, &mut current_index);
}

fn reset_at_inner(d: &mut Data, index: usize, current_index: &mut usize) -> bool {
    match d {
        Data::Pair(pair) => {
            let left = &mut pair.0;
            let right = &mut pair.1;

            if *current_index == index {
                if let (Data::Integer(_), Data::Integer(_)) = (&left, &right) {
                    *d = Data::Integer(0);
                    return true;
                }
            }

            return reset_at_inner(left, index, current_index)
                || reset_at_inner(right, index, current_index);
        }
        Data::Integer(_) => {
            *current_index += 1;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::a18::data::Data;
    use pretty_assertions::assert_eq;

    use super::reset_at;

    fn parse(raw: &str) -> Data {
        serde_json::from_str(raw).unwrap()
    }

    #[test]
    fn reset_at_inner() {
        let mut d = parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");

        reset_at(&mut d, 7);

        assert_eq!(d, parse("[[3,[2,[8,0]]],[9,[5,[4,0]]]]"))
    }
}
