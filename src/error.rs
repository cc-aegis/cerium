use crate::lexer::error::SyntaxError;

#[derive(Debug)]
pub enum CompilerError {
    SyntaxError(SyntaxError),
}