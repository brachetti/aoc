pub mod lib;
extern crate clap;

pub use crate::lib::prelude::*;

use clap::{App, Arg};
use crate::lib::batch_file::policy::StraightPolicy;

fn main() {
    let matches = App::new("scanner")
        .about("North Pole Friendly Scanner")
        .arg(
            Arg::new("file")
                .about("file containing batch of passports")
                .short('f')
                .long("file")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let input_file = matches
        .value_of("file")
        .expect("Please provide an input file");
    let content = std::fs::read_to_string(input_file).expect("Could not open field");

    let batch_file = BatchFile::from_str(content.as_str()).expect("Could not parse Batch file");
    let policy = NorthPoleFriendlyPolicy::new();
    let number_of_valid_passports = batch_file.count_valid_passports(Box::new(policy));

    println!("Valid passports: {:?}", number_of_valid_passports);
}
