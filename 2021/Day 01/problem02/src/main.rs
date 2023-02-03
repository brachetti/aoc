use std::env;
use std::fs;

const TARGET: usize = 2020;

fn get_numbers() -> Vec<usize> {
    let args: Vec<String> = env::args().collect();
    let input = args.get(1).expect("Missing commandline argument!");

    let contents = fs::read_to_string(input).expect("Could not read from file!");

    const ZERO: usize = 0;

    let mut numbers: Vec<usize> = contents
        .split(char::is_whitespace)
        .map(|s| s.parse::<usize>().unwrap_or(ZERO))
        .filter(|x| x > &ZERO)
        .filter(|x| x < &TARGET)
        .collect();

    numbers.sort_unstable();

    return numbers;
}

#[derive(Debug)]
struct SearchResult {
    target: usize,
    one: usize,
    two: usize,
    three: usize,
}

fn find_matching_triple(numbers: &Vec<usize>) -> SearchResult {
    const START: SearchResult = SearchResult {
        target: 0,
        one: 0,
        two: 0,
        three: 0,
    };

    numbers.iter().fold(START, |acc, element| {
        if acc.target == TARGET {
            return acc;
        }

        let pot_second_target: usize = TARGET - element;

        let result: SearchResult = find_matching_pair(numbers, pot_second_target);

        match result.target {
            0 => acc,
            _ => SearchResult {
                target: TARGET,
                one: element.clone(),
                two: result.two,
                three: result.three,
            },
        }
    })
}

fn find_matching_pair(numbers: &Vec<usize>, pair_target: usize) -> SearchResult {
    const START: SearchResult = SearchResult {
        target: 0,
        one: 0,
        two: 0,
        three: 0,
    };

    numbers.iter().fold(START, |acc, element| {
        if element > &pair_target || acc.target == pair_target {
            return acc;
        }

        let pot_right = pair_target - element;

        match numbers.binary_search(&pot_right) {
            Result::Ok(_) => SearchResult {
                target: pair_target,
                one: 0,
                two: element.clone(),
                three: pot_right,
            },
            Result::Err(_) => acc,
        }
    })
}

fn main() {
    let numbers: Vec<usize> = get_numbers();

    let matching = find_matching_triple(&numbers);

    println!("found matching triple {:?}", matching);
    println!("multiplied {:?}", matching.one * matching.two * matching.three);
}
