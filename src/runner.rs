use crate::parser;

pub trait Run {
    fn run(self) -> Result;
}

impl Run for parser::Ast {
    fn run(self) -> Result {
        println!("Hello, world!");

        Ok(())
    }
}

pub type Result = std::result::Result<(), ()>;
