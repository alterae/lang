use crate::cli;

pub trait Lex {
    fn lex(self) -> TokenStream;
}

impl Lex for cli::File {
    fn lex(self) -> TokenStream {
        let stream = Vec::new();

        use std::io::BufRead;
        // TODO: actually lex the input and return a token stream
        for line in self.lines() {
            println!("{line:?}");
        }

        stream
    }
}

pub enum Token {}

pub type TokenStream = Vec<Token>;
