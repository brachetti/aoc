use lazy_static::lazy_static;
use regex::Regex;
use std::env;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct PasswordLine {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

lazy_static! {
    static ref PASSWORD_LINE_RE: Regex =
        Regex::new(r"^(?P<min>\d+)-(?P<max>\d+) (?P<letter>\w): (?P<password>\w+)$").unwrap();
}

impl PasswordLine {
    fn evaluate_rule(&self) -> bool {
        let count = self.password.matches(self.letter).count();

        count >= self.min && count <= self.max
    }

    fn is_valid(&self) -> bool {
        self.evaluate_rule()
    }
}

impl FromStr for PasswordLine {
    type Err = std::string::ParseError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let c = PASSWORD_LINE_RE.captures(text).unwrap();

        let min = u8::from_str(c.name("min").unwrap().as_str()).unwrap();
        let max = u8::from_str(c.name("max").unwrap().as_str()).unwrap();
        let letter = c.name("letter").unwrap().as_str();
        let password = c.name("password").unwrap().as_str();

        Ok(PasswordLine {
            min: min as usize,
            max: max as usize,
            letter: char::from_str(letter).unwrap(),
            password: password.into(),
        })
    }
}

fn main() {
    let input_file: String = read_input_file();
    let password_lines: Vec<PasswordLine> = extract_password_lines(input_file)
        .into_iter().filter(PasswordLine::is_valid).collect();

    println!("{:?}", password_lines.len())
}

fn read_input_file() -> String {
    let args: Vec<String> = env::args().collect();
    let input = args.get(1).expect("Missing commandline argument!");

    fs::read_to_string(input).expect("Could not read from file!")
}

fn extract_password_lines(input: String) -> Vec<PasswordLine> {
    input.lines().into_iter().map(PasswordLine::from_str).map(Result::unwrap).collect()
}