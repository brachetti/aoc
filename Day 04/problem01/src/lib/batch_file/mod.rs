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

#[derive(Deserialize, Debug, Clone, Builder, PartialEq, Eq, Hash)]
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
    pub(crate) hcl: Option<RGB>,
    // Eye Color
    #[builder(default)]
    pub(crate) ecl: Option<EyeColor>,
    // passport id
    #[builder(default)]
    pub(crate) pid: Option<String>,
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
                    passport_data_builder.pid(PassportData::string_value(cap));
                }
                Some(name) if name.as_str() == "cid" => {
                    passport_data_builder.cid(PassportData::usize_value(cap));
                }
                Some(name) if name.as_str() == "ecl" => {
                    passport_data_builder.ecl(PassportData::ecl_value(cap));
                }
                Some(name) if name.as_str() == "hcl" => {
                    passport_data_builder.hcl(PassportData::rgb_value(cap));
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

    fn string_value(cap: Captures) -> Option<String> {
        match cap.name("value") {
            Some(value) => {
                let val = String::from(value.as_str());
                Some(val)
            }
            None => None,
        }
    }

    fn ecl_value(cap: Captures) -> Option<EyeColor> {
        match cap.name("value") {
            Some(value) => match value.as_str().parse::<EyeColor>() {
                Ok(val) => Some(val),
                Err(e) => None,
            },
            None => None,
        }
    }

    fn rgb_value(cap: Captures) -> Option<RGB> {
        let rgb_tester = Regex::new(r"\#[a-fA-F0-9]{6}").unwrap();
        match cap.name("value") {
            Some(value) if rgb_tester.is_match(value.as_str()) => {
                let res = RGB::from_str(value.as_str());
                if res.is_ok() {
                    Some(res.unwrap())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn color_value(cap: Captures) -> Option<Color> {
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
    amb,
    blu,
    brn,
    gry,
    grn,
    hzl,
    oth,
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
            "blu" => Ok(Self::blu),
            "oth" => Ok(Self::oth),
            _ => ::std::result::Result::Err(::strum::ParseError::VariantNotFound)
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
    use crate::EyeColor::amb;

    #[test]
    fn should_split_passports() {
        let given_input = "byr:1984\n\nbyr:1985\n\nbyr:1990";
        let result = BatchFile::from_str(given_input).unwrap();

        assert_eq!(result.passports.len(), 3)
    }

    #[test]
    fn should_split_passports2() {
        let given_input = given_aoc_example_input_valid();
        let result = BatchFile::from_str(given_input).unwrap();

        // println!("pps:");
        // for pp in &result.passports {
        //     println!("- {:?}", pp);
        // }

        assert_eq!(result.passports.len(), 4)
    }

    #[test]
    fn should_recognize_ecl_as_name() {
        let given_input = "ecl:amb";
        let result = BatchFile::from_str(given_input).unwrap();

        assert_eq!(
            result
                .passports
                .get(0)
                .expect("Needs to exist")
                .ecl
                .expect("Should exist"),
            EyeColor::amb
        )
    }

    #[test]
    fn should_ignore_invalid_ecl() {
        let given_input = "ecl:z";
        let result = BatchFile::from_str(given_input).unwrap();

        assert_eq!(result.passports.get(0).expect("Needs to exist").ecl, None)
    }

    #[test]
    fn should_validate_passports() {
        let given_input = given_aoc_example_input_valid();
        let batch_file = BatchFile::from_str(given_input).unwrap();
        let policy = NorthPoleFriendlyPolicy::new();
        let result = batch_file.count_valid_passports(Box::new(policy));

        assert_eq!(result, 4)
    }

    #[test]
    fn should_invalidate_passports() {
        let given_input = given_aoc_example_input_invalid();
        let batch_file = BatchFile::from_str(given_input).unwrap();
        let policy = NorthPoleFriendlyPolicy::new();
        let result = batch_file.count_valid_passports(Box::new(policy));

        assert_eq!(result, 0)
    }

    #[test]
    fn should_invalidate_pid() {
        let given_input = "pid:08749970 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f";
        let batch_file = BatchFile::from_str(given_input).unwrap();
        let policy = NorthPoleFriendlyPolicy::new();
        let result = batch_file.count_valid_passports(Box::new(policy));

        assert_eq!(result, 0)
    }

    #[test]
    fn should_invalidate_stuff() {
        let given_input =
            "eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926";
        let batch_file = BatchFile::from_str(given_input).unwrap();
        let policy = NorthPoleFriendlyPolicy::new();
        let result = batch_file.count_valid_passports(Box::new(policy));

        assert_eq!(result, 0)
    }

    fn given_aoc_example_input_valid() -> &'static str {
        "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
    }

    fn given_aoc_example_input_invalid() -> &'static str {
        "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"
    }
}
