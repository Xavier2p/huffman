//! This module contains the `Tree` struct and implementation.

/// Struct of the Tree
#[allow(dead_code)]
pub struct Tree {
    /// The key of the node, here a char.
    key: char,

    /// Link to the left tree.
    left: Option<Box<Tree>>,

    /// Link to the right tree.
    right: Option<Box<Tree>>,
}

#[allow(dead_code)]
impl Tree {
    /// Allows to create a new Tree.
    pub fn new(key: char, left: Option<Box<Tree>>, right: Option<Box<Tree>>) -> Tree {
        Tree { key, left, right }
    }

    /// Returns the key of the tree.
    pub fn get_key(self) -> char {
        self.key
    }
}

#[cfg(test)]
mod tests {}
