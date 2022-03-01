//! Module parser contains code to turn flat token streams into abstract
//! syntax trees.

use crate::lexer;

/// Ast is an abstract syntax tree.
pub struct Ast {}

/// Parse converts a stream of tokens into an abstract syntax tree.
pub fn parse(stream: Vec<lexer::Token>) -> Ast {
    let stream = TokenStream::new(stream);
    Ast {}
}

// TokenStream holds state for the token stream.
struct TokenStream {
    // The stream of tokens to parse.
    stream: Vec<lexer::Token>,
    // The index of the token currently being parsed.
    current: usize,
}

impl TokenStream {
    // New constructs a new token stream with a current index of 0.
    fn new(stream: Vec<lexer::Token>) -> Self {
        Self { stream, current: 0 }
    }

    // Current gets the current token in the stream.
    fn current(&self) -> lexer::Token {
        self.stream[self.current]
    }

    // Next gets the next token in the stream.
    fn next(&self) -> lexer::Token {
        self.stream[self.current + 1]
    }

    // Expect checks if the next token is equal to t. If it is, the token is consumed and returned.
    fn expect(&mut self, t: lexer::Token) -> Option<lexer::Token> {
        if self.current() == t {
            Some(self.advance())
        } else {
            None
        }
    }

    // Advances returns the current token, and advances the current index.
    fn advance(&mut self) -> lexer::Token {
        let result = self.current();
        self.current += 1;
        result
    }
}
