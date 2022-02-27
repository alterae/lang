use clap::StructOpt;

mod cli;
mod parser;

fn main() {
    let opts = cli::Opts::parse();
    let path = match opts.cmd {
        cli::SubCommand::Build { path } => path,
        cli::SubCommand::Run { path, bytecode } => path,
    };

    // fixme: does not work
    let foo = parser::Parser::parse();
}
