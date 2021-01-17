use crate::lib::prelude::*;

pub trait ValidityPolicy {
    fn is_valid(&self, passport_data: &PassportData) -> bool;
}

