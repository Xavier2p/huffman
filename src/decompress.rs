//! This module contains all the things to decompress a text using Huffman.

use crate::tree::Tree;

/// Decodes a string using the corresponding huffman tree into something more readable.
///
/// ## Arguements:
/// - `tree`: the huffman tree for decoding.
/// - `data`: the input binary string we want to decode.
///
/// ## Returns:
/// the decoded text.
fn decode_data(_tree: Tree, _data: &str) -> &str {
    todo!()
}

/// Decodes a huffman tree from its binary representation:
/// - a '0' means we add a new internal node and go to its left node
/// - a '1' means the next 8 values are the encoded character of the current leaf
///
/// ## Arguements:
/// - `data`: the binary string containing the encoded huffman tree.
///
/// ## Returns:
/// decoded huffman tree
fn decode_tree(_data: &str) -> Tree {
    todo!()
}

/// Retrieve a string containing binary code from its real binary value
/// (inverse of `compress::to_binary()`).
///
/// ## Arguements:
/// - `data`: the string to convert in binary code.
/// - `align`: the number of bits to ignore at the end of the input (alignment bits).
///
/// ## Returns:
/// a string containing the binary code.
fn from_binary(_data: &str, _align: usize) -> &str {
    todo!()
}

/// The whole decompression process.
///
/// ## Arguments:
/// - `data`: the compressed data.
/// - `aling_data`: the number of bits to ignore at the end of data.
/// - `tree`: the compressed tree.
/// - `align_tree`: the number of bits to ignore at the end of tree.
///
/// ## Returns:
/// the decompressed string.
pub fn main<'a>(_data: &'a str, _align_data: usize, _tree: &'a str, _align_tree: usize) -> &'a str {
    let binary_tree: &str = from_binary(_tree, _align_tree);
    let binary_data: &str = from_binary(_data, _align_data);
    let tree_decoded: Tree = decode_tree(binary_tree);
    decode_data(tree_decoded, binary_data)
}
