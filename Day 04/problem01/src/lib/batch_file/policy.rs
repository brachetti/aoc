use crate::lib::batch_file::height::{Height, Measurement};
use crate::lib::prelude::*;
use crate::lib::batch_file::rgb::RGB;

pub trait ValidityPolicy {
    fn is_valid(&self, passport_data: &PassportData) -> bool;

    fn byr_test(&self, data: Option<usize>) -> bool {
        let d = data.unwrap_or(0);
        d >= 1920 && d <= 2002
    }

    fn iyr_test(&self, data: Option<usize>) -> bool {
        let d = data.unwrap_or(0);
        d >= 2010 && d <= 2020
    }

    fn eyr_test(&self, data: Option<usize>) -> bool {
        let d = data.unwrap_or(0);
        d >= 2020 && d <= 2030
    }

    fn hgt_test(&self, data: Option<Height>) -> bool {
        let d = data.unwrap_or(Height { amount: 0, measurement: Measurement::cm });
        match d.measurement {
            Measurement::cm => 150 <= d.amount && d.amount <= 193,
            Measurement::r#in => d.amount >= 59 && d.amount <= 76,
        }
    }

    fn hcl_test(&self, data: Option<RGB>) -> bool {
        data.is_some()
    }

    fn ecl_test(&self, data: Option<EyeColor>) -> bool {
        data.is_some()
    }

    fn pid_test(&self, data: Option<&String>) -> bool {
        let default = String::default();
        let d = data.unwrap_or(&default);
        let regex = Regex::new(r"[0-9]{9}").unwrap();
        regex.is_match(d.as_str())
    }

    fn cid_test(&self, data: Option<usize>) -> bool {
        data.is_some()
    }
}

pub struct StraightPolicy;

impl StraightPolicy {
    pub fn new() -> Self {
        StraightPolicy {}
    }
}

impl ValidityPolicy for StraightPolicy {
    fn is_valid(&self, passport_data: &PassportData) -> bool {
        passport_data.is_valid(self)
    }
}

pub struct NorthPoleFriendlyPolicy;

impl ValidityPolicy for NorthPoleFriendlyPolicy {
    fn is_valid(&self, passport_data: &PassportData) -> bool {
        passport_data.is_valid(self)
    }

    fn cid_test(&self, _data: Option<usize>) -> bool {
        true
    }
}

impl NorthPoleFriendlyPolicy {
    pub fn new() -> Self {
        NorthPoleFriendlyPolicy {}
    }
}

#[cfg(test)]
mod tests {
    use crate::lib::prelude::*;
    use crate::lib::batch_file::policy::StraightPolicy;
    use crate::lib::batch_file::height::Height;
    use crate::lib::prelude::height::Measurement;
    use crate::lib::batch_file::rgb::RGB;

    #[test]
    fn should_reject_byr() {
        rejecting_byr(120);
        rejecting_byr(1800);
        rejecting_byr(1919);
        rejecting_byr(2003);
        rejecting_byr(2021);
    }

    #[test]
    fn should_accept_byr() {
        accepting_byr(1920);
        accepting_byr(1980);
        accepting_byr(2002);
    }

    fn accepting_byr(byr: usize) {
        let pd = building_pd().byr(Some(byr)).build().unwrap();
        let policy = given_policy();
        let result = when_checking_validity(&pd, policy);

        assert_eq!(result, true, "byr of {} should be valid", byr)
    }

    fn rejecting_byr(byr: usize) {
        let pd = building_pd().byr(Some(byr)).build().unwrap();
        let policy = given_policy();
        let result = when_checking_validity(&pd, policy);

        assert_eq!(result, false, "byr of {} should be invalid", byr)
    }

    fn building_pd() -> PassportDataBuilder {
        let mut pdb = PassportDataBuilder::default();
        pdb.byr(Some(1960))
            .iyr(Some(2012))
            .eyr(Some(2025))
            .hgt(Some(Height { amount: 170, measurement: Measurement::cm }))
            .hcl(Some(RGB::new(10, 10, 10)))
            .ecl(Some(EyeColor::amb))
            .pid(Some("012345678".to_string()))
            .cid(Some(5));
        pdb
    }

    fn when_checking_validity(pd: &PassportData, policy: StraightPolicy) -> bool {
        policy.is_valid(pd)
    }

    fn given_policy() -> StraightPolicy {
        StraightPolicy::new()
    }
}