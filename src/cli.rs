/// A simple programming language.
#[derive(clap::Parser)]
pub(crate) struct Opts {
    /// The file to run. Defaults to standard input.
    pub file: Option<std::path::PathBuf>,
}
