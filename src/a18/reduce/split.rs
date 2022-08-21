use super::super::data::Data;

pub fn split(data: &mut Data) -> bool {
    match data {
        Data::Pair(pair) => split(&mut pair.0) || split(&mut pair.1),
        Data::Integer(n) => {
            let n = *n as f32;
            let result = n >= 10.;

            if result {
                *data = Data::Pair(Box::new((
                    Data::Integer((n / 2.).floor() as u8),
                    Data::Integer((n / 2.).ceil() as u8),
                )));
            }

            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::split;
    use crate::a18::data::Data;
    use pretty_assertions::assert_eq;

    fn parse(raw: &str) -> Data {
        serde_json::from_str(raw).unwrap()
    }

    #[test]
    fn one() {
        let mut input = parse("[[[[0,7],4],[15,[0,13]]],[1,1]]");

        let result = split(&mut input);

        assert!(result);
        assert_eq!(input, parse("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"));

        let result = split(&mut input);

        assert!(result);
        assert_eq!(input, parse("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]"));
    }
}
