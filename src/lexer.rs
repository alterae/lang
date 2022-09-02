use std::io;

use crate::cli;

pub trait Lex {
    fn lex(self) -> TokenStream;
}

impl Lex for cli::File {
    fn lex(self) -> TokenStream {
        let mut stream = Vec::new();

        use std::io::BufRead;
        // TODO: actually lex the input and return a token stream
        for line in self.lines() {
            stream.append(&mut line.lex());
        }

        stream
    }
}

impl Lex for Result<String, io::Error> {
    fn lex(self) -> TokenStream {
        let mut stream = Vec::new();

        for token in self.unwrap().split_ascii_whitespace() {
            println!("{token}");
            stream.push(Token::Ident(token.to_owned()));
        }

        stream
    }
}

pub enum Token {
    Ident(String),
}

pub type TokenStream = Vec<Token>;
