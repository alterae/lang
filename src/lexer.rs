//! Module lexer contains code for turning a string of source code into a stream
//! of tokens.

/// Token is an atomic unit of source code.
#[derive(logos::Logos, Debug, PartialEq)]
pub enum Token {
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}
