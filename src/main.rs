mod compress;
mod decompress;
mod heap;
mod parsing;

use clap::Parser;

fn main() {
    let args: parsing::Args = parsing::Args::parse();

    let res: &str = if args.get_command() == *"compress" {
        compress::main("test")
    } else {
        "Not Implemted yet"
    };

    println!("{res}");
}
