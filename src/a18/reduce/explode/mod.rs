mod find_explosion_index;
mod reset_at;
mod to_vec_mut;
use super::super::data::Data;
use find_explosion_index::find_explosion_index;
use reset_at::reset_at;
use to_vec_mut::to_vec_mut;

pub fn explode(data: &mut Data) -> bool {
    if let Some(index_to_explode) = find_explosion_index(data) {
        let index_to_explode = index_to_explode;

        let mut numbers_left_to_right = to_vec_mut(data);

        let right = *numbers_left_to_right.remove(index_to_explode + 1); // remove pair right element
        let left = *numbers_left_to_right[index_to_explode];

        if index_to_explode != 0 {
            *numbers_left_to_right[index_to_explode - 1] += left;
        }

        if index_to_explode + 1 != numbers_left_to_right.len() {
            *numbers_left_to_right[index_to_explode + 1] += right;
        }

        reset_at(data, index_to_explode);
        true
    } else {
        false
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
        let mut input = parse("[[[[[9,8],1],2],3],4]");

        let result = explode(&mut input);

        assert!(result);
        assert_eq!(input, parse("[[[[0,9],2],3],4]"));
    }

    #[test]
    fn two() {
        let mut input = parse("[[[[0,9],2],3],4]");

        let result = explode(&mut input);

        assert!(result);
        assert_eq!(input, parse("[7,[6,[5,[7,0]]]]"));
    }

    #[test]
    fn three() {
        let mut input = parse("[[6,[5,[4,[3,2]]]],1]");

        let result = explode(&mut input);

        assert!(result);
        assert_eq!(input, parse("[[6,[5,[7,0]]],3]"));
    }

    #[test]
    fn four() {
        let mut input = parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");

        let result = explode(&mut input);

        assert!(result);
        assert_eq!(input, parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"));
    }

    #[test]
    fn five() {
        let mut input = parse("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]");

        let result = explode(&mut input);

        assert!(result);
        assert_eq!(input, parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
    }
}
