use crate::cli;

pub trait Lex {
    fn lex(self);
}

impl Lex for cli::File {
    fn lex(self) {
        use std::io::BufRead;
        // TODO: actually lex the input and return a token stream
        for line in self.lines() {
            println!("{line:?}");
        }
    }
}
