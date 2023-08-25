//! This file contains some structs and implementations that will be useful
//! for the whole project.
struct BinTree {
    left: Option<Box<BinTree>>,
    right: Option<Box<BinTree>>,
    value: i32,
}

/// Stores the elements forming the heap.
struct Heap {
    elements: Vec<Option<Element>>,
}

/// An element of the heap, defined by its value and its elt.
struct Element {
    value: usize,
    elt: char,
}

/// Implementation containing methods to play with heaps.
impl Heap {
    /// Allows to create a new heap and returns it.
    /// The heap created is just a `vec` of `None` elements.
    fn new() -> Heap {
        Heap {
            elements: vec![None],
        }
    }

    /// Checks if a heap is empty or bot.
    /// ## Return Type:
    /// - `bool`
    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Add an element at the right place in the heap.
    /// ## Arguments:
    /// - `elt`: The element to push in the heap.
    /// ## Returns:
    /// - The updated heap
    /// ## Return Type:
    /// - `Heap`
    fn push(&mut self, elt: Element) -> &Heap {
        self.elements.push(Some(elt));
        let mut index: usize = self.elements.len() - 1;
        while (index > 1) && elt.value < self.elements[index / 2].unwrap().value {}
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let heap = Heap::new();
    }
}
