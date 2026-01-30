use crate::error::CompilerError;

#[derive(Debug)]
pub struct UnexpectedCharacter {
    pub expected: char,
    pub actual: char,
    pub idx: usize,
}

impl CompilerError for UnexpectedCharacter {
    fn format(&self, code: &str) -> String {
        todo!()
    }
}