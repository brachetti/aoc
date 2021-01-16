use crate::lib::prelude::*;

mod height;
mod rgb;

use height::*;
use rgb::*;

pub struct BatchFile {
    passports: Vec<PassportData>,
}

impl FromStr for BatchFile {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(BatchFile { passports: vec![] })
    }
}

pub struct PassportData {
    // Birth year
    byr: usize,
    // Issue year
    iyr: usize,
    // Expiration year
    eyr: usize,
    // height in centimeters
    hgt: Height,
    // hair color
    hcl: RGB,
    // Eye Color
    ecl: EyeColor,
    // passport id
    pid: usize,
    // country id of issuing country
    cid: usize,
}

pub enum EyeColor {
    Gry,
    Brn,
    Grn,
}

