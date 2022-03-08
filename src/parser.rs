//! Module parser contains code to turn flat token streams into abstract
//! syntax trees.
use crate::lexer;

/// Parse parses the token stream from the lexer.
pub fn parse(mut lexer: lexer::Lexer) -> Program {
    let mut program = Vec::new();

    while lexer.peek().is_some() {
        program.push(Declaration::new(&mut lexer));
    }

    program
}

/// The root node of the AST.
pub type Program = Vec<Declaration>;

/// AST node representing a declaration.
#[derive(Debug)]
pub enum Declaration {
    /// A `use` declaration.
    Use(Path),
    /// A function declaration.
    /// TODO: return types
    Fn { params: Params, block: Block },
}

impl Declaration {
    /// Constructs a new `Declaration`.
    fn new(lexer: &mut lexer::Lexer) -> Self {
        use lexer::Token::*;
        // this unwrap is okay because we already checked `is_some()` in the
        // calling while loop
        match lexer.next().unwrap() {
            Use => Self::Use(Path::new(lexer)),
            Fn => Self::Fn {
                params: Params::new(lexer),
                block: Block::new(lexer),
            },
            Type => todo!(),
            t => {
                // TODO: something other than panic
                panic!("unexpected token {t:?}. expected `use`, `fn`, or `type`")
            }
        }
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

/// A list of function parameters.
#[derive(Debug)]
pub struct Params(Vec<Param>);

impl Params {
    fn new(lexer: &mut lexer::Lexer) -> Self {
        todo!()
    }
}

/// A function parameter.
#[derive(Debug)]
pub struct Param {
    /// The name of the function parameter.
    pub name: String,
    /// The type of the function parameter. Called `datatype` because `type` is
    /// a reserved word in Rust.
    pub datatype: String,
}

impl Param {
    fn new(lexer: &mut lexer::Lexer) -> Self {
        todo!()
    }
}

/// AST node representing a sequence of expressions.
#[derive(Debug)]
pub struct Block(Vec<Expr>);

impl Block {
    fn new(lexer: &mut lexer::Lexer) -> Self {
        todo!()
    }
}

/// AST node representing an expression.
#[derive(Debug)]
pub struct Expr;

impl Expr {
    fn new(lexer: &mut lexer::Lexer) -> Self {
        todo!()
    }
}
