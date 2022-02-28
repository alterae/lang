//! Module lexer contains code for turning a string of source code into a stream
//! of tokens.

/// Token is an atomic unit of source code.
pub enum Token {
    Eof,
}

/// Lex converts a sequence of source code bytes into a stream of tokens.
pub fn lex(source: Vec<u8>) -> Vec<Token> {
    println!("{source:?}");
    vec![Token::Eof]
}
