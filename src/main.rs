use clap::Parser;
use parser::Parse;
use runner::Run;

use crate::lexer::Lex;

mod cli;
mod lexer;
mod parser;
mod runner;

fn main() -> runner::Result {
    let opts = cli::Opts::from_args();

    opts.file().lex().parse().run()
}
