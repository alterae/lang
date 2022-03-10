//! Module lexer contains code for turning a string of source code into a stream
//! of tokens.
use std::{iter, num};

use logos::Logos;

/// Token is an atomic unit of source code.
/// TODO: figure out how to get source text attached to these
#[derive(logos::Logos, Clone, Debug, PartialEq)]
#[logos(subpattern digits = r"[0-9][_0-9]*")]
pub enum Token {
    #[regex(r"[_a-zA-Z][_0-9a-zA-Z]*", string)]
    Identifier(String),

    // keywords
    #[token("fn")]
    Fn,
    #[token("type")]
    Type,
    #[token("use")]
    Use,

    // literals
    #[regex(r#""(?:[^"]|\\")*""#, string)] // scuffed quotes lol
    String(String),
    #[regex(r"(?&digits)(?:e(?&digits)|\.(?&digits)(?:e(?&digits))?)", number)]
    // todo: support for non-integers, negative numbers
    Number(f64),

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
    #[token(",")]
    Comma,
    #[token("=")]
    Equal,
    #[token("==")]
    EqualEqual,

    #[error]
    #[regex(r"\s+", logos::skip)]
    #[regex(r"//[^\n]*", logos::skip)]
    Error,
}

/// Wrapper type for the logos lexer type to simplify things.
pub type Lexer<'source> = iter::Peekable<logos::Lexer<'source, Token>>;

/// New constructs and returns a new Lexer for the given source.
pub fn new(source: &str) -> Lexer {
    Token::lexer(source).peekable()
}

fn string(lex: &mut logos::Lexer<Token>) -> String {
    lex.slice().into()
}

fn number(lex: &mut logos::Lexer<Token>) -> Result<f64, num::ParseFloatError> {
    lex.slice().parse()
}
