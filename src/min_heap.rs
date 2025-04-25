use std::{cmp::Reverse, collections::BinaryHeap};

/// A very straightforward implementation of a min-heap.
///
///
/// ```rust
/// // example usage
/// let mut heap = MinHeap::new();
/// heap.insert("two", 2);
/// heap.insert("three", 3);
/// heap.insert("five", 5);
/// assert_eq!(heap.len(), 3);
/// assert_eq!(heap.pop(), Some("two"));
/// assert_eq!(heap.pop(), Some("three"));
/// assert_eq!(heap.pop(), Some("five"));
/// assert_eq!(heap.pop(), None);
/// ```
pub struct MinHeap<State> {
    heap: BinaryHeap<Node<State>>,
}

impl<State: Ord> MinHeap<State> {
    /// Create a new empty MinHeap.
    pub fn new() -> MinHeap<State> {
        MinHeap {
            heap: BinaryHeap::new(),
        }
    }

    /// Insert a new state in the heap with the given f-value.
    /// Note that a state made be inserted multiple times with different f-values and may do thus appear multiple times in the heap.
    pub fn insert(&mut self, state: State, f_value: u32) {
        self.heap.push(Node {
            priority: Reverse(f_value),
            state,
        });
    }

    /// Returns the state with the smallest f-value from the heap.
    /// Returns `None` if the heap is empty.
    pub fn pop(&mut self) -> Option<State> {
        self.heap.pop().map(|node| node.state)
    }

    /// Returns `true` if the heap is empty.
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// Returns true
    pub fn len(&self) -> usize {
        self.heap.len()
    }
}

#[derive(Eq, PartialEq, PartialOrd, Ord)]
struct Node<State> {
    priority: Reverse<u32>,
    state: State,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_min_heap() {
        use super::*;

        let mut heap = MinHeap::new();
        heap.insert("two", 2);
        heap.insert("three", 3);
        heap.insert("five", 5);
        heap.insert("one", 1);
        heap.insert("four", 4);

        assert_eq!(heap.len(), 5);
        assert_eq!(heap.pop(), Some("one"));
        assert_eq!(heap.pop(), Some("two"));
        assert_eq!(heap.pop(), Some("three"));
        assert_eq!(heap.pop(), Some("four"));
        assert_eq!(heap.pop(), Some("five"));
        assert_eq!(heap.pop(), None);
        assert!(heap.is_empty());
        assert_eq!(heap.len(), 0);
    }
}
