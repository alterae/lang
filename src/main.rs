use clap::Parser;

mod cli;

fn main() {
    let _opts = cli::Opts::from_args();
    todo!("read, lex, parse, and run input");
}
