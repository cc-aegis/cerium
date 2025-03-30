use std::fmt::{Debug, Display};
use std::ops::Range;
use std::rc::Rc;
use crate::parser::cerium_type::CeriumType;

#[derive(Debug, Clone)]
pub struct RangeAnnotation<Inner> {
    pub range: Range<usize>,
    pub inner: Inner,
}

impl<Inner> RangeAnnotation<Inner> {
    pub fn new(range: Range<usize>, inner: Inner) -> Self {
        RangeAnnotation { range, inner }
    }
}

impl <Inner> std::ops::Deref for RangeAnnotation<Inner> {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

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
    pub name: RangeAnnotation<Qualifier>,
    pub parameters: Vec<(RangeAnnotation<Qualifier>, RangeAnnotation<CeriumType>)>,
    pub return_type: Option<RangeAnnotation<CeriumType>>,
    pub body: RangeAnnotation<Expression>,
}

#[derive(Debug)]
pub struct Const {
    pub name: RangeAnnotation<Qualifier>,
    pub const_type: RangeAnnotation<CeriumType>,
    pub value: RangeAnnotation<Expression>,
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub name: RangeAnnotation<Qualifier>,
    pub attributes: Vec<(RangeAnnotation<Qualifier>, RangeAnnotation<CeriumType>)>,
}

#[derive(Debug, Clone)]
pub struct Qualifier {
    pub names: Rc<[Box<str>]>,
}

impl Qualifier {
    pub fn from(names: Vec<String>) -> Self {
        Qualifier {
            names: names
                .into_iter()
                .map(String::into_boxed_str)
                .collect(),
        }
    }

    pub fn from_str(name: impl Into<Box<str>>) -> Self {
        Qualifier {
            names: Rc::new([name.into()]),
        }
    }
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
    Scope(Box<Scope>),
    TypeCast(Box<TypeCast>),
    TypeAlias(Box<TypeAlias>),
    UnsignedInteger(u16),
    SignedInteger(i16),
    Float(f32),
    Boolean(bool),
    Nullptr,
    Variable(Qualifier),
    FieldAccess(Box<FieldAccess>),
    ArrayAccess(Box<ArrayAccess>),
    FunctionCall(Box<FunctionCall>),
    Assignment(Box<Assignment>),
    LogicalAnd(Box<LogicalAnd>),
    LogicalOr(Box<LogicalOr>),
    LessThan(Box<LessThan>),
    LessThanEquals(Box<LessThanEquals>),
    GreaterThan(Box<GreaterThan>),
    GreaterThanEquals(Box<GreaterThanEquals>),
    Equals(Box<Equals>),
    NotEquals(Box<NotEquals>),
    BitwiseOr(Box<BitwiseOr>),
    BitwiseXor(Box<BitwiseXor>),
    BitwiseAnd(Box<BitwiseAnd>),
    LeftShift(Box<LeftShift>),
    RightShift(Box<RightShift>),
    Addition(Box<Addition>),
    Subtraction(Box<Subtraction>),
    Multiplication(Box<Multiplication>),
    Division(Box<Division>),
    Borrow(Box<Borrow>),
    Negation(Box<Negation>),
    Deref(Box<Deref>),
    Iter(Box<Iter>),
    Inversion(Box<Inversion>),
    Let(Box<Let>),
    LetIn(Box<LetIn>),
    If(Box<If>),
    ForTo(Box<ForTo>),
    ForDownTo(Box<ForDownTo>),
    ForIn(Box<ForIn>),
    While(Box<While>),
    Loop(Box<Loop>),
}

#[derive(Debug)]
pub struct FieldAccess {
    pub base: RangeAnnotation<Expression>,
    pub field: RangeAnnotation<Qualifier>,
}

#[derive(Debug)]
pub struct ArrayAccess {
    pub base: RangeAnnotation<Expression>,
    pub index: RangeAnnotation<Expression>,
}

#[derive(Debug)]
pub struct FunctionCall {
    pub func: RangeAnnotation<Expression>,
    pub params: Vec<RangeAnnotation<Expression>>,
}

#[derive(Debug)]
pub struct Scope {
    pub instructions: Vec<RangeAnnotation<Expression>>,
    pub value: Option<RangeAnnotation<Expression>>,
}

#[derive(Debug)]
pub struct TypeCast {
    pub value: RangeAnnotation<Expression>,
    pub target_type: RangeAnnotation<CeriumType>,
}

#[derive(Debug)]
pub struct TypeAlias {
    pub value: RangeAnnotation<Expression>,
    pub target_type: RangeAnnotation<CeriumType>,
}

#[derive(Debug)]
pub struct Assignment {
    pub target: RangeAnnotation<Expression>,
    pub value: RangeAnnotation<Expression>,
}

#[derive(Debug)]
pub struct LogicalAnd {
    pub lhs: RangeAnnotation<Expression>,
    pub rhs: RangeAnnotation<Expression>,
}

