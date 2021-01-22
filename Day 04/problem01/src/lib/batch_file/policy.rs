use crate::lib::batch_file::height::Measurement;
use crate::lib::batch_file::height::Measurement::cm;
use crate::lib::prelude::*;

pub trait ValidityPolicy {
    fn is_valid(&self, passport_data: &PassportData) -> bool;

    fn create_values_map(&self, passport_data: &PassportData) -> HashMap<&str, bool> {
        let mut tests: HashMap<&str, bool> = HashMap::new();
        tests.insert("byr", {
            let d = passport_data.byr;
            d.is_some() && d.unwrap() >= 1920 && d.unwrap() <= 2002
        });
        tests.insert("cid", passport_data.cid.is_some());
        tests.insert("ecl", {
            let d = passport_data.ecl;
            passport_data.ecl.is_some()
        });
        tests.insert("eyr", {
            let d = passport_data.eyr;
            d.is_some() && d.unwrap() >= 2020 && d.unwrap() <= 2030
        });
        tests.insert("hcl", passport_data.hcl.is_some());
        tests.insert(
            "hgt",
            match passport_data.hgt {
                Some(height) if height.measurement == cm => {
                    150 <= height.amount && height.amount <= 193
                }
                Some(height) if height.measurement == Measurement::r#in => {
                    height.amount >= 59 && height.amount <= 76
                }
                _ => false,
            },
        );
        tests.insert("iyr", {
            let d = passport_data.iyr;
            d.is_some() && d.unwrap() >= 2010 && d.unwrap() <= 2020
        });
        tests.insert("pid", passport_data.pid.is_some());

        tests
    }
}

pub struct StraightPolicy;

impl ValidityPolicy for StraightPolicy {
    fn is_valid(&self, passport_data: &PassportData) -> bool {
        let tests = self.create_values_map(passport_data);

        tests.into_iter().all(|(_, is_some)| is_some == true)
    }
}

impl StraightPolicy {
    pub fn new() -> Self {
        StraightPolicy {}
    }
}

pub struct NorthPoleFriendlyPolicy;

impl ValidityPolicy for NorthPoleFriendlyPolicy {
    fn is_valid(&self, passport_data: &PassportData) -> bool {
        let mut tests = self.create_values_map(passport_data);
        tests.remove("cid");

        match tests.into_iter().all(|(_, is_some)| is_some == true) {
            true => true,
            false => {
                let mut tests2 = self.create_values_map(passport_data);
                tests2.remove("cid");
                let missing: Vec<&str> = tests2
                    .into_iter()
                    .filter(|(_, is_some)| !*is_some)
                    .map(|(name, _)| name)
                    .collect();
                if missing.contains(&"hcl") || missing.contains(&"ecl") {
                    println!(
                        "Not valid:\n- {:?}\n- missing: {:?}\n",
                        passport_data, missing
                    );
                }
                false
            }
        }
    }
}

impl NorthPoleFriendlyPolicy {
    pub fn new() -> Self {
        NorthPoleFriendlyPolicy {}
    }
}
