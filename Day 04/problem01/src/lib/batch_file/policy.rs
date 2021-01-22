use crate::lib::batch_file::height::Measurement::cm;
use crate::lib::batch_file::height::{Height, Measurement};
use crate::lib::prelude::*;

pub trait ValidityPolicy {
    fn is_valid(&self, passport_data: &PassportData) -> bool;

    fn create_values_map(&self, passport_data: &PassportData) -> HashMap<&str, bool> {
        let mut tests: HashMap<&str, bool> = HashMap::new();
        tests.insert("byr", passport_data.byr.is_some());
        tests.insert("byr range", {
            let d = passport_data.byr.unwrap_or_else(|| 0);
            d >= 1920 && d <= 2002
        });
        tests.insert("cid", passport_data.cid.is_some());
        tests.insert("ecl", passport_data.ecl.is_some());
        tests.insert("eyr", passport_data.eyr.is_some());
        tests.insert("eyr range", {
            let d = passport_data.eyr.unwrap_or(0);
            d >= 2020 && d <= 2030
        });
        tests.insert("hcl", passport_data.hcl.is_some());
        // ordinarily I would put validators into the class itself
        tests.insert("hgt",
            passport_data.hgt.is_some()
        );
        tests.insert("hgt range", {
            let d = passport_data.hgt.unwrap_or_else(|| Height {
                amount: 0,
                measurement: Measurement::cm,
            });
            match d.measurement {
                Measurement::cm => 150 <= d.amount && d.amount <= 193,
                Measurement::r#in => d.amount >= 59 && d.amount <= 76,
            }
        });
        tests.insert("iyr", passport_data.iyr.is_some());
        tests.insert("iyr range", {
            let d = passport_data.iyr.unwrap_or(0);
            d >= 2010 && d <= 2020
        });
        tests.insert("pid", passport_data.pid.is_some());
        tests.insert("pid format", {
            let d = passport_data.pid.clone().unwrap_or("".into());
            let regex = Regex::new(r"[0-9]{9}").unwrap();
            regex.is_match(d.as_str())
        });

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
