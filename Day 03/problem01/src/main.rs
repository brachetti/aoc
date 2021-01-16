use ndarray::{Array, Array2};
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
        CellType::Square
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

impl Grid {}

#[derive(Debug)]
struct Board {
    contents: Grid,
    move_pattern: MovePattern,
}

impl Board {
    pub fn new(contents: Grid, move_pattern: MovePattern) -> Self {
        Board { contents, move_pattern }
    }
}

impl FromStr for Board {
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match Grid::from_str(input) {
            Ok(result) => Ok(Board::new(result, MovePattern::default())),
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
                "needs stable number of columns ({})",
                ncols
            );
        };

        let lines: Vec<&str> = input
            .lines()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .collect(); // needs to be mutable for size checks
        assert_lines_2dimensional(&lines);
        assert_all_lines_share_dimensions(&lines);
        let ncols = get_ncols(&lines);

        let mut grid = Array2::<CellType>::default((lines.len(), ncols));

        for (index, mut row) in grid.outer_iter_mut().enumerate() {
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

#[derive(Copy, Clone, Debug, PartialEq)]
struct MovePattern {
    rows: usize,
    cols: usize,
}

impl Default for MovePattern {
    fn default() -> Self {
        MovePattern { rows: 1, cols: 1 }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::CellType::{Square, Tree};
    use crate::{CellType, Grid, MovePattern, Board};
    use ndarray::{array, ArrayBase, OwnedRepr, Ix1};
    use once_cell::sync::OnceCell;
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

    #[test]
    fn should_handle_aoc_grid_without_error() {
        let input = given_aoc_input();
        let grid = when_initializing_grid(input);

        then_first_line_is(grid, array![
            Square, Square, Tree, Tree, Square, Square, Square, Square, Square, Square, Square
        ])
    }

    #[test]
    fn should_compute_board_with_move_pattern() {
        let grid = given_aoc_grid();
        let board = when_initializing_board(grid, AOC_MOVE_PATTERN);
        ()
    }

    fn when_initializing_board(grid: Grid, mp: MovePattern) {
        let board = Board::new(grid, mp);
    }

    fn given_aoc_grid() -> Grid {
        let input = given_aoc_input();
        when_initializing_grid(input)
    }

    fn then_first_line_is(grid : Grid, expected: ArrayBase<OwnedRepr<CellType>, Ix1>) {
        assert_eq!(
            grid.row(0),
            expected,
            "First line does not match"
        )
    }

    fn given_aoc_input() -> &'static String {
        static AOC_EXAMPLE: OnceCell<String> = OnceCell::new();
        AOC_EXAMPLE.get_or_init(|| {
            String::from(
                "..##.......\n
                    #...#...#..\n
                    .#....#..#.\n
                    ..#.#...#.#\n
                    .#...##..#.\n
                    ..#.##.....\n
                    .#.#.#....#\n
                    .#........#\n
                    #.##...#...\n
                    #...##....#\n
                    .#..#...#.#",
            )
        })
    }

    const AOC_MOVE_PATTERN : MovePattern = MovePattern { rows: 1, cols: 3 };
}
