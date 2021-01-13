use crate::CellType::Square;
use ndarray::{Array, Array2, Axis};
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone)]
enum CellType {
    Tree,
    Square,
}

impl FromStr for CellType {
    type Err = std::string::ParseError;

    fn from_str(cell: &str) -> Result<Self, Self::Err> {
        cell.len();
        match cell {
            "." => Ok(Self::Square),
            "#" => Ok(Self::Tree),
            _ => panic!("unrecognized cell value {:?}!", cell),
        }
    }
}

impl Default for CellType {
    fn default() -> Self {
        Square
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Grid(Array2<CellType>);

impl Deref for Grid {
    type Target = Array2<CellType>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match Grid::from_str(input) {
            Ok(result) => Ok(Board::new(result)),
            Err(error) => Err(error),
        }
    }
}

impl FromStr for Grid {
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let assert_lines_2dimensional = |lines: &Vec<&str>| {
            assert!(lines.len() > 1, "need more than one line on grid");
            assert!(lines[0].len() > 1, "need more than one column on grid");
        };

        let get_ncols = |lines: &Vec<&str>| -> usize { lines[0].len() };

        let assert_all_lines_share_dimensions = |lines: &Vec<&str>| {
            let ncols = lines[0].len();
            assert!(
                lines.iter().all(|line| line.len() == ncols),
                "needs stable number of columns"
            );
        };

        let lines: Vec<&str> = input.lines().collect(); // needs to be mutable for size checks
        assert_lines_2dimensional(&lines);
        assert_all_lines_share_dimensions(&lines);
        let ncols = get_ncols(&lines);

        let mut grid = Array2::<CellType>::default((lines.len(), ncols));

        for (index, mut row) in grid.axis_iter_mut(Axis(ncols - 1)).enumerate() {
            let cells: Vec<CellType> = lines
                .get(index)
                .unwrap()
                .split_terminator("")
                .skip(1)
                .map(|f| CellType::from_str(f).unwrap())
                .collect::<Vec<CellType>>();
            // row = .view_mut();
            row.assign(&Array::from(cells));
        }

        Ok(Grid(grid))
    }
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        match Grid::from_str(input) {
            Ok(result) => result,
            Err(error) => panic!(error),
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::CellType::{Square, Tree};
    use crate::{CellType, Grid};
    use ndarray::{array};
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

        assert_eq!(result.0, array![[Square, Square], [Square, Square]]);
    }

    #[test]
    fn should_init_grid_with_only_trees() {
        let input = given_input("##\n##");
        let result = when_initializing_grid(input);

        assert_eq!(result.0, array![[Tree, Tree], [Tree, Tree]]);
    }

    #[test]
    fn should_init_grid_with_chess_pattern() {
        let input = given_input("#.\n.#");
        let result = when_initializing_grid(input);

        assert_eq!(result.0, array![[Tree, Square], [Square, Tree]]);
    }

    fn when_initializing_grid(input: &str) -> Grid {
        input.into()
    }

    fn given_input(input: &str) -> &str {
        input
    }

    #[test_case("-" => panics "unrecognized cell value \"-\"")]
    // #[test_case("/" => panics "unrecognized cell value /")] # Note: Due to code generation,
    // this would lead to a duplicated test name, all special signs leading to an exception need to
    // be tested separately. So, sadly, this was left as a warning.
    #[test_case("3" => panics "unrecognized cell value \"3\"")]
    #[test_case("a" => panics "unrecognized cell value \"a\"")]
    fn should_recognize_no_other_cell_types(input: &str) {
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
