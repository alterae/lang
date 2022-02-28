//! Module parser contains code to turn flat token streams into abstract
//! syntax trees.

use crate::lexer;

/// Ast is an abstract syntax tree.
pub struct Ast {}

/// Parse converts a stream of tokens into an abstract syntax tree.
pub fn parse(stream: Vec<lexer::Token>) -> Ast {
    println!("{stream:?}");
    // todo
    Ast {}
}
