use crate::a18::data::Data;

pub fn to_vec_mut(d: &mut Data) -> Vec<&mut u8> {
    let mut res = vec![];

    to_vec_mut_inner(d, &mut res);

    res
}

fn to_vec_mut_inner<'a>(d: &'a mut Data, res: &mut Vec<&'a mut u8>) {
    match d {
        Data::Pair(pair) => {
            to_vec_mut_inner(&mut pair.0, res);
            to_vec_mut_inner(&mut pair.1, res);
        }
        Data::Integer(number) => res.push(number),
    }
}

#[cfg(test)]
mod tests {
    use crate::a18::data::Data;
    use pretty_assertions::assert_eq;

    use super::to_vec_mut;

    fn parse(raw: &str) -> Data {
        serde_json::from_str(raw).unwrap()
    }

    #[test]
    fn to_vec_mut_basic() {
        let mut result = parse("[[[[[9,8],1],2],3],4]");

        assert_eq!(
            to_vec_mut(&mut result),
            vec![&mut 9, &mut 8, &mut 1, &mut 2, &mut 3, &mut 4]
        );
    }
}
