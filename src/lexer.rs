//! Module lexer contains code for turning a string of source code into a stream
//! of tokens.

/// Token is an atomic unit of source code.
#[derive(logos::Logos, Copy, Debug, PartialEq)]
pub enum Token {
    #[regex(r"[_a-zA-Z][_0-9a-zA-Z]*")]
    Identifier,

    // keywords
    #[token("use")]
    Use,
    #[token("fn")]
    Fn,

    // literals
    #[regex(r#""(?:[^"]|\\")*""#)] // scuffed quotes lol
    String,
    #[regex(r"[1-9][_1-9]*")] // todo: support for non-integers
    Number,

    // parenthesis, curly braces
    #[token("(")]
    ParenLeft,
    #[token(")")]
    ParenRight,
    #[token("{")]
    BraceLeft,
    #[token("}")]
    BraceRight,

    // other punctuation
    #[token("::")]
    ColonColon,
    #[token(":=")]
    ColonEqual,
    #[token("=")]
    Equal,
    #[token("==")]
    EqualEqual,

    #[error]
    #[regex(r"\s+", logos::skip)]
    #[regex(r"//[^\n]*", logos::skip)]
    Error,
}
