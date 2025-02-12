#![allow(unused)] // suppress warnings for unused code (there is plenty when you start)

// declare other modules that are in other files and must be compiled
mod board;
mod heuristics;
mod min_heap;
mod search;

// import the content of the modules
use board::*;
use heuristics::*;
use search::*;
/// Represents an empty cell in the board.
pub const EMPTY_CELL: Cell = 0;

fn main() {
    // 1. Créer le plateau initial
    let initial_board = Board::new([[1, 2, 3], [4, 8, 5], [EMPTY_CELL, 7, 6]]);

    use Direction :: * ;
    let actions = [Right,Up,Right,Down];


    initial_board.play(&actions);

    if  initial_board.is_valid_plan(&actions) {
        println!("Yiiiiikes, ur Actions work !");
    } else
    {
        println!("Naaah Hoomie, try again ");

    }


}
