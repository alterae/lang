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
            Fn => todo!(),
            Type => todo!(),
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
#[derive(Debug)]
pub enum Declaration {
    Use(Path),
}

impl Declaration {
    /// Constructs a new `Declaration::Use`.
    fn new_use(lexer: &mut lexer::Lexer) -> Self {
        Self::Use(Path::new(lexer))
    }
}

/// A path to an item in a module.
/// TODO: break out parsing logic into separate function we can reuse
#[derive(Debug)]
pub struct Path(Vec<String>);

impl Path {
    /// Constructs a new `Path`, getting the tokens for the path from the lexer.
    fn new(lexer: &mut lexer::Lexer) -> Self {
        let mut path = Vec::new();
        while let (Some(t1), Some(t2)) = (lexer.next(), lexer.peek()) {
            use lexer::Token::*;
            match (t1, t2) {
                (Identifier(ident), &ColonColon) => {
                    path.push(ident);

                    // discard the ColonColon
                    lexer.next();
                }
                (Identifier(ident), _t) => {
                    path.push(ident);

                    break;
                }
                (t, _) => {
                    panic!("unexpected token {t:?}. expected identifier")
                }
            }
        }

        Self(path)
    }
}
