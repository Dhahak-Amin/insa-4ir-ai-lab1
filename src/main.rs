// exemple d'utilisation dans main.rs
mod board;
mod heuristics;
mod min_heap;
mod search;

use board::Board;
use heuristics::Heuristic;
use search::{ida_star, beam_search};

fn main() {
    let initial = Board::random(30);
    println!("Board initial :\n{}", initial);

    // A* (si mémoire disponible)
    // let (plan, stats) = search(initial.clone(), &Heuristic::Manhattan);

    // IDA*
    if let Some(plan) = ida_star(initial.clone(), &Heuristic::Manhattan) {
        println!("Yess IDA* solution en {} coups", plan.len());
        initial.play(&plan);
    } else {
        println!("IDA* n'a pas trouvé de solution");
    }

    // Beam Search (beam_width=100, max_depth=50)
    if let Some(plan) = beam_search(initial, &Heuristic::Manhattan, 100, 50) {
        println!("Yess, Beam Search a trové une solution en {} coups", plan.len());
        // initial.play(&plan); // ou joue si tu veux
    } else {
        println!("Beam Search n'a pas trouvé de solution");
    }
}
