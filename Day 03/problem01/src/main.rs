use std::str::{FromStr};
use ndarray::Array2;

#[derive(Debug, Eq, PartialEq)]
enum CellType {
    Tree, Square
}

impl FromStr for CellType {
    type Err = std::string::ParseError;

    fn from_str(cell: &str) -> Result<Self, Self::Err> {
        cell.len();
        match cell {
            "." => Ok(Self::Square),
            "#" => Ok(Self::Tree),
            _ => panic!("unrecognized cell value {}", cell),
        }
    }
}

type Grid = Array2<CellType>;

#[derive(Debug)]
struct Board {
    contents: Grid,
}

impl Board {
    pub fn new(contents: Grid) -> Self {
        Board { contents }
    }
}

impl FromStr for Board {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match Grid::from_str(input) {
            Ok(T) => Ok(Board::new(T)),
            Err(E) => Err(E)
        }
    }
}

impl FromStr for Grid {
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let assert_lines_2dimensional = |lines: &Vec<str>| {
            assert!(lines.count() > 1, "need more than one line on grid");
            assert!(lines.get(0).len() > 1, "need more than one column on grid");
        };

        let get_ncols = |lines: &Vec<str>| -> usize {
            lines.get(0).len()
        };

        let assert_all_lines_share_dimensions = |lines: &mut Vec<str>| {
            let ncols = lines.get(0).len();
            assert!(lines.all(|line: &str| line.len() == ncols), "needs stable number of columns");
        };

        let mut lines : Vec<str> = input.lines().unwrap(); // needs to be mutable for size checks
        assert_lines_2dimensional(&lines);
        assert_all_lines_share_dimensions(&mut lines);
        let ncols = get_ncols(&lines);
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::{CellType, Grid};
    use crate::CellType::{Tree, Square};
    use std::str::FromStr;
    use test_case::test_case;

    #[test]
    fn should_recognize_tree() {
        let cell_type: CellType = CellType::from_str("#").unwrap();

        assert_eq!(Tree, cell_type);
    }

    #[test]
    fn should_recognize_square() {
        let cell_type: CellType = CellType::from_str(".").unwrap();

        assert_eq!(Square, cell_type);
    }

    #[test]
    fn should_init_grid() {
        let input = given_input("..\n..");
        let result = when_initializing_grid(input);
    }

    fn when_initializing_grid(input: &str) -> Grid {
        input.into()
    }

    fn given_input(input: &str) -> &str {
        input
    }

    #[test_case("-" => panics "unrecognized cell value -")]
    // #[test_case("/" => panics "unrecognized cell value /")] # Note: Due to code generation,
    // this would lead to a duplicated test name, all special signs leading to an exception need to
    // be tested separately. So, sadly, this was left as a warning.
    #[test_case("3" => panics "unrecognized cell value 3")]
    #[test_case("a" => panics "unrecognized cell value a")]
    fn should_recognize_no_other_cell_types(input : &str) {
        CellType::from_str(input).unwrap();
    }

//
//     const AOC_EXAMPLE: String = String::from("\
// ..##.......
// #...#...#..
// .#....#..#.
// ..#.#...#.#
// .#...##..#.
// ..#.##.....
// .#.#.#....#
// .#........#
// #.##...#...
// #...##....#
// .#..#...#.#");
//
//     fn given_input() -> String {
//         AOC_EXAMPLE
//     }
}