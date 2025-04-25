use std::fmt::{self, Display, Formatter};
use rand::prelude::*;



pub const N: usize = 4;
pub type Cell = u8;
pub const EMPTY_CELL: Cell = 0;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Board {
    cells: [[Cell; N]; N],
}

impl Board {




    //PARTIE ComplEMENTaire 
    pub fn random(shuffle_moves: usize) -> Self {
        let mut rng = thread_rng();
        let mut board = Board::GOAL;

        for _ in 0..shuffle_moves {
            // choisir un mouvement aléatoire applicable
            let applicable_moves: Vec<_> = DIRECTIONS
                .iter()
                .filter(|&&dir| board.apply(dir).is_some())
                .collect();
            
            if let Some(&&random_dir) = applicable_moves.choose(&mut rng) {
                board = board.apply(random_dir).unwrap();
            }
        }
        board
    }



    /// The goal state of the 8-puzzle problem.
    ///
    /// ```rust
    /// let goal: Board = Board::GOAL;
    /// assert_eq!(goal.value_at(0, 0), 1);
    /// assert_eq!(goal.value_at(2, 2), 0);
    /// ```
   /// État objectif pour 4×4
   pub const GOAL: Board = Board::new([
    [ 1,  2,  3,  4],
    [ 5,  6,  7,  8],
    [ 9, 10, 11, 12],
    [13, 14, 15,  0],
]);

    pub const fn new(cells: [[Cell; N]; N]) -> Board {
        Board { cells }
    }

    /// Returns the value of the cell at the given position.
    pub fn value_at(&self, line: usize, column: usize) -> Cell {
        self.cells[line][column]
    }

    /// Returns the result of applying the given action to the board.
    /// If the action is not applicable (the empty cell would move outside the board), returns `None`.
    /// Otherwise, returns the new board (wrapped in `Some(...)`).
    pub fn apply(&self, dir: Direction) -> Option<Board> {
        let (x, y) = self.position(EMPTY_CELL);
        let new_pos = match dir {
            Direction::Up if x > 0           => Some((x-1, y)),
            Direction::Down if x < N - 1     => Some((x+1, y)),
            Direction::Left if y > 0         => Some((x, y-1)),
            Direction::Right if y < N - 1    => Some((x, y+1)),
            _ => None
        };
        new_pos.map(|(nx, ny)| {
            let mut c = self.cells;
            c[x][y] = c[nx][ny];
            c[nx][ny] = EMPTY_CELL;
            Board::new(c)
        })
    }
    /// Returns the position `(line, column)` of the given cell value.
    pub fn position(&self, value: Cell) -> (usize, usize) {
        for x in 0..N {
            for y in 0..N {
                if self.cells[x][y] == value {
                    return (x, y);
                }
            }
        }
        panic!("No such cell: {value}");
    }

    /// Plays a sequence of moves on the board, printing the board at each step.
    /// Intended for displaying purpose but very slow (the thread will be put to sleep between each frame)
    pub fn play(&self, moves: &[Direction]) {
        // current board from which the play starts
        let mut current_board = self.clone();
        println!("{current_board}");
        for &direction in moves {
            if let Some(next) = current_board.apply(direction) {
                // action is applicable, update the current board
                current_board = next;

                // wait half a second before displaying
                std::thread::sleep(std::time::Duration::from_millis(500));
                // print the current board
                println!("{current_board}");
            } else {
                // non-applicable action, call `panic!()` which represents an unrecoverable error
                panic!(
                    "The action {direction:?} is not applicable on the board: {current_board:?}"
                );
            }
        }
    }

    /// Returs `true` if the given sequence of actions is a valid plan that leads to the goal state.
    pub fn is_valid_plan(&self, actions: &[Direction]) -> bool {
        let mut board = *self;

        for &act in actions {
            if let Some(new_board) = board.apply(act) {
                board = new_board;
            }
            else {
                return false ;
            }
        }
        board == Board::GOAL
    }


}

// Specifies how to display a board in a human-readable way.
// This is what is used when you use the `{}` format specifier in a `println!` macro.
impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Determine cell width based on max value (N*N-1)
        let max_val = (N * N - 1) as u32;
        let digit_width = max_val.to_string().len();
        // add padding of 2 (one space each side)
        let cell_width = digit_width + 2;

        // Top border
        write!(f, "┏")?;
        for col in 0..N {
            write!(f, "{}", "━".repeat(cell_width))?;
            write!(f, "{}", if col + 1 < N { "┳" } else { "┓\n" })?;
        }

        // Rows
        for row in 0..N {
            // Cell line
            write!(f, "┃")?;
            for col in 0..N {
                let v = self.value_at(row, col);
                let content = if v == EMPTY_CELL {
                    " ".repeat(cell_width)
                } else {
                    let s = v.to_string();
                    let padding = cell_width - s.len();
                    format!("{}{}", " ".repeat(padding), s)
                };
                write!(f, "{}┃", content)?;
            }
            write!(f, "\n")?;

            // Separator or bottom border
            if row + 1 < N {
                write!(f, "┣")?;
                for col in 0..N {
                    write!(f, "{}", "━".repeat(cell_width))?;
                    write!(f, "{}", if col + 1 < N { "╋" } else { "┫\n" })?;
                }
            } else {
                write!(f, "┗")?;
                for col in 0..N {
                    write!(f, "{}", "━".repeat(cell_width))?;
                    write!(f, "{}", if col + 1 < N { "┻" } else { "┛\n" })?;
                }
            }
        }

        Ok(())
    }
}

/// The possible directions to move the empty cell.
///
/// A direction is *one of* `Up`, `Down`, `Left` or `Right`.
#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    /// Returns the opposite direction of the current one.
    ///
    /// ```rust
    /// assert_eq!(Direction::Up.opposite(), Direction::Down);
    /// ```
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

// Implements prettry printing for the `Direction` enum.
// This is what is used when you use the `{}` format specifier in a `println!` macro.
impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Up => "↑",
                Direction::Down => "↓",
                Direction::Left => "←",
                Direction::Right => "→",
            }
        )
    }
}

/// An iterable sequence of all possible directions.
///
/// ```rust
/// for direction in &DIRECTIONS {
///    println!("{direction:?}");
/// }
/// ```
pub const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];
