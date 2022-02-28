use std::{fs, process};

use clap::StructOpt;

mod cli;
mod lexer;
mod parser;

fn main() {
    let opts = cli::Opts::parse();

    // read the entire file into memory, which isn't great for large programs
    // but whatever
    let _tokens = match fs::read(&opts.path) {
        Err(e) => {
            eprintln!("Error opening {:?}: {e}", opts.path);
            process::exit(1)
        }
        Ok(code) => lexer::lex(code),
    };
}
