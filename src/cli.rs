use std::path;

/// A proof-of-concept language interpreter.
#[derive(clap::Parser, Debug)]
#[clap(version)]
pub struct Opts {
    /// The file to run.
    pub path: path::PathBuf,
}
