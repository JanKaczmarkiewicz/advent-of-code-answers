use super::data::Data;

#[derive(Debug)]
pub enum ExplodeResult {
    Exploded(Option<u8>, Option<u8>, u32),
    NotExploaded,
}

fn explode_inner(d: &mut Data, depth: u32) -> ExplodeResult {
    if let Data::Pair(pair) = d {
        let left = &mut pair.0;
        let right = &mut pair.1;

        if let (Data::Integer(l), Data::Integer(r)) = (&left, &right) {
            if depth >= 3 {
                let result = ExplodeResult::Exploded(Some(*l), Some(*r), depth);
                *d = Data::Integer(0);
                return result;
            }
        }

        for result in [
            explode_inner(left, depth + 1),
            explode_inner(right, depth + 1),
        ] {
            if let ExplodeResult::Exploded(exploded_left, exploded_right, exploded_at) = result {
                if let Some(exploded_left) = exploded_left {
                    if let Data::Integer(left) = left {
                        let skip_overriding_exploaded_pair = *left == 0 && exploded_at == depth + 1;

                        if !skip_overriding_exploaded_pair {
                            *left += exploded_left;
                            return ExplodeResult::Exploded(None, exploded_right, exploded_at);
                        }
                    }
                }

                if let Some(exploded_right) = exploded_right {
                    if let Data::Integer(right) = right {
                        let skip_overriding_exploaded_pair =
                            *right == 0 && exploded_at == depth + 1;

                        if !skip_overriding_exploaded_pair {
                            *right += exploded_right;
                            return ExplodeResult::Exploded(exploded_left, None, exploded_at);
                        }
                    }
                }

                return ExplodeResult::Exploded(None, None, exploded_at);
            }
        }
    }

    return ExplodeResult::NotExploaded;
}

pub fn explode(mut d: Data) -> Data {
    explode_inner(&mut d, 0);
    return d;
}

#[cfg(test)]
mod tests {
    use crate::a18::data::Data;
    use pretty_assertions::assert_eq;

    use super::explode;

    fn parse(raw: &str) -> Data {
        serde_json::from_str(raw).unwrap()
    }

    #[test]
    fn one() {
        let res = explode(parse("[[[[[9,8],1],2],3],4]"));

        assert_eq!(parse("[[[[0,9],2],3],4]"), res);
    }

    #[test]
    fn two() {
        let res = explode(parse("[7,[6,[5,[4,[3,2]]]]]"));

        assert_eq!(parse("[7,[6,[5,[7,0]]]]"), res);
    }

    #[test]
    fn three() {
        let res = explode(parse("[[6,[5,[4,[3,2]]]],1]"));

        assert_eq!(parse("[[6,[5,[7,0]]],3]"), res);
    }

    #[test]
    fn four() {
        let res = explode(parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"));

        assert_eq!(parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"), res);
    }

    #[test]
    fn five() {
        let res = explode(parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"));

        assert_eq!(parse("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"), res);
    }
}
