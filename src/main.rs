use std::fs;

use clap::StructOpt;

mod cli;
mod lexer;
mod parser;

fn main() {
    let opts = cli::Opts::parse();

    // read the entire file into memory, which isn't great for large programs
    // but whatever
    let source = &fs::read_to_string(&opts.path).expect("error reading file");

    // construct a lexer for the source code
    let lexer = lexer::new(source);

    // pass the parser the lexer so it can just use it and make things easier for me
    let _program = parser::parse(lexer);
}
