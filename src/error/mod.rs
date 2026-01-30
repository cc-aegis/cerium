use std::fmt::Debug;

pub mod unexpected_character;
pub mod unexpected_eof;
pub mod invalid_character_literal_length;
pub mod invalid_character;

pub trait CompilerError: Debug {
    fn format(&self, code: &str) -> String;
}

impl <'a, E: CompilerError + 'a> From<E> for Box<dyn CompilerError + 'a> {
    fn from(value: E) -> Self {
        Box::new(value)
    }
}