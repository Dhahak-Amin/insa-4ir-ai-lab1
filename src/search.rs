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

/// IDA* search: memory-efficient depth-first iterative deepening A*.
pub fn ida_star(init: Board, heuristic: &Heuristic) -> Option<Vec<Direction>> {
    /// Performs a depth-first search bounded by a threshold.
    /// Returns Err(solution_path) if found, or Ok(min_overrun) if not.
    fn dfs(
        board: &Board,
        g: u32,
        threshold: u32,
        heuristic: &Heuristic,
        path: &mut Vec<Direction>,
        visited: &mut HashSet<Board>,
    ) -> Result<u32, Vec<Direction>> {
        let h = heuristic.estimate(board);
        let f = g + h;
        if f > threshold {
            return Ok(f);
        }
        if *board == Board::GOAL {
            return Err(path.clone());
        }
        let mut min_over = u32::MAX;
        for &dir in &DIRECTIONS {
            if let Some(next) = board.apply(dir) {
                if visited.insert(next.clone()) {
                    path.push(dir);
                    match dfs(&next, g + 1, threshold, heuristic, path, visited) {
                        Err(sol) => return Err(sol),
                        Ok(t) if t < min_over => min_over = t,
                        _ => {}
                    }
                    path.pop();
                    visited.remove(&next);
                }
            }
        }
        Ok(min_over)
    }

    let mut threshold = heuristic.estimate(&init);
    let mut path = Vec::new();
    let mut visited = HashSet::new();
    visited.insert(init.clone());

    loop {
        match dfs(&init, 0, threshold, heuristic, &mut path, &mut visited) {
            Err(solution) => return Some(solution),
            Ok(next_threshold) if next_threshold == u32::MAX => return None,
            Ok(next_threshold) => threshold = next_threshold,
        }
    }
}

// Beam search: memory-limited heuristic search with fixed beam width.
pub fn beam_search(
    init: Board,
    heuristic: &Heuristic,
    beam_width: usize,
    max_depth: usize,
) -> Option<Vec<Direction>> {
    // Each entry: (state, path, g_cost)
    let mut frontier: Vec<(Board, Vec<Direction>, u32)> = vec![(init.clone(), Vec::new(), 0)];

    for depth in 0..=max_depth {
        // Check if any in frontier is goal
        for (state, path, _) in &frontier {
            if *state == Board::GOAL {
                return Some(path.clone());
            }
        }
        // Generate candidates
        let mut candidates = Vec::new();
        for (state, path, g) in &frontier {
            for &dir in &DIRECTIONS {
                if let Some(next) = state.apply(dir) {
                    let mut new_path = path.clone();
                    new_path.push(dir);
                    let g_next = g + 1;
                    let f = g_next + heuristic.estimate(&next);
                    candidates.push((next, new_path, g_next, f));
                }
            }
        }
        if candidates.is_empty() {
            break;
        }
        // Keep top beam_width by f
        candidates.sort_by_key(|c| c.3);
        frontier = candidates
            .into_iter()
            .take(beam_width)
            .map(|(s, p, g, _)| (s, p, g))
            .collect();
    }
    None
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;

    #[test]
    fn test_search_basic() {
        let board = Board::new([
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 0, 14, 15],
        ]);
        let opt_plan = ida_star(board, &Heuristic::Manhattan);
        let plan = opt_plan.expect("IDA* should solve simple instance");
        assert_eq!(plan.len(), 2);
    }

    #[test]
    fn test_beam_search_basic() {
        let board = Board::new([
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 0, 14, 15],
        ]);
        let plan = beam_search(board, &Heuristic::Manhattan, 2, 5)
            .expect("Beam search should solve simple instance");
        assert_eq!(plan.len(), 2);
    }
}

