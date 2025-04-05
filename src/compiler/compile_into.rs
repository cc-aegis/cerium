use crate::compiler::asm::{Instruction, IntermediateOperand};
use crate::compiler::context::CompilerContext;
use crate::error::CompilerError;
use crate::parser::ast::{Expression, Scope};
use crate::parser::cerium_type::CeriumType;

impl Expression {
    pub fn compile_into(self, target: IntermediateOperand, ctx: &mut CompilerContext) -> Result<(Vec<Instruction<IntermediateOperand>>, Option<CeriumType>), CompilerError> {
        match self {
            Expression::Scope(scope) => scope.compile_into(target, ctx),
            Expression::TypeCast(_) => todo!(),
            Expression::TypeAlias(_) => todo!(),
            Expression::UnsignedInteger(_) => todo!(),
            Expression::SignedInteger(_) => todo!(),
            Expression::Float(_) => todo!(),
            Expression::Boolean(_) => todo!(),
            Expression::Nullptr => todo!(),
            Expression::Variable(_) => todo!(),
            Expression::FieldAccess(_) => todo!(),
            Expression::ArrayAccess(_) => todo!(),
            Expression::FunctionCall(_) => todo!(),
            Expression::Assignment(_) => todo!(),
            Expression::LogicalAnd(_) => todo!(),
            Expression::LogicalOr(_) => todo!(),
            Expression::LessThan(_) => todo!(),
            Expression::LessThanEquals(_) => todo!(),
            Expression::GreaterThan(_) => todo!(),
            Expression::GreaterThanEquals(_) => todo!(),
            Expression::Equals(_) => todo!(),
            Expression::NotEquals(_) => todo!(),
            Expression::BitwiseOr(_) => todo!(),
            Expression::BitwiseXor(_) => todo!(),
            Expression::BitwiseAnd(_) => todo!(),
            Expression::LeftShift(_) => todo!(),
            Expression::RightShift(_) => todo!(),
            Expression::Addition(_) => todo!(),
            Expression::Subtraction(_) => todo!(),
            Expression::Multiplication(_) => todo!(),
            Expression::Division(_) => todo!(),
            Expression::Borrow(_) => todo!(),
            Expression::Negation(_) => todo!(),
            Expression::Deref(_) => todo!(),
            Expression::Iter(_) => todo!(),
            Expression::Inversion(_) => todo!(),
            Expression::Let(_) => todo!(),
            Expression::LetIn(_) => todo!(),
            Expression::If(_) => todo!(),
            Expression::ForTo(_) => todo!(),
            Expression::ForDownTo(_) => todo!(),
            Expression::ForIn(_) => todo!(),
            Expression::While(_) => todo!(),
            Expression::Loop(_) => todo!(),
        }
    }
}

impl Scope {
    fn compile_into(self, target: IntermediateOperand, ctx: &mut CompilerContext) -> Result<(Vec<Instruction<IntermediateOperand>>, Option<CeriumType>), CompilerError> {
        todo!()
    }
}