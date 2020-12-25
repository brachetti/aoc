use std::env;
use std::fs;

const TARGET: usize = 2020;

fn get_numbers() -> Vec<usize> {
    let args: Vec<String> = env::args().collect();
    let input = args.get(1).expect("Missing commandline argument!");

    let contents = fs::read_to_string(input)
        .expect("Could not read from file!");

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
    left: usize,
    right: usize,
}

fn find_matching_pair(numbers: &Vec<usize>) -> SearchResult {
    const START: SearchResult = SearchResult { target: 0, left: 0, right: 0 };

    numbers.iter().fold(START, |acc, element| {
        match acc.target {
            TARGET => acc,
            _ => {
                let pot_right = TARGET - element;
                match numbers.binary_search(&pot_right) {
                    Result::Ok(_) => SearchResult {
                        target: TARGET,
                        left: element.clone(),
                        right: pot_right,
                    },
                    Result::Err(_) => acc,
                }
            }
        }
    }, )
}

fn main() {
    let numbers: Vec<usize> = get_numbers();

    let matching = find_matching_pair(&numbers);

    println!("{:?}", matching.left * matching.right);
}
