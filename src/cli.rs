use std::path;

/// A proof-of-concept language interpreter.
#[derive(clap::Parser, Debug)]
#[clap(version)]
pub struct Opts {
    #[clap(subcommand)]
    pub cmd: SubCommand,
}

#[derive(clap::Subcommand, Debug)]
pub enum SubCommand {
    /// Compile a program to bytecode without running it.
    Build {
        /// The path to the program to build.
        path: path::PathBuf,
    },
    /// Run a program.
    Run {
        /// The path to the program to run.
        path: path::PathBuf,
        #[clap(short, long)]
        /// Run an already compiled bytecode program.
        bytecode: bool,
    },
}
