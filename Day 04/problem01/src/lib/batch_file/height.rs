use crate::lib::prelude::*;
use regex::Captures;

#[derive(PartialEq, Debug, Deserialize, Recap, Copy, Clone, Eq, Hash)]
#[recap(regex = r#"(?P<amount>\d{2,3})(?P<measurement>.{2})"#)]
pub struct Height {
    amount: usize,
    measurement: Measurement,
}

#[derive(PartialEq, Debug, Deserialize, Copy, Clone, Eq, Hash)]
pub enum Measurement {
    r#in,
    cm,
}

#[cfg(test)]
mod tests {
    use crate::lib::batch_file::height::{Height, Measurement};

    #[test]
    fn should_parse_centimetres() {
        let given = "182cm";
        let result: Height = given.parse::<Height>().unwrap();

        assert_eq!(
            result,
            Height {
                amount: 182,
                measurement: Measurement::cm
            }
        )
    }

    #[test]
    fn should_cmp_measurements() {
        assert_eq!(Measurement::r#in, Measurement::r#in)
    }
}
