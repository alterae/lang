use clap::Parser;

use crate::lexer::Lex;

mod cli;
mod lexer;

fn main() {
    let opts = cli::Opts::from_args();

    opts.file().lex();

    todo!("parse, and run input");
}
