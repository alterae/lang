use std::{fs, process};

use clap::StructOpt;
use logos::Logos;

mod cli;
mod lexer;
mod parser;

fn main() {
    let opts = cli::Opts::parse();

    // read the entire file into memory, which isn't great for large programs
    // but whatever
    let stream = match fs::read_to_string(&opts.path) {
        Err(e) => {
            eprintln!("Error opening {:?}: {e}", opts.path);
            process::exit(1)
        }
        Ok(code) => lexer::TokenStream::new(lexer::Token::lexer(&code).collect()),
    };

    let _ = parser::parse(stream);
}
