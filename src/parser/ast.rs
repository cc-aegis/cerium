use std::ops::Range;
use crate::parser::cerium_type::CeriumType;

pub struct Program {
    pub(crate) definitions: Vec<Definition>
}

pub enum Definition {
    Function(Function),
}

pub struct Function {
    pub name: Qualifier,
    pub parameters: Vec<(String, CeriumType)>,
    pub return_type: Option<CeriumType>,
    pub body: Box<Expression>,
}

pub struct Qualifier {
    pub names: Vec<String>,
}




pub enum Expression {
    Scope(Range<usize>, Scope),
    Integer(Range<usize>, String),
    Float(Range<usize>, String),
    Boolean(Range<usize>, bool),
    Nullptr(Range<usize>),
    Variable(Range<usize>, String),
    FieldAccess(Range<usize>, FieldAccess),
    ArrayAccess(Range<usize>, ArrayAccess),
}

impl Expression {
    pub fn range(&self) -> Range<usize> {
        match self {
            Expression::Scope(range, _) => range,
            Expression::Integer(range, _) => range,
            Expression::Float(range, _) => range,
            Expression::Boolean(range, _) => range,
            Expression::Nullptr(range) => range,
            Expression::Variable(range, _) => range,
            Expression::FieldAccess(range, _) => range,
            Expression::ArrayAccess(range, _) => range,
        }.clone()
    }
}

pub struct FieldAccess {
    pub base: Box<Expression>,
    pub field: String,
}

pub struct ArrayAccess {
    pub base: Box<Expression>,
    pub index: Box<Expression>,
}

pub struct Scope {
    pub instructions: Vec<Expression>,
    pub value: Option<Box<Expression>>,
}

struct TypeCast {
    pub value: Box<Expression>,
    pub target_type: Box<CeriumType>,
}