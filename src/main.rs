mod board;
mod heuristics;
mod min_heap;
mod search;

use board::Board;
use heuristics::Heuristic;
use search::{ida_star, beam_search,search};

fn main() {
    let initial = Board::random(60);
    println!("Board initial :\n{}", initial);

    println!("----------------------*********----------------------");
    // let (plan, stats) = search(initial.clone(), &Heuristic::Manhattan);

    // IDA*
    if let Some(plan) = ida_star(initial.clone(), &Heuristic::Hamming) {
        println!("Yess !!!  IDA*  a trouvé une solution en {} coups", plan.len());
        initial.play(&plan);
    } else {
        println!("IDA* n'a pas trouvé de solution");
    }
println!("----------------------*********----------------------");
    // Beam Search (beam_width=100, max_depth=50)
    if let Some(plan) = beam_search(initial, &Heuristic::Manhattan, 100, 50) {
        println!("Yess, Beam Search a trové une solution en {} coups", plan.len());
    } else {
        println!("Beam Search n'a pas trouvé de solution");
    }
}
