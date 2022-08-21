mod find_explosion_index;
mod to_vec_mut;
use super::data::Data;
use find_explosion_index::find_explosion_index;
use to_vec_mut::to_vec_mut;

fn explode(mut data: Data) -> (Data, bool) {
    // TODO: we need to change [x,y] -> 0
    if let Some(index_to_explode) = find_explosion_index(&data) {
        let mut numbers_left_to_right = to_vec_mut(&mut data);

        let right = *numbers_left_to_right.remove(index_to_explode + 1); // remove pairs right element
        let left = *numbers_left_to_right[index_to_explode];

        *numbers_left_to_right[index_to_explode - 1] += left;
        *numbers_left_to_right[index_to_explode] = 0;
        *numbers_left_to_right[index_to_explode + 1] += right;

        (data, true)
    } else {
        (data, false)
    }
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

        assert_eq!(res, (parse("[[[[0,9],2],3],4]"), true));
    }

    #[test]
    fn two() {
        let res = explode(parse("[7,[6,[5,[4,[3,2]]]]]"));

        assert_eq!(res, (parse("[7,[6,[5,[7,0]]]]"), true));
    }

    #[test]
    fn three() {
        let res = explode(parse("[[6,[5,[4,[3,2]]]],1]"));

        assert_eq!(res, (parse("[[6,[5,[7,0]]],3]"), true));
    }

    #[test]
    fn four() {
        let res = explode(parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"));

        assert_eq!(res, (parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"), true));
    }

    #[test]
    fn five() {
        let res = explode(parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"));

        assert_eq!(res, (parse("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"), true));
    }
}
