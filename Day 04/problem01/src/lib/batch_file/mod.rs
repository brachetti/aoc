use crate::lib::prelude::*;

pub mod height;
pub mod policy;
pub mod rgb;

pub use policy::NorthPoleFriendlyPolicy;
pub use policy::ValidityPolicy;

use super::derive_builder::export::core::num::ParseIntError;
use crate::Color::{Detailed, Simple};
use height::*;
use regex::{Captures, Match};
use rgb::*;
use serde::Deserializer;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct BatchFile {
    pub(crate) passports: Vec<PassportData>,
}

impl FromStr for BatchFile {
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lines = input.split_terminator("\n\n");
        let result: Vec<PassportData> = lines
            .into_iter()
            .map(|l| PassportData::from_str(l))
            .map(|v| v.unwrap())
            .collect();

        Ok(BatchFile { passports: result })
    }
}

impl BatchFile {
    pub fn validate(&self, policy: Box<dyn ValidityPolicy>) -> HashMap<&PassportData, bool> {
        let mut result = HashMap::with_capacity(self.passports.len());
        self.passports.iter().for_each(|pp| {
            result.insert(pp, policy.is_valid(pp));
        });

        result
    }

    pub fn count_valid_passports(&self, policy: Box<dyn ValidityPolicy>) -> usize {
        self.validate(policy)
            .into_iter()
            .filter(|(_, is_valid)| *is_valid == true)
            .count()
    }
}

#[derive(Deserialize, Debug, Copy, Clone, Builder, PartialEq, Eq, Hash)]
// #[builder]
pub struct PassportData {
    // Birth year
    #[builder(default)]
    pub(crate) byr: Option<usize>,
    // Issue year
    #[builder(default)]
    pub(crate) iyr: Option<usize>,
    // Expiration year
    #[builder(default)]
    pub(crate) eyr: Option<usize>,
    // height in centimeters
    #[builder(default)]
    pub(crate) hgt: Option<Height>,
    // hair color
    #[builder(default)]
    pub(crate) hcl: Option<Color>,
    // Eye Color
    #[builder(default)]
    pub(crate) ecl: Option<EyeColor>,
    // passport id
    #[builder(default)]
    pub(crate) pid: Option<usize>,
    // country id of issuing country
    #[builder(default)]
    pub(crate) cid: Option<usize>,
}

impl FromStr for PassportData {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(r#"(?:(?P<name>\S{3}):(?P<value>[\S]+))+"#).unwrap();
        let mut passport_data_builder = PassportDataBuilder::default();
        for cap in regex.captures_iter(input) {
            match cap.name("name") {
                Some(name) if name.as_str() == "byr" => {
                    passport_data_builder.byr(PassportData::usize_value(cap));
                }
                Some(name) if name.as_str() == "iyr" => {
                    passport_data_builder.iyr(PassportData::usize_value(cap));
                }
                Some(name) if name.as_str() == "eyr" => {
                    passport_data_builder.eyr(PassportData::usize_value(cap));
                }
                Some(name) if name.as_str() == "pid" => {
                    passport_data_builder.pid(PassportData::usize_value(cap));
                }
                Some(name) if name.as_str() == "cid" => {
                    passport_data_builder.cid(PassportData::usize_value(cap));
                }
                Some(name) if name.as_str() == "ecl" => {
                    passport_data_builder.ecl(PassportData::ecl_value(cap));
                }
                Some(name) if name.as_str() == "hcl" => {
                    passport_data_builder.hcl(PassportData::hcl_value(cap));
                }
                Some(name) if name.as_str() == "hgt" => {
                    passport_data_builder.hgt(PassportData::hgt_value(cap));
                }
                None => {}
                _ => {}
            }
        }

        let result: PassportData = passport_data_builder.build().unwrap();
        Ok(result)
    }
}

impl PassportData {
    fn usize_value(cap: Captures) -> Option<usize> {
        match cap.name("value") {
            Some(value) => match usize::from_str(value.as_str()) {
                Ok(val) => Some(val),
                _ => None,
            },
            None => None,
        }
    }

    fn ecl_value(cap: Captures) -> Option<EyeColor> {
        match cap.name("value") {
            Some(value) => match value.as_str().parse::<EyeColor>() {
                Ok(val) => Some(val),
                _ => None,
            },
            None => None,
        }
    }

