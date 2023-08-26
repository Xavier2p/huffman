//! This module contains all the function to compress a text using Huffman.

use crate::{heap::Heap, node::Node, tree::Tree};

/// Builds a `Node` list of the character frequencies in the input.
///
/// ## Arguments:
/// - `data`: the data from which we want to build the frequency list.
///
/// ## Returns:
/// the list of frequencies: each element is a tuple where first element
/// is the frequency of the character stored in the second part.
fn build_frequency_list(data: &str) -> Vec<Node<char>> {
    let mut result: Vec<Node<char>> = Vec::new();

    for c in data.chars() {
        let mut found: bool = false;

        for item in &mut result {
            if c == item.get_elt() {
                item.add_one();
                found = true;
            }
        }

        if !found {
            result.push(Node::new(1, c));
        }
    }

    heap_sort(result)
}

/// Helper who sort the list in the right order.
fn heap_sort<T: std::clone::Clone>(list: Vec<Node<T>>) -> Vec<Node<T>> {
    let mut tmp: Heap<T> = Heap::new();
    let mut result: Vec<Node<T>> = Vec::new();

    for value in list {
        tmp.push(value);
    }

    while !tmp.is_empty() {
        result.push(tmp.pop().unwrap());
    }

    result
}

/// Processes the frequency list into a Huffman tree according to the algorithm.
///
/// ## Arguments:
/// - `list`: the frequency list from `build_frequency_list()`.
///
/// ## Returns:
/// a huffman tree containing all the characters from the list in leaves.
fn build_huffman_tree(list: Vec<Node<char>>) -> Tree {
    let mut trees: Vec<Node<Tree>> = Vec::new();
    for item in list {
        trees.push(Node::new(
            item.get_value(),
            Tree::new(item.get_elt(), None, None),
        ));
    }

    while trees.len() != 1 {
        // trees = heap_sort();
    }

    todo!()
}

/// Encodes the input string to its binary string representation.
///
/// ## Arguments:
/// - `tree`: the huffman tree from `build_huffman_tree()`.
/// - `data`: the data we want to encode.
///
/// ## Returns:
/// the binary string.
fn encode_data(_tree: Tree, _data: &str) -> &str {
    todo!()
}

/// Encodes a huffman tree to its binary representation using a preOrder traversal:
/// - each leaf key is encoded into its binary representation on 8 bits preceded by '1'
/// - each time we go left we add a '0' to the result
///
/// ## Arguments:
/// - `tree`: the huffman tree to encode.
///
/// ## Returns:
/// a string corresponding to the binary representation of the huffman tree.
fn encode_tree(_tree: Tree) -> &'static str {
    todo!()
}

/// Compresses a string containing binary code to its real binary value.
///
/// ## Arguments:
/// - `data`: the data to compress.
///
/// ## Returns:
/// a string corresponding to the binary input and the number of alignment bits
fn to_binary(_data: &str) -> (&str, usize) {
    todo!()
}

/// The main function that makes the whole compression process.
///
/// ## Arguments:
/// - `data`: the data we want to encode.
///
/// ## Returns:
/// a pair (data compressed, tree compressed) where each is a pair
/// (string, align) with align the number of bits to reach a multiple of 8
pub fn main(data: &str) -> ((&str, usize), (&str, usize)) {
    let tree: Tree = build_huffman_tree(build_frequency_list(data));
    (
        to_binary(encode_data(tree.clone(), data)),
        to_binary(encode_tree(tree)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASE_TEXT: &str = "bbaabtttaabtctce";

    fn generate_base_tree() -> Vec<Node<char>> {
        vec![
            Node::new(1, 'e'),
            Node::new(2, 'c'),
            Node::new(4, 'a'),
            Node::new(4, 'b'),
            Node::new(5, 't'),
        ]
    }

    #[test]
    fn test_build_frequency_list() {
        assert_eq!(build_frequency_list(BASE_TEXT), generate_base_tree());
    }
}
