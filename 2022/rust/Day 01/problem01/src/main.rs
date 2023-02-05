use std::{str::FromStr, usize};

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

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::{str::FromStr};

    use crate::Elf;

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
}
