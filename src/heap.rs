//! This file contains some structs and implementations that will be useful
//! for the whole project.

/// An element of the heap, defined by its value and its elt.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Node {
    /// For priority in the heap
    value: usize,

    /// The stored value (here a char)
    elt: char,
}

/// Implementation containing methods to use `Node` struct.
impl Node {
    /// Allows to create a new node for the heap and returns it.
    /// ## Arguements:
    /// - `value`
    /// - `elt`
    /// ## Returns:
    /// - the new Node just created
    pub fn new(value: usize, elt: char) -> Node {
        Node { value, elt }
    }
}

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
    /// ## Return Type:
    /// - `bool`
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.nodes.len() == 1 && self.nodes[0].is_none()
    }

    /// Add an element at the right place in the heap.
    /// ## Arguments:
    /// - `elt`: The element to push in the heap.
    /// ## Returns:
    /// - The updated heap
    /// ## Return Type:
    /// - `Heap`
    pub fn push(&mut self, elt: Node) -> &Heap {
        self.nodes.push(Some(elt));
        let mut index: usize = self.nodes.len() - 1;
        while (index > 1) && elt.value < self.nodes[index / 2].unwrap().value {
            (self.nodes[index], self.nodes[index / 2]) = (self.nodes[index / 2], self.nodes[index]);
            index /= 2;
        }
        self
    }

    // pub fn pop(&mut self) -> Result<Node, String> {
    //     if self.is_empty() {
    //         Err("The heap is empty".to_string())
    //     } else {
    //         Ok()
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let heap = Heap::new();
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
}
