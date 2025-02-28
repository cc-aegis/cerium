use std::ops::Range;
use crate::parser::ast::Qualifier;
use crate::parser::cerium_type::CeriumType;

#[derive(Clone, Debug)]
pub struct MismatchedReturnTypeError {
    pub function_name: Qualifier,
    pub expected: Option<CeriumType>,
    pub actual: Option<CeriumType>,
    pub range: Range<usize>
}