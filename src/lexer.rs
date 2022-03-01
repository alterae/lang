//! Module lexer contains code for turning a string of source code into a stream
//! of tokens.

/// Token is an atomic unit of source code.
#[derive(logos::Logos, Copy, Clone, Debug, PartialEq)]
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

// TokenStream holds state for the token stream.
pub struct TokenStream {
    // The stream of tokens to parse.
    _stream: Vec<Token>,
    // The index of the token currently being parsed.
    _current: usize,
}

impl TokenStream {
    // New constructs a new token stream.
    pub fn new(stream: Vec<Token>) -> Self {
        Self {
            _stream: stream,
            _current: 0,
        }
    }

    // Current gets the current token in the stream.
    pub fn _current(&self) -> Token {
        self._stream[self._current]
    }

    // Next gets the next token in the stream.
    pub fn _next(&self) -> Token {
        self._stream[self._current + 1]
    }

    // Expect checks if the next token is equal to t. If it is, the token is consumed and returned.
    pub fn _expect(&mut self, t: Token) -> Option<Token> {
        if self._current() == t {
            Some(self._advance())
        } else {
            None
        }
    }

    // Advances returns the current token, and advances the current index.
    pub fn _advance(&mut self) -> Token {
        let result = self._current();
        self._current += 1;
        result
    }
}
