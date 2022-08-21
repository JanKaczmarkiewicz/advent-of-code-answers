mod find_explosion_index;
mod to_vec_mut;
use super::data::Data;
use find_explosion_index::find_explosion_index;
use to_vec_mut::to_vec_mut;

fn explode(data: Data) -> (Data, bool) {
    panic!("Not implemented");
    find_explosion_index(data);
    to_vec_mut(&mut data);
}

#[cfg(test)]
mod tests {
    use crate::a18::data::Data;
    use pretty_assertions::assert_eq;

    use super::explode;

    fn parse(raw: &str) -> Data {
        serde_json::from_str(raw).unwrap()
    }

    // #[test]
    // fn one() {
    //     let res = explode(parse("[[[[[9,8],1],2],3],4]"));

    //     assert_eq!(parse("[[[[0,9],2],3],4]"), res);
    // }

    // #[test]
    // fn two() {
    //     let res = explode(parse("[7,[6,[5,[4,[3,2]]]]]"));

    //     assert_eq!(parse("[7,[6,[5,[7,0]]]]"), res);
    // }

    // #[test]
    // fn three() {
    //     let res = explode(parse("[[6,[5,[4,[3,2]]]],1]"));

    //     assert_eq!(parse("[[6,[5,[7,0]]],3]"), res);
    // }

    // #[test]
    // fn four() {
    //     let res = explode(parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"));

    //     assert_eq!(parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"), res);
    // }

    // #[test]
    // fn five() {
    //     let res = explode(parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"));

    //     assert_eq!(parse("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"), res);
    // }
}
