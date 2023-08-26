//! This module contains some structs and implementations that will be useful
//! for the whole project.

use crate::node::Node;

/// Stores the nodes forming the heap.
pub struct Heap {
    /// The nodes of the heap.
    pub nodes: Vec<Option<Node>>,
}

/// Implementation containing methods to play with heaps.
impl Heap {
    /// Allows to create a new heap and returns it.
    /// The heap created is just a `vec` of `None` nodes.
    pub fn new() -> Heap {
        Heap { nodes: vec![None] }
    }

    /// Checks if a heap is empty or bot.
    // #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.nodes.len() == 1 && self.nodes[0].is_none()
    }

    /// Add an element at the right place in the heap.
    /// ## Arguments:
    /// - `elt`: The element to push in the heap.
    /// ## Returns:
    /// The updated heap
    pub fn push(&mut self, elt: Node) -> &Heap {
        self.nodes.push(Some(elt));
        let mut index: usize = self.nodes.len() - 1;

        while (index > 1) && elt.get_value() < self.nodes[index / 2].unwrap().get_value() {
            (self.nodes[index], self.nodes[index / 2]) = (self.nodes[index / 2], self.nodes[index]);
            index /= 2;
        }

        self
    }

    /// Removes and return the first element of the heap.
    /// ## Raises:
    /// - "The heap is empty" if the heap is empty
    /// ## Returns:
    /// the first `Node` of the heap.
    pub fn pop(&mut self) -> Result<Node, String> {
        if self.is_empty() {
            Err("The heap is empty".to_string())
        } else {
            let elt: Node = self.nodes[1].unwrap();
            self.nodes[1] = self.nodes[self.nodes.len() - 1];
            self.nodes.pop();
            let length: usize = self.nodes.len() - 1;
            let mut done: bool = false;
            let mut index: usize = 1;

            while (index <= length / 2) && !done {
                let mut jndex: usize = index * 2;
                if (jndex < length)
                    && (self.nodes[jndex + 1].unwrap().get_value()
                        < self.nodes[jndex].unwrap().get_value())
                {
                    jndex += 1;
                }
                if self.nodes[index].unwrap().get_value() > self.nodes[jndex].unwrap().get_value() {
                    (self.nodes[index], self.nodes[jndex]) = (self.nodes[jndex], self.nodes[index]);
                    index = jndex;
                } else {
                    done = true;
                }
            }

            Ok(elt)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let heap: Heap = Heap::new();
        assert_eq!(heap.nodes[0], None);
    }

    #[test]
    fn test_is_empty() {
        let heap: Heap = Heap::new();
        assert!(heap.is_empty());
    }

    #[test]
    fn test_is_not_empty() {
        let mut heap: Heap = Heap::new();
        let new_heap: &Heap = heap.push(Node::new(1, 'a'));
        assert!(!new_heap.is_empty());
    }

    #[test]
    fn test_push_first() {
        let mut heap: Heap = Heap::new();
        let new_heap: &Heap = heap.push(Node::new(1, 'a'));
        assert_eq!(new_heap.nodes, vec![None, Some(Node::new(1, 'a'))]);
    }

    #[test]
    fn test_push_last() {
        let mut heap: Heap = Heap::new();
        let _tmp: &Heap = heap.push(Node::new(1, 'a'));
        let new_heap: &Heap = heap.push(Node::new(2, 'b'));
        assert_eq!(
            new_heap.nodes,
            vec![None, Some(Node::new(1, 'a')), Some(Node::new(2, 'b'))]
        );
    }

    #[test]
    fn test_push_to_first() {
        let mut heap: Heap = Heap::new();
        let _tmp: &Heap = heap.push(Node::new(2, 'b'));
        let new_heap: &Heap = heap.push(Node::new(1, 'a'));
        assert_eq!(
            new_heap.nodes,
            vec![None, Some(Node::new(1, 'a')), Some(Node::new(2, 'b'))]
        );
    }

    #[test]
    fn test_pop_empty_heap() {
        let mut heap: Heap = Heap::new();
        assert!(heap.pop().is_err());
    }

    #[test]
    fn test_pop() {
        let mut heap: Heap = Heap::new();
        let _tmp: &Heap = heap.push(Node::new(2, 'b'));
        let _new_heap: &Heap = heap.push(Node::new(1, 'a'));
        assert_eq!(heap.pop().ok(), Some(Node::new(1, 'a')));
        assert_eq!(heap.nodes, vec![None, Some(Node::new(2, 'b'))]);
    }
}
