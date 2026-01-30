use crate::error::CompilerError;

#[derive(Debug)]
pub struct InvalidCharacter {
    pub found: char,
    pub idx: usize,
}

impl CompilerError for InvalidCharacter {
    fn format(&self, code: &str) -> String {
        todo!()
    }
}