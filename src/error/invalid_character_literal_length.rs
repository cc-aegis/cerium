use std::ops::Range;
use crate::error::CompilerError;

#[derive(Debug)]
pub struct InvalidCharacterLiteralLength {
    pub indices: Range<usize>,
    pub literal: String,
}

impl CompilerError for InvalidCharacterLiteralLength {
    fn format(&self, code: &str) -> String {
        todo!()
    }
}