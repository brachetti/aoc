use std::{str::FromStr, usize};
use clap::{Arg, Command};

#[derive(Debug, Eq, PartialEq, Clone)]
struct Elf {
    calories: usize,
}

impl FromStr for Elf {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let calories: usize = s.lines()
            .map(str::trim)
            .map(|x| usize::from_str(x).unwrap())
            .sum();

        Ok(Elf {
            calories
        })
    }
}

fn get_elfs(input: &str) -> Vec<Elf> {
    let elfs: Vec<Elf> = input.split("\n\n").map(|i| Elf::from_str(i).unwrap()).collect();

    elfs
}

fn main() {
    let matches = Command::new("AOC Day 01")
        .about("Elfs with calories")
        .arg(
            Arg::new("file")
            .short('f')
            .long("file")
            .required(true)
            .help("input file")
            )
        .get_matches();

    let input_file: &String = matches.get_one::<String>("file").expect("'file' is required");
    let content = std::fs::read_to_string(input_file).expect("Could not open file");

    let elfs: Vec<Elf> = get_elfs(&content);
}

#[cfg(test)]
mod tests {
    use std::{str::FromStr};

    use crate::{Elf, get_elfs};

    #[test]
    fn should_have_elf_w_calories() {
        let input = "1000";
        let elf = Elf::from_str(input).unwrap();

        assert_eq!(elf.calories, 1000)
    }

    #[test]
    fn should_have_elf_w_calories_1() {
        let input = "2000";
        let elf = Elf::from_str(input).unwrap();

        assert_eq!(elf.calories, 2000)
    }

    #[test]
    fn should_have_elf_w_more_calories() {
        let input = "1000\n2000";
        let elf = Elf::from_str(input).unwrap();

        assert_eq!(elf.calories, 3000)
    }

    #[test]
    fn should_have_one_elf() {
        let input = "1000\n";
        let elfs = get_elfs(input);
        assert_eq!(elfs.len(), 1);
    }

    #[test]
    fn should_have_two_elfs() {
        let input = "1000\n\n2000";
        let elfs = get_elfs(input);
        assert_eq!(elfs.len(), 2);
    }
}
