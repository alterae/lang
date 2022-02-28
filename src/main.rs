use clap::StructOpt;

mod cli;
mod parser;

fn main() {
    let opts = cli::Opts::parse();
    let _path = match opts.cmd {
        cli::SubCommand::Build { path } => path,
        cli::SubCommand::Run { path, bytecode: _ } => path,
    };
}
