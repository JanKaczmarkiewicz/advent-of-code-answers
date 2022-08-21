mod explode;
mod split;

use super::data::Data;
use explode::explode;
use split::split;

pub fn reduce(mut data: Data) -> Data {
    // To reduce a snailfish number, you must repeatedly do the first action in this list that applies to the snailfish number:

    // If any pair is nested inside four pairs, the leftmost such pair explodes.
    // If any regular number is 10 or greater, the leftmost such regular number splits.

    loop {
        if !(explode(&mut data) || split(&mut data)) {
            return data;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::reduce;
    use crate::a18::data::Data;
    use pretty_assertions::assert_eq;

    fn parse(raw: &str) -> Data {
        serde_json::from_str(raw).unwrap()
    }

    #[test]
    fn one() {
        let result = reduce(parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]"));

        assert_eq!(result, parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
    }
}
