use clap::StructOpt;

mod cli;
mod lexer;
mod parser;

fn main() {
    let opts = cli::Opts::parse();

    println!("running {:?}", opts.path.as_os_str());
}
