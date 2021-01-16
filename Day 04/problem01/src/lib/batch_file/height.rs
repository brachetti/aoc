use crate::lib::prelude::*;
use Measurement::{Inch, Centimeters};
use regex::Captures;

#[derive(Debug, Clone)]
struct ParseHeightError;

impl Display for ParseHeightError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to parse to a Height")
    }
}

impl Error for ParseHeightError {
    fn description(&self) -> &str {
        "Unable to parse to a Height"
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

impl From<ParseError> for ParseHeightError {
    fn from(_: ParseError) -> Self {
        ParseHeightError
    }
}

#[derive(PartialEq, Debug)]
pub struct Height {
    amount: usize,
    measurement: Measurement,
}

const HEIGHT_REGEX: &str = r"^(?P<height>\d{2,3}})(?P<measurement>in|cm)$";

impl FromStr for Height {
    type Err = ParseHeightError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(HEIGHT_REGEX).unwrap();

        regex.captures(input).ok_or(ParseHeightError)
            .and_then(|cap| Ok(Height {
                amount: &cap["height"],
                measurement: Measurement::from(&cap["measurement"])
            }))
    }
}


#[derive(PartialEq, Debug)]
pub enum Measurement {
    Inch,
    Centimeters,
}

impl FromStr for Measurement {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "in" => Ok(Inch),
            "cm" => Ok(Centimeters),
            _ => panic!("Could not parse"),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::lib::batch_file::height::{Measurement, Height};

    #[test]
    fn should_parse_centimetres() {
        let given = "182cm";
        let result: Height = Height::from_str(given);

        assert_eq!(result, Height { amount: 182, measurement: Measurement::Centimeters })
    }

    #[test]
    fn should_cmp_measurements() {
        assert_eq!(Measurement::Inch, Measurement::Inch)
    }
}
