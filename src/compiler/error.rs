use std::ops::Range;
use crate::parser::cerium_type::CeriumType;

#[derive(Clone, Debug)]
pub struct MismatchedTypesError {
    pub expected: Option<CeriumType>,
    pub actual: Option<CeriumType>,
    pub range: Range<usize>
}