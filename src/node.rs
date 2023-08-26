//! This module contains the `Node` struct and  its implementation.

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
    /// the new Node just created
    pub fn new(value: usize, elt: char) -> Node {
        Node { value, elt }
    }

    /// Getter for the `elt` of the node.
    /// ## Retruns:
    /// the field `elt` of the node
    pub fn get_elt(self) -> char {
        self.elt
    }

    /// Getter for the `value` of the node.
    /// ## Retruns:
    /// the field `value` of the node
    pub fn get_value(self) -> usize {
        self.value
    }

    /// Adds one to the value of the node.
    /// In-place.
    pub fn add_one(&mut self) {
        self.value += 1
    }
}
