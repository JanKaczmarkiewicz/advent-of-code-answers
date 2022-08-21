use crate::a18::data::Data;

fn find_explosion_index_inner(d: &mut Data, depth: usize, index: &mut i32) -> bool {
    match d {
        Data::Pair(pair) => {
            let left = &mut pair.0;
            let right = &mut pair.1;

            if let (Data::Integer(_), Data::Integer(_)) = (&left, &right) {
                if depth >= 4 {
                    return true;
                }
            }

            let result = find_explosion_index_inner(left, depth + 1, index)
                || find_explosion_index_inner(right, depth + 1, index);

            if result {
                return result;
            }
        }
        Data::Integer(_) => {
            *index += 1;
        }
    }

    return false;
}

pub fn find_explosion_index(mut d: Data) -> Option<usize> {
    let mut index = -1;

    if find_explosion_index_inner(&mut d, 0, &mut index) {
        Some((index + 1) as usize)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::find_explosion_index;

    fn setup(raw: &str) -> Option<usize> {
        find_explosion_index(serde_json::from_str(raw).unwrap())
    }

    #[test]
    fn find_explosion_index_one() {
        assert_eq!(setup("[[[[[9,8],1],2],3],4]"), Some(0));
    }

    #[test]
    fn find_explosion_index_two() {
        assert_eq!(setup("[7,[6,[5,[4,[3,2]]]]]"), Some(4));
    }

    #[test]
    fn find_explosion_index_three() {
        assert_eq!(setup("[[6,[5,[4,[3,2]]]],1]"), Some(3));
    }

    #[test]
    fn find_explosion_index_four() {
        assert_eq!(setup("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"), Some(7));
    }
}
