use ndarray::{Array, Array2, Array1, array};
use std::ops::Deref;
use std::str::FromStr;
use crate::CellType::{Tree, Square};
use clap::{App, Arg};

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

impl Grid {
    pub(crate) fn cell_at(&self, row: usize, col: usize) -> Option<&CellType> {
        self.get((row, col))
    }
}

#[derive(Debug)]
struct Board {
    contents: Grid,
    move_pattern: Array1<usize>,
    current_position: Array1<usize>,
}

impl Board {
    pub fn new(contents: Grid, mp: MovePattern) -> Self {
        Board {
            contents,
            move_pattern: array![mp.rows, mp.cols],
            current_position: array![0, 0]
        }
    }

    pub fn set_move_pattern(&mut self, mp: MovePattern) {
        self.move_pattern = array![mp.rows, mp.cols]
    }

    pub(crate) fn calculate_collisions(&mut self) -> usize {
        let mut collisions = 0;

        while self.current_row() != self.grid_rows() {
            self.move_position();
            if self.hit_a_tree() {
                collisions += 1;
            }
        }

        collisions
    }

    fn hit_a_tree(&self) -> bool {
        let row = self.current_row() % self.grid_rows();
        let col = self.current_col() % self.grid_cols();
        match self.contents.cell_at(row, col) {
            None => panic!("Position does not exist m("),
            Some(cell) if *cell == Tree => true,
            Some(_) => false,
        }
    }

    fn move_position(&mut self) {
        self.current_position += &self.move_pattern;
    }

    fn current_row(&self) -> usize {
        *self.current_position.get(0).unwrap()
    }

    fn current_col(&self) -> usize {
        *self.current_position.get(1).unwrap()
    }

    fn grid_rows(&self) -> usize {
        self.contents.nrows()
    }

    fn grid_cols(&self) -> usize {
        self.contents.ncols()
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
    let matches = App::new("AOC Day 3")
        .about("Calculates collisions in a forest")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .takes_value(true)
                .required(true)
                .about("the input file"))
        .get_matches();

    let input_file = matches.value_of("file").unwrap();
    let content = std::fs::read_to_string(input_file).expect("Could not open file");

    let mut board = Board::from_str(content.as_str()).expect("Could not init Board");
    board.set_move_pattern(MovePattern { rows: 1, cols: 3 });

    println!("Amount of collisions on the way {}", board.calculate_collisions());
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
    fn should_build_board_with_move_pattern() {
        let grid = given_aoc_grid();
        when_initializing_board(grid, AOC_MOVE_PATTERN);
        () // all is well
    }

    #[test]
    fn should_calculating_aoc_move_pattern() {
        let board = when_initializing_board(given_aoc_grid(), AOC_MOVE_PATTERN);
        let trees_hit = when_calculating_collisions(board);

        assert_eq!(trees_hit, 7)
    }

    #[test]
    fn should_add_to_position() {
        let mut left = array![0,0];
        let right = array![1, 3];

        left += &right;
        left += &right;

        assert_eq!(left, array![2, 6])
    }

    fn when_calculating_collisions(mut board: Board) -> usize {
        board.calculate_collisions()
    }

    fn when_initializing_board(grid: Grid, mp: MovePattern) -> Board {
       Board::new(grid, mp)
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
