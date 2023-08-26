//! This module contains all things related to parsing command line arguments.

use clap::Parser;

#[derive(Clone, Debug, Parser)]
#[clap(about, version, author)]
pub struct Args {
    /// Command [compress | decompress]
    #[clap(value_parser = check_command)]
    command: String,

    /// Enable verbose mode
    #[clap(short, long)]
    verbose: bool,
}

impl Args {
    pub fn get_command(self) -> String {
        self.command
    }
}

fn check_command(input: &str) -> Result<String, String> {
    match input {
        "compress" | "decompress" => Ok(input.to_string()),
        "c" => Ok("compress".to_string()),
        "d" => Ok("decompress".to_string()),
        _ => Err(format!("This command doesn't exists: {}.", input)),
    }
}
