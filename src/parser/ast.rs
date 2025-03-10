use std::fmt::{Debug, Display};
use std::ops::Range;
use crate::parser::cerium_type::CeriumType;

#[derive(Debug)]
pub struct Program {
    pub(crate) definitions: Vec<Definition>
}

#[derive(Debug)]
pub enum Definition {
    Function(Function),
    Const(Const),
    Struct(Struct),
}

#[derive(Debug)]
pub struct Function {
    pub name: Qualifier,
    pub parameters: Vec<(Qualifier, CeriumType)>,
    pub return_type: Option<CeriumType>,
    pub body: Box<Expression>,
}

#[derive(Debug)]
pub struct Const {
    pub name: Qualifier,
    pub const_type: CeriumType,
    pub value: Expression,
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub name: Qualifier,
    pub attributes: Vec<(String, CeriumType)>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Qualifier {
    pub names: Vec<String>,
}

impl Display for Qualifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.names.iter();
        if let Some(first) = iter.next() {
            f.write_str(first)?;
        }
        for name in iter {
            write!(f, "::{name}")?;
        }
        Ok(())
    }
}




#[derive(Debug)]
pub enum Expression {
    Scope(Range<usize>, Scope),
    TypeCast(Range<usize>, TypeCast),
    TypeAlias(Range<usize>, TypeAlias),
    Integer(Range<usize>, String),
    Float(Range<usize>, String),
    Boolean(Range<usize>, bool),
    Nullptr(Range<usize>),
    Variable(Range<usize>, Qualifier),
    FieldAccess(Range<usize>, FieldAccess),
    ArrayAccess(Range<usize>, ArrayAccess),
    FunctionCall(Range<usize>, FunctionCall),
    Assignment(Range<usize>, Assignment),
    LogicalAnd(Range<usize>, LogicalAnd),
    LogicalOr(Range<usize>, LogicalOr),
    LessThan(Range<usize>, LessThan),
    LessThanEquals(Range<usize>, LessThanEquals),
    GreaterThan(Range<usize>, GreaterThan),
    GreaterThanEquals(Range<usize>, GreaterThanEquals),
    Equals(Range<usize>, Equals),
    NotEquals(Range<usize>, NotEquals),
    BitwiseOr(Range<usize>, BitwiseOr),
    BitwiseXor(Range<usize>, BitwiseXor),
    BitwiseAnd(Range<usize>, BitwiseAnd),
    LeftShift(Range<usize>, LeftShift),
    RightShift(Range<usize>, RightShift),
    Addition(Range<usize>, Addition),
    Subtraction(Range<usize>, Subtraction),
    Multiplication(Range<usize>, Multiplication),
    Division(Range<usize>, Division),
    Borrow(Range<usize>, Borrow),
    Negation(Range<usize>, Negation),
    Deref(Range<usize>, Deref),
    Iter(Range<usize>, Iter),
    Inversion(Range<usize>, Inversion),
    Let(Range<usize>, Let),
    If(Range<usize>, If),
    For(Range<usize>, For),
    While(Range<usize>, While),
    Loop(Range<usize>, Loop),
}

impl Expression {
    pub fn range(&self) -> Range<usize> {
        match self {
            Expression::Scope(range, _) => range,
            Expression::TypeCast(range, _) => range,
            Expression::TypeAlias(range, _) => range,
            Expression::Integer(range, _) => range,
            Expression::Float(range, _) => range,
            Expression::Boolean(range, _) => range,
            Expression::Nullptr(range) => range,
            Expression::Variable(range, _) => range,
            Expression::FieldAccess(range, _) => range,
            Expression::ArrayAccess(range, _) => range,
            Expression::FunctionCall(range, _) => range,
            Expression::Assignment(range, _) => range,
            Expression::LogicalAnd(range, _) => range,
            Expression::LogicalOr(range, _) => range,
            Expression::LessThan(range, _) => range,
            Expression::LessThanEquals(range, _) => range,
            Expression::GreaterThan(range, _) => range,
            Expression::GreaterThanEquals(range, _) => range,
            Expression::Equals(range, _) => range,
            Expression::NotEquals(range, _) => range,
            Expression::BitwiseOr(range, _) => range,
            Expression::BitwiseXor(range, _) => range,
            Expression::BitwiseAnd(range, _) => range,
            Expression::LeftShift(range, _) => range,
            Expression::RightShift(range, _) => range,
            Expression::Addition(range, _) => range,
            Expression::Subtraction(range, _) => range,
            Expression::Multiplication(range, _) => range,
            Expression::Division(range, _) => range,
            Expression::Borrow(range, _) => range,
            Expression::Negation(range, _) => range,
            Expression::Deref(range, _) => range,
            Expression::Iter(range, _) => range,
            Expression::Inversion(range, _) => range,
            Expression::Let(range, _) => range,
            Expression::If(range, _) => range,
            Expression::For(range, _) => range,
            Expression::While(range, _) => range,
            Expression::Loop(range, _) => range,
        }.clone()
    }
}

