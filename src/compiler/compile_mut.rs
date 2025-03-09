use crate::compiler::assembly::{Instruction, Operand};
use crate::compiler::vars::Vars;
use crate::error::CompilerError;
use crate::parser::ast::{Addition, Deref, Expression};
use crate::parser::cerium_type::CeriumType;

impl Expression {
    pub fn compile_mut(self, vars: &mut Vars) -> Result<(Vec<Instruction>, Option<(Operand, CeriumType)>), CompilerError> {
        match self {
            Expression::Scope(_, _) => todo!(),
            Expression::TypeCast(_, _) => todo!(),
            Expression::TypeAlias(_, _) => todo!(),
            Expression::Integer(_, _) => todo!(),
            Expression::Float(_, _) => todo!(),
            Expression::Boolean(_, _) => todo!(),
            Expression::Nullptr(_) => todo!(),
            Expression::Variable(_, var) => {
                let Some((src_op, c_type)) = vars.find(&var) else {
                    todo!("error: var does not exist")
                };
                let result_type = c_type.clone();
                let dst_op = vars.push(None, c_type.clone());
                Ok((vec![Instruction::Mov(dst_op.clone(), src_op)], Some((dst_op, result_type))))
            },
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
            Expression::Addition(_, add) => add.compile_mut(vars),
            Expression::Subtraction(_, _) => todo!(),
            Expression::Multiplication(_, _) => todo!(),
            Expression::Division(_, _) => todo!(),
            Expression::Borrow(_, _) => todo!(),
            Expression::Negation(_, _) => todo!(),
            Expression::Deref(_, deref) => deref.compile_mut(vars),
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

impl Addition {
    pub fn compile_mut(self, vars: &mut Vars) -> Result<(Vec<Instruction>, Option<(Operand, CeriumType)>), CompilerError> {
        //todo copy code and call this from compile(_ref) instead
        self.compile(vars)
    }
}

impl Deref {
    pub fn compile_mut(self, vars: &mut Vars) -> Result<(Vec<Instruction>, Option<(Operand, CeriumType)>), CompilerError> {
        //TODO: EITHER compile_mut deref OR compile read
        /*
        let (mut asm, Some((op, c_type))) = self.inner.compile_mut(vars)? else {
            todo!()
        };
        let CeriumType::Pointer(box inner_type) = c_type else {
            todo!()
        };
        asm.push(Instruction::Read(op.clone(), op.clone()));
        Ok((asm, Some((op, inner_type))))
         */
        vars.begin_scope();
        let (mut asm, Some((op, c_type))) = self.inner.compile(vars)? else {
            todo!()
        };
        let CeriumType::Pointer(box inner_type) = c_type else {
            todo!()
        };
        vars.end_scope();
        let addr = vars.push(None, inner_type.clone());
        asm.push(Instruction::Read(addr.clone(), op));
        Ok((asm, Some((addr, inner_type))))
    }
}