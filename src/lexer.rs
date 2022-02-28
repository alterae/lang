//! Module lexer contains code for turning a string of source code into a stream
//! of tokens.

/// Token is an atomic unit of source code.
#[derive(logos::Logos, Debug, PartialEq)]
pub enum Token {
    #[regex(r"[_a-zA-Z][_0-9a-zA-Z]*")]
    Identifier,

    // keywords
    #[regex(r"use")]
    Use,
    #[regex(r"fn")]
    Fn,

    // literals
    #[regex(r#""(?:[^"]|\\")*""#)] // scuffed quotes lol
    String,
    #[regex(r"[1-9][_1-9]*")] // todo: support for non-integers
    Number,

    // parenthesis, curly braces
    #[regex(r"\(")]
    ParenLeft,
    #[regex(r"\)")]
    ParenRight,
    #[regex(r"\{")]
    BraceLeft,
    #[regex(r"\}")]
    BraceRight,

    // other punctuation
    #[regex(r":")]
    Colon,
    #[regex(r"::")]
    ColonColon,
    #[regex(r"=")]
    Equal,
    #[regex(r"==")]
    EqualEqual,

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[regex(r"//[^\n]*", logos::skip)]
    Error,
}