#[derive(Debug)]
pub struct LogicalOr {
    pub lhs: RangeAnnotation<Expression>,
    pub rhs: RangeAnnotation<Expression>,
}

#[derive(Debug)]
pub struct LessThan {
    pub lhs: RangeAnnotation<Expression>,
    pub rhs: RangeAnnotation<Expression>,
}

#[derive(Debug)]
pub struct LessThanEquals {
    pub lhs: RangeAnnotation<Expression>,
    pub rhs: RangeAnnotation<Expression>,
}

#[derive(Debug)]
pub struct GreaterThan {
    pub lhs: RangeAnnotation<Expression>,
    pub rhs: RangeAnnotation<Expression>,
}

#[derive(Debug)]
pub struct GreaterThanEquals {
    pub lhs: RangeAnnotation<Expression>,
    pub rhs: RangeAnnotation<Expression>,
}

#[derive(Debug)]
pub struct Equals {
    pub lhs: RangeAnnotation<Expression>,
    pub rhs: RangeAnnotation<Expression>,
}

#[derive(Debug)]
pub struct NotEquals {
    pub lhs: RangeAnnotation<Expression>,
    pub rhs: RangeAnnotation<Expression>,
}

#[derive(Debug)]
pub struct BitwiseOr {
    pub lhs: RangeAnnotation<Expression>,
    pub rhs: RangeAnnotation<Expression>,
}

#[derive(Debug)]
pub struct BitwiseXor {
    pub lhs: RangeAnnotation<Expression>,
    pub rhs: RangeAnnotation<Expression>,
}

#[derive(Debug)]
pub struct BitwiseAnd {
    pub lhs: RangeAnnotation<Expression>,
    pub rhs: RangeAnnotation<Expression>,
}

#[derive(Debug)]
pub struct LeftShift {
    pub lhs: RangeAnnotation<Expression>,
    pub rhs: RangeAnnotation<Expression>,
}

#[derive(Debug)]
pub struct RightShift {
    pub lhs: RangeAnnotation<Expression>,
    pub rhs: RangeAnnotation<Expression>,
}

#[derive(Debug)]
pub struct Addition {
    pub lhs: RangeAnnotation<Expression>,
    pub rhs: RangeAnnotation<Expression>,
}

#[derive(Debug)]
pub struct Subtraction {
    pub lhs: RangeAnnotation<Expression>,
    pub rhs: RangeAnnotation<Expression>,
}

#[derive(Debug)]
pub struct Multiplication {
    pub lhs: RangeAnnotation<Expression>,
    pub rhs: RangeAnnotation<Expression>,
}

#[derive(Debug)]
pub struct Division {
    pub lhs: RangeAnnotation<Expression>,
    pub rhs: RangeAnnotation<Expression>,
}

#[derive(Debug)]
pub struct Borrow {
    pub inner: RangeAnnotation<Expression>,
}

#[derive(Debug)]
pub struct Negation {
    pub inner: RangeAnnotation<Expression>,
}

#[derive(Debug)]
pub struct Deref {
    pub inner: RangeAnnotation<Expression>,
}

#[derive(Debug)]
pub struct Iter {
    pub inner: RangeAnnotation<Expression>,
}

#[derive(Debug)]
pub struct Inversion {
    pub inner: RangeAnnotation<Expression>,
}

#[derive(Debug)]
pub struct Let {
    pub name: RangeAnnotation<Qualifier>,
    pub value: RangeAnnotation<Expression>,
}

#[derive(Debug)]
pub struct LetIn {
    pub name: RangeAnnotation<Qualifier>,
    pub value: RangeAnnotation<Expression>,
    pub body: RangeAnnotation<Expression>,
}

#[derive(Debug)]
pub struct If {
    pub condition: RangeAnnotation<Expression>,
    pub if_branch: RangeAnnotation<Expression>,
    pub else_branch: Option<RangeAnnotation<Expression>>,
}

#[derive(Debug)]
pub struct ForTo {
    pub var: RangeAnnotation<Qualifier>,
    pub limit: RangeAnnotation<Expression>,
    pub body: RangeAnnotation<Expression>,
}

#[derive(Debug)]
pub struct ForDownTo {
    pub var: RangeAnnotation<Qualifier>,
    pub limit: RangeAnnotation<Expression>,
    pub body: RangeAnnotation<Expression>,
}

#[derive(Debug)]
pub struct ForIn {
    pub var: RangeAnnotation<Qualifier>,
    pub iterator: RangeAnnotation<Expression>,
    pub body: RangeAnnotation<Expression>,
}

#[derive(Debug)]
pub struct While {
    pub condition: RangeAnnotation<Expression>,
    pub body: RangeAnnotation<Expression>,
}

#[derive(Debug)]
pub struct Loop {
    pub body: RangeAnnotation<Expression>,
}