use crate::board::*;
use crate::heuristics::*;
use crate::min_heap::*;
use std::collections::*;
use std::time::{Duration, Instant};

/// Statistics of the search, used to evaluate the performance of the search algorithms.
/// Feel free to add more fields to this struct if you need them.
pub struct Stats {
    /// Numbers of states expanded during search
    pub expanded: usize,
    /// Total runtime spend in the search.
    ///
    /// ```rust
    // let start_time: Instant = std::time::Instant::now();
    /// // do something
   // let runtime: Duration = start_time.elapsed();
    /// ```
    pub runtime: Duration,
}

impl Stats {
    /// Creates a new `Stats` instance with the given expanded states count and runtime.
    pub fn new(expanded: usize, runtime: Duration) -> Stats {
        Stats { expanded, runtime }
    }
}


pub fn search(init_state: Board,heuristic: &Heuristic) -> (Option<Vec<Direction>>, Stats) {
    let start = Instant::now();
    let mut heap = MinHeap::new();
    let mut costs = HashMap::new();
    let mut came_from = HashMap::new(); // Stocke l'état précédent pour reconstruire le chemin
    let mut expanded = 0;

    let initial_cost = heuristic.estimate(&init_state);

    heap.insert(init_state.clone(), 0);
    costs.insert(init_state.clone(), initial_cost);
    came_from.insert(init_state.clone(), None);


    while let Some(state) = heap.pop() {
        if state == Board::GOAL {
            let mut path = Vec::new();
            let mut current = &state;
            while let Some(Some((prev, dir))) = came_from.get(current) {
                path.push(*dir);
                current = prev;
            }
            path.reverse();
            return (Some(path), Stats::new(expanded, start.elapsed()));
        }

        expanded += 1;
        let cost = costs[&state] + 1;

        for &dir in &DIRECTIONS {
            if let Some(new_state) = state.apply(dir) {
                let new_cost = cost + heuristic.estimate(&new_state);

                if !costs.contains_key(&new_state) || cost < costs[&new_state] {
                    costs.insert(new_state.clone(), new_cost);
                    came_from.insert(new_state.clone(), Some((state.clone(), dir)));
                    heap.insert(new_state, new_cost);
                }
            }
        }
    }


    (None, Stats::new(expanded, start.elapsed()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        for (expected_cost, init) in &INSTANCES[0..20] {
            let heuristic = Heuristic::Hamming;
            let (path, stats) = search(*init, &heuristic);
            let path = path.expect("no plan");
            assert!(init.is_valid_plan(&path));
            assert_eq!(path.len(), *expected_cost as usize);
        }
    }
}

