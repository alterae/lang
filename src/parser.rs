use crate::lexer;

pub trait Parse {
    fn parse(self) -> Ast;
}

impl Parse for lexer::TokenStream {
    fn parse(self) -> Ast {
        println!("todo: parser");

        Ast // TODO
    }
}

pub struct Ast;
