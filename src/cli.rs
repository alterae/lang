use std::{fs, io};

pub type File = Box<dyn io::BufRead>;

/// A simple programming language.
#[derive(clap::Parser)]
pub(crate) struct Opts {
    /// The file to run. Defaults to standard input.
    pub file: Option<std::path::PathBuf>,
}

impl Opts {
    pub fn file(&self) -> File {
        match &self.file {
            Some(path) => Box::new(io::BufReader::new(fs::File::open(path).unwrap())),
            None => Box::new(io::BufReader::new(io::stdin())),
        }
    }
}
