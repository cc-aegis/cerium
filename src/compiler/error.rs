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

#[derive(Clone, Debug)]
pub struct MismatchedAssignTypeError {
    pub dst_range: Range<usize>,
    pub dst_type: Option<CeriumType>,
    pub src_range: Range<usize>,
    pub src_type: Option<CeriumType>,
}

#[derive(Clone, Debug)]
pub struct InvalidDerefError {
    pub range: Range<usize>,
    pub found: Option<CeriumType>,
}