use crate::compiler::assembly::{Instruction, Operand};
use crate::compiler::vars::Vars;
use crate::error::CompilerError;
use crate::parser::ast::Expression;
use crate::parser::cerium_type::CeriumType;

impl Expression {
    pub(crate) fn compile_mut(self, vars: &mut Vars) -> Result<(Vec<Instruction>, Option<(Operand, CeriumType)>), CompilerError> {
        match self {
            Expression::Scope(_, _) => todo!(),
            Expression::TypeCast(_, _) => todo!(),
            Expression::TypeAlias(_, _) => todo!(),
            Expression::Integer(_, _) => todo!(),
            Expression::Float(_, _) => todo!(),
            Expression::Boolean(_, _) => todo!(),
            Expression::Nullptr(_) => todo!(),
            Expression::Variable(_, _) => todo!(),
            Expression::FieldAccess(_, _) => todo!(),
            Expression::ArrayAccess(_, _) => todo!(),
            Expression::FunctionCall(_, _) => todo!(),
            Expression::Assignment(_, _) => todo!(),
            Expression::LogicalAnd(_, _) => todo!(),
            Expression::LogicalOr(_, _) => todo!(),
            Expression::LessThan(_, _) => todo!(),
            Expression::LessThanEquals(_, _) => todo!(),
            Expression::GreaterThan(_, _) => todo!(),
            Expression::GreaterThanEquals(_, _) => todo!(),
            Expression::Equals(_, _) => todo!(),
            Expression::NotEquals(_, _) => todo!(),
            Expression::BitwiseOr(_, _) => todo!(),
            Expression::BitwiseXor(_, _) => todo!(),
            Expression::BitwiseAnd(_, _) => todo!(),
            Expression::LeftShift(_, _) => todo!(),
            Expression::RightShift(_, _) => todo!(),
            Expression::Addition(_, _) => todo!(),
            Expression::Subtraction(_, _) => todo!(),
            Expression::Multiplication(_, _) => todo!(),
            Expression::Division(_, _) => todo!(),
            Expression::Borrow(_, _) => todo!(),
            Expression::Negation(_, _) => todo!(),
            Expression::Deref(_, _) => todo!(),
            Expression::Iter(_, _) => todo!(),
            Expression::Inversion(_, _) => todo!(),
            Expression::Let(_, _) => todo!(),
            Expression::If(_, _) => todo!(),
            Expression::For(_, _) => todo!(),
            Expression::While(_, _) => todo!(),
            Expression::Loop(_, _) => todo!(),
        }
    }
}