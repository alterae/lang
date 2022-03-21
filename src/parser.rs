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
    Fn {
        name: String,
        params: Params,
        return_type: Option<String>,
        block: Block,
    },
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
                name: match lexer.next() {
                    Some(Identifier(s)) => s,
                    Some(t) => panic!("unexpected token {t:?}. expected function name"),
                    None => panic!("unexpected end of file. expected function name"),
                },
                params: Params::new(lexer),
                return_type: match lexer.next() {
                    Some(Identifier(s)) => Some(s),
                    Some(BraceLeft) => None,
                    Some(t) => panic!("unexpected token {t:?}. expected return type or `{{`"),
                    None => panic!("unexpected end of file. expected return type or `{{`"),
                },
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
                (Identifier(ident), _) => {
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

    fn prepend(self, ident: String) -> Self {
        let path = vec![ident];
        path.append(&mut self.0);
        Self(path)
    }
}

/// A list of function parameters.
#[derive(Debug)]
pub struct Params(Vec<Param>);

impl Params {
    fn new(lexer: &mut lexer::Lexer) -> Self {
        match lexer.next().expect("unexpected end of file. expected `(`") {
            lexer::Token::ParenLeft => {}
            t => panic!("unexpected token {t:?}. expected `(`"),
        };

        let mut params = Vec::new();

        if let Some(lexer::Token::ParenRight) = lexer.peek() {
            lexer.next();
            return Self(params);
        }

        while let (Some(t1), Some(t2), Some(t3)) = (lexer.next(), lexer.next(), lexer.next()) {
            use lexer::Token::*;
            match (t1, t2, t3) {
                (Identifier(name), Identifier(datatype), Comma) => {
                    params.push(Param { name, datatype })
                }
                (Identifier(name), Identifier(datatype), ParenRight) => {
                    params.push(Param { name, datatype });
                    break;
                }
                ts => panic!("unexpected tokens {ts:?}. expected `name Type,` or `name Type)`"),
            }
        }

        Self(params)
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

/// AST node representing a sequence of expressions.
#[derive(Debug)]
pub struct Block(Vec<Expr>);

impl Block {
    fn new(lexer: &mut lexer::Lexer) -> Self {
        let mut block = Vec::new();

        while lexer.peek().is_some() {
            block.push(Expr::new(lexer))
        }

        Self(block)
    }
}

/// AST node representing an expression.
#[derive(Debug)]
pub enum Expr {
    /// Binary expresion (ie `l + r`).
    Binary {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
        op: Operator,
    },
    /// Unary expression (ie `-x`).
    Unary { expr: Box<Expr>, op: Operator },
    /// "Variable" binding (ie `name := expr`).
    Binding { name: String, value: Box<Expr> },
    /// Function call.
    Call { path: Path, args: Vec<Expr> },
    /// String literal.
    String(String),
    /// Numeric literal.
    Number(f64),
    /// Binding name.
    Identifier(String),
}

impl Expr {
    fn new(lexer: &mut lexer::Lexer) -> Self {
        match lexer.next() {
            Some(lexer::Token::String(s)) => Self::String(s),
            Some(lexer::Token::Number(n)) => Self::Number(n),
            // if it starts with a name it's either a reference to an existing
            // variable or a binding to a new one
            Some(lexer::Token::Identifier(s)) => match lexer.peek() {
                Some(lexer::Token::ColonEqual) => {
                    lexer.next();
                    lexer.next();

                    Self::Binding {
                        name: s,
                        value: Expr::new(lexer).into(),
                    }
                }
                Some(lexer::Token::ColonColon) => {
                    // TODO: skip the `::`, parse a path, prepend `s`
                    todo!()
                }
                Some(_) => Self::Identifier(s),
                None => panic!("unexpected end of file. expected `:=` or start of expression"),
            },
            Some(t) => panic!("unexpected token {t:?}. expected start of expression"),
            None => panic!("unexpected end of file. expected start of expression"),
        }
    }
}

#[derive(Debug)]
pub enum Operator {}
