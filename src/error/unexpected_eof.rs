use crate::error::CompilerError;

#[derive(Debug)]
pub enum Expected {
    Character(char),
    Literal,
    Number,
    Identifier,
}

#[derive(Debug)]
pub struct UnexpectedEof {
    pub expected: Expected,
}

impl CompilerError for UnexpectedEof {
    fn format(&self, code: &str) -> String {
        todo!()
    }
}