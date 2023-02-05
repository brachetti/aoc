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
    input.split("\n\n").map(|i| Elf::from_str(i).unwrap()).collect()
}

fn sum_top_elfs(elfs: Vec<Elf>, amount: usize) -> usize {
    let mut calories: Vec<usize> = elfs.into_iter()
        .map(|e| e.calories)
        .collect();

    // reverse sort
    calories.sort_unstable_by(|a,b| b.cmp(a));

    calories.iter().take(amount)
        .sum()
}

fn main() {
    let matches = Command::new("AOC Day 01")
        .about("Elfs with calories")
        .arg(
            Arg::new("file")
            .short('f')
            .long("file")
            .required(true)
            .help("input file"))
        .get_matches();

    let input_file: &String = matches.get_one::<String>("file").expect("'file' is required");
    let content = std::fs::read_to_string(input_file).expect("Could not open file");

    let elfs: Vec<Elf> = get_elfs(&content);
    println!("{:?} Elfs", elfs.len());
    let max = sum_top_elfs(elfs, 3);
    println!("Sum of top three elfs: {:?}", max);
}

#[cfg(test)]
mod tests {
    use std::{str::FromStr};

    use crate::{Elf, get_elfs, sum_top_elfs};

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

    #[test]
    fn should_sum_top_one() {
        let input = "1\n2\n3\n\n4\n\n5\n6\n\n7\n8\n9\n\n10";
        let elfs = get_elfs(input);
        let sum = sum_top_elfs(elfs, 1);

        assert_eq!(sum, 24)
    }

    #[test]
    fn should_sum_top_three() {
        let input = "1\n2\n3\n\n4\n\n5\n6\n\n7\n8\n9\n\n10";
        let elfs = get_elfs(input);
        let sum = sum_top_elfs(elfs, 3);

        assert_eq!(sum, 45)
    }
}
