use crate::lexer::error::SyntaxError;
use crate::parser::UnexpectedTokenError;

#[derive(Clone, Debug)]
pub enum CompilerError {
    SyntaxError(SyntaxError),
    UnexpectedTokenError(UnexpectedTokenError),
    MissingTokenError,
}