//! Module parser contains code to turn flat token streams into abstract
//! syntax trees.
use crate::lexer;

/// Parse parses the token stream from the lexer.
pub fn parse(mut lexer: lexer::Lexer) -> Program {
    let mut program = Vec::new();

    while let Some(t) = lexer.next() {
        use lexer::Token::*;
        match t {
            Use => program.push(Declaration::new_use(&mut lexer)),
            _ => {
                // TODO: something other than panic
                panic!("unexpected token {t:?}. expected `use`, `fn`, or `type`")
            }
        }
    }

    program
}

/// The root node of the AST.
pub type Program = Vec<Declaration>;

/// AST node representing a `use` declaration.
pub enum Declaration {
    Use(Path),
}

impl Declaration {
    /// Constructs a new `Declaration::Use`, getting a path from the lexer.
    /// FIXME: bad because it consumes things we can then not un-consume
    fn new_use(lexer: &mut lexer::Lexer) -> Self {
        let path = Vec::new();
        while let (Some(t1), Some(t2)) = (lexer.next(), lexer.next()) {
            use lexer::Token::*;
            match (t1, t2) {
                (Identifier, ColonColon) => {
                    // TODO: add the identifier to the path
                }
                (Identifier, _) => {
                    // TODO: add the identifier to the path and somehow deal
                    // with the second item
                    break;
                }
                _ => {
                    // TODO: somehow deal with this
                    break;
                }
            }
        }
        Self::Use(path)
    }
}

/// A path to an item in a module.
pub type Path = Vec<String>;
