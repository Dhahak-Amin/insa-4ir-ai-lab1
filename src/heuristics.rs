use crate::board::*;

/// A heuristic function to estimate the cost of reaching the goal state from a given board.
///
/// ```rust
/// let board = Board::new([[8, 7, 3], [2, 0, 5], [1, 4, 6]]);
/// let h = Heuristic::Manhattan.estimate(&board);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Heuristic {
    /// The blind heuristic always returns 0.
    Blind,
    /// The Hamming heuristic, which counts the number of misplaced tiles.
    Hamming,
    /// The Manhattan heuristic, which computes the sum of the Manhattan distances of each tile to its goal position.
    Manhattan,
}

impl Heuristic {
    pub fn estimate(&self, board: &Board) -> u32 {
        match self {
            Heuristic::Blind => 0,
            Heuristic::Hamming => {
                let mut count = 0;
                for i in 0..N {
                    for j in 0..N {
                        let value = board.value_at(i, j);
                        if value != 0 &&  value != Board::GOAL.value_at(i, j) {
                            count += 1;
                        }
                    }
                }
                count
            }
            Heuristic::Manhattan => {
                let mut distance = 0;
                for i in 0..N {
                    for j in 0..N {
                        let value = board.value_at(i, j);
                        if value != 0 {
                            let (goal_x, goal_y) = Board::GOAL.position(value);
                            distance += (i as i32 - goal_x as i32).abs() as u32;
                            distance += (j as i32 - goal_y as i32).abs() as u32;
                        }
                    }
                }
                distance
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_heuristic() {
        use super::*;
        let board = Board::new([[8, 7, 3], [2, 0, 5], [1, 4, 6]]);
        assert_eq!(Heuristic::Blind.estimate(&board), 0);
        assert_eq!(Heuristic::Hamming.estimate(&board), 7);
        assert_eq!(Heuristic::Manhattan.estimate(&board), 14);
    }
}