#[derive(Debug)]
pub struct FieldAccess {
    pub base: Box<Expression>,
    pub field: String,
}

#[derive(Debug)]
pub struct ArrayAccess {
    pub base: Box<Expression>,
    pub index: Box<Expression>,
}

#[derive(Debug)]
pub struct FunctionCall {
    pub func: Box<Expression>,
    pub params: Vec<Expression>,
}

#[derive(Debug)]
pub struct Scope {
    pub instructions: Vec<Expression>,
    pub value: Option<Box<Expression>>,
}

#[derive(Debug)]
pub struct TypeCast {
    pub value: Box<Expression>,
    pub target_type: Box<CeriumType>,
}

#[derive(Debug)]
pub struct TypeAlias {
    pub value: Box<Expression>,
    pub target_type: Box<CeriumType>,
}

#[derive(Debug)]
pub struct Assignment {
    pub target: Box<Expression>,
    pub value: Box<Expression>,
}

#[derive(Debug)]
pub struct LogicalAnd {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

#[derive(Debug)]
pub struct LogicalOr {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

#[derive(Debug)]
pub struct LessThan {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

#[derive(Debug)]
pub struct LessThanEquals {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

#[derive(Debug)]
pub struct GreaterThan {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

#[derive(Debug)]
pub struct GreaterThanEquals {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

#[derive(Debug)]
pub struct Equals {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

#[derive(Debug)]
pub struct NotEquals {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

#[derive(Debug)]
pub struct BitwiseOr {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

#[derive(Debug)]
pub struct BitwiseXor {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

#[derive(Debug)]
pub struct BitwiseAnd {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

#[derive(Debug)]
pub struct LeftShift {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

#[derive(Debug)]
pub struct RightShift {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

#[derive(Debug)]
pub struct Addition {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

#[derive(Debug)]
pub struct Subtraction {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

#[derive(Debug)]
pub struct Multiplication {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

#[derive(Debug)]
pub struct Division {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

#[derive(Debug)]
pub struct Borrow {
    pub inner: Box<Expression>
}

#[derive(Debug)]
pub struct Negation {
    pub inner: Box<Expression>
}

#[derive(Debug)]
pub struct Deref {
    pub inner: Box<Expression>
}

#[derive(Debug)]
pub struct Iter {
    pub inner: Box<Expression>
}

#[derive(Debug)]
pub struct Inversion {
    pub inner: Box<Expression>
}

#[derive(Debug)]
pub struct Let {
    pub name: Qualifier,
    pub value: Box<Expression>
}

#[derive(Debug)]
pub struct If {
    pub condition: Box<Expression>,
    pub if_branch: Box<Expression>,
    pub else_branch: Option<Box<Expression>>,
}

#[derive(Debug)]
pub struct For {
    pub counter: Qualifier,
    pub initialization: Option<Box<Expression>>,
    pub limit: Option<Box<Expression>>,
    pub step: Option<Box<Expression>>,
    pub body: Box<Expression>,
}

#[derive(Debug)]
pub struct While {
    pub condition: Box<Expression>,
    pub body: Box<Expression>,
}

#[derive(Debug)]
pub struct Loop {
    pub body: Box<Expression>,
}