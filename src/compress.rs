//! This module contains all the function to compress a text using Huffman.

use crate::heap::{Heap, Node};

fn build_frequency_list(data: &str) -> Vec<Node> {
    let mut result: Vec<Node> = Vec::new();

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

fn heap_sort(list: Vec<Node>) -> Vec<Node> {
    let mut tmp: Heap = Heap::new();
    let mut result: Vec<Node> = Vec::new();

    for value in list {
        tmp.push(value);
    }

    while !tmp.is_empty() {
        result.push(tmp.pop().unwrap());
    }

    result
}

pub fn main(data: &str) -> &str {
    //((&str, usize), (&str, usize)) {
    let _list_freq = build_frequency_list(data);

    // (("", 1), ("", 2))
    "compression in progress..."
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const BASE_TEXT: &str = "bbaabtttaabtctce";

    fn generate_base_tree() -> Vec<Node> {
        vec![
            Node::new(1, 'e'),
            Node::new(2, 'c'),
            Node::new(4, 'a'),
            Node::new(4, 'b'),
            Node::new(5, 't'),
        ]
    }

    #[test_case(BASE_TEXT, generate_base_tree(); "base")]
    fn test_build_frequency_list(input: &str, result: Vec<Node>) {
        assert_eq!(build_frequency_list(input), result);
    }
}
