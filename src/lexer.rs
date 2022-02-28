//! Module lexer contains code for turning a string of source code into a stream
//! of tokens.

/// Token is an atomic unit of source code.
pub enum Token {
    Eof,
}

/// Lex converts a string of source code into a stream of tokens.
pub fn lex(source: &str) -> Vec<Token> {
    vec![]
}