    fn hcl_value(cap: Captures) -> Option<Color> {
        let rgb_tester = Regex::new(r"\#?[a-fA-F0-9]{6}").unwrap();
        match cap.name("value") {
            Some(value) if rgb_tester.is_match(value.as_str()) => {
                match value.as_str().parse::<RGB>() {
                    Ok(val) => Some(Detailed(val)),
                    _ => None,
                }
            }
            Some(value) => match value.as_str().parse::<EyeColor>() {
                Ok(val) => Some(Simple(val)),
                _ => None,
            },
            None => None,
        }
    }

    fn hgt_value(cap: Captures) -> Option<Height> {
        match cap.name("value") {
            Some(value) => match value.as_str().parse::<Height>() {
                Ok(val) => Some(val),
                _ => None,
            },
            None => None,
        }
    }
}

#[derive(Copy, Clone, Deserialize, Hash, Eq, PartialEq, Debug)]
pub enum Color {
    Simple(EyeColor),
    Detailed(RGB),
}

#[derive(Debug, PartialEq, Copy, Clone, Deserialize, Eq, Hash)]
pub enum EyeColor {
    gry,
    brn,
    grn,
    amb,
    hzl,
    zzz,
    z,
}

impl FromStr for EyeColor {
    type Err = ::strum::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gry" => Ok(Self::gry),
            "brn" => Ok(Self::brn),
            "grn" => Ok(Self::grn),
            "amb" => Ok(Self::amb),
            "hzl" => Ok(Self::hzl),
            "zzz" => Ok(Self::zzz),
            "z" => Ok(Self::z),
            _ => ::std::result::Result::Err(::strum::ParseError::VariantNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lib::batch_file::policy::{NorthPoleFriendlyPolicy, StraightPolicy};
    use crate::lib::batch_file::rgb::RGB;
    use crate::lib::batch_file::BatchFile;
    use crate::lib::prelude::*;
    use crate::Color::{Detailed, Simple};

    #[test]
    fn should_split_passports() {
        let given_input = "byr:1984\n\nbyr:1985\n\nbyr:1990";
        let result = BatchFile::from_str(given_input).unwrap();

        assert_eq!(result.passports.len(), 3)
    }

    #[test]
    fn should_split_passports2() {
        let given_input = given_aoc_example_input();
        let result = BatchFile::from_str(given_input).unwrap();

        println!("pps:");
        for pp in &result.passports {
            println!("- {:?}", pp);
        }

        assert_eq!(result.passports.len(), 4)
    }

    #[test]
    fn should_recognize_hcl_as_rgb() {
        let given_input = "hcl:#00FF00";
        let result = BatchFile::from_str(given_input).unwrap();

        assert_eq!(
        result.passports.get(0).expect("Needs to exist").hcl.expect("Should exist"),
        Detailed(RGB::new(0, 255, 0,))
        )
    }

    #[test]
    fn should_recognize_hcl_as_rgb_2() {
        let given_input = "hcl:00FF00";
        let result = BatchFile::from_str(given_input).unwrap();

        assert_eq!(
        result.passports.get(0).expect("Needs to exist").hcl.expect("Should exist"),
        Detailed(RGB::new(0, 255, 0,))
        )
    }

    #[test]
    fn should_recognize_hcl_as_name() {
        let given_input = "hcl:hzl";
        let result = BatchFile::from_str(given_input).unwrap();

        assert_eq!(
        result.passports.get(0).expect("Needs to exist").hcl.expect("Should exist"),
        Simple(EyeColor::hzl)
        )
    }

    #[test]
    fn should_validate_aoc_traditionally() {
        let given_input = given_aoc_example_input();
        let batch_file = BatchFile::from_str(given_input).unwrap();
        let policy = StraightPolicy::new();
        let result = batch_file.count_valid_passports(Box::new(policy));

        assert_eq!(result, 1)
    }

    #[test]
    fn should_validate_aoc_north_pole_friendly() {
        let given_input = given_aoc_example_input();
        let batch_file = BatchFile::from_str(given_input).unwrap();
        let policy = NorthPoleFriendlyPolicy::new();
        let result = batch_file.count_valid_passports(Box::new(policy));

        assert_eq!(result, 2)
    }

    fn given_aoc_example_input() -> &'static str {
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"
    }
}
