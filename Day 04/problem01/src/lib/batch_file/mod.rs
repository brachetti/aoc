use crate::lib::prelude::*;

mod height;
mod rgb;

use super::derive_builder::export::core::num::ParseIntError;
use height::*;
use regex::{Captures, Match};
use rgb::*;

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

#[derive(Deserialize, Debug, Copy, Clone, Builder)]
// #[builder]
pub struct PassportData {
    // Birth year
    #[builder(default)]
    byr: Option<usize>,
    // Issue year
    #[builder(default)]
    iyr: Option<usize>,
    // Expiration year
    #[builder(default)]
    eyr: Option<usize>,
    // height in centimeters
    #[builder(default)]
    hgt: Option<Height>,
    // hair color
    #[builder(default)]
    hcl: Option<RGB>,
    // Eye Color
    #[builder(default)]
    ecl: Option<EyeColor>,
    // passport id
    #[builder(default)]
    pid: Option<usize>,
    // country id of issuing country
    #[builder(default)]
    cid: Option<usize>,
}

impl FromStr for PassportData {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(r#"(?:(?P<name>\S{3}):(?P<value>[\S]+))+"#).unwrap();
        let mut passport_data_builder = PassportDataBuilder::default();
        for cap in regex.captures_iter(input) {
            println!("cap {:?}", cap);
            match cap.name("name") {
                Some(name) if name.as_str() == "byr" => {
                    passport_data_builder.byr(PassportData::usize_value(cap));
                }
                Some(name) if name.as_str() == "iyr" => {
                    passport_data_builder.iyr(PassportData::usize_value(cap));
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
            Some(value) => {
                let val = value.as_str();
                match usize::from_str(val) {
                    Ok(T) => Some(T),
                    _ => None
                }
            },
            None => None,
        }

    }
}

#[derive(Deserialize, Debug, Copy, Clone)]
pub enum EyeColor {
    gry,
    brn,
    grn,
}

#[cfg(test)]
mod tests {
    use crate::lib::batch_file::BatchFile;
    use crate::lib::prelude::*;

    #[test]
    fn should_split_passports() {
        let given_input = "byr:1984\n\nbyr:1985\n\nbyr:1990";
        let result = BatchFile::from_str(given_input);
    }

    #[test]
    fn should_split_passports2() {
        let given_input = given_aoc_example_input();
        let result = BatchFile::from_str(given_input).unwrap();
        println!("end of test");
        for pp in result.passports {
            println!("- {:?}", pp);
        }
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
