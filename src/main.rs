//! A simple *Huffman Compression Algorithm* implementation.
//!
//! ## Usage
//! ...
//!
//! ## Dependencies
//!
//! + [`test-case`](https://crates.io/crates/test-case)
//! + [`clap`](https://crates.io/crates/clap)
//!
//! ## About
//!
//! + Written by [Xavier2p](https://github.com/Xavier2p).
//! + License: MIT
//! + Repository: [xavier2p/huffman](https://github.com/Xavier2p/huffman)
mod compress;
mod decompress;
mod heap;
mod node;
mod parsing;
mod tree;

use clap::Parser;

/// Launches the program, parses arguements and play!
fn main() {
    let args: parsing::Args = parsing::Args::parse();

    // let res: &str = if args.clone().get_command() == *"compress" {
    //     compress::main("test")
    // } else if args.get_command() == *"decompress" {
    //     decompress::main("test")
    // } else {
    //     "Not Implemted yet"
    // };

    // println!("{res}");
}
