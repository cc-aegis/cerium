use crate::compiler::assembly::{Instruction, Operand, Register};
use crate::compiler::error::{InvalidDerefError, MismatchedAssignTypeError};
use crate::compiler::vars::Vars;
use crate::error::CompilerError;
use crate::parser::ast::{Assignment, Expression, Let, Scope};
use crate::parser::cerium_type::CeriumType;

impl Expression {
    pub(crate) fn compile_unit(self, vars: &mut Vars) -> Result<Vec<Instruction>, CompilerError> {
        match self {
            Expression::Scope(_, scope) => scope.compile_unit(vars),
            Expression::TypeCast(_, _) => todo!(),
            Expression::TypeAlias(_, _) => todo!(),
            Expression::Integer(_, int) => {
                Ok(Vec::new())
            },
            Expression::Float(_, _) => todo!(),
            Expression::Boolean(_, _) => todo!(),
            Expression::Nullptr(_) => todo!(),
            Expression::Variable(_, var) => {
                match vars.find(&var) {
                    Some((op, c_type)) => {
                        Ok(Vec::new())
                    }
                    None => todo!("error"),
                }
            },
            Expression::FieldAccess(_, _) => todo!(),
            Expression::ArrayAccess(_, _) => todo!(),
            Expression::FunctionCall(_, _) => todo!(),
            Expression::Assignment(_, assign) => assign.compile_unit(vars),
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
            Expression::Addition(_, addition) => todo!(),
            Expression::Subtraction(_, subtraction) => todo!(),
            Expression::Multiplication(_, _) => todo!(),
            Expression::Division(_, _) => todo!(),
            Expression::Borrow(_, _) => todo!(),
            Expression::Negation(_, _) => todo!(),
            Expression::Deref(_, _) => todo!(),
            Expression::Iter(_, _) => todo!(),
            Expression::Inversion(_, _) => todo!(),
            Expression::Let(_, decl) => decl.compile_unit(vars),
            Expression::If(_, _) => todo!(),
            Expression::For(_, for_loop) => todo!(),
            Expression::While(_, _) => todo!(),
            Expression::Loop(_, _) => todo!(),
        }
    }
}


impl Scope {
    fn compile_unit(self, vars: &mut Vars) -> Result<Vec<Instruction>, CompilerError> {
        let mut result = Vec::new();
        vars.begin_scope();
        for instruction in self.instructions {
            result.append(&mut instruction.compile_unit(vars)?);
        }
        if let Some(value) = self.value {
            let mut inst = value.compile_unit(vars)?;
            result.append(&mut inst);
        }
        vars.end_scope();
        Ok(result)
    }
}

impl Assignment {
    //x = y, *x = y, ^x = y, x[idx] = y, x.field = y
    fn compile_unit(self, vars: &mut Vars) -> Result<Vec<Instruction>, CompilerError> {
        match self.target {
            box Expression::Variable(_, var) => {
                todo!()
            },
            box Expression::Iter(range, iter) => {
                let ranges = (iter.inner.range(), self.value.range());
                vars.begin_scope();
                let (lhs_asm, Some((lhs_op, lhs_type))) = iter.inner.compile(vars)? else {
                    return Err(CompilerError::InvalidDerefError(InvalidDerefError {
                        range,
                        found: None,
                    }));
                };
                let CeriumType::Pointer(box inner_type) = lhs_type else {
                    return Err(CompilerError::InvalidDerefError(InvalidDerefError {
                        range,
                        found: Some(lhs_type),
                    }));
                };
                let value_range = self.value.range();
                let (rhs_asm, Some((rhs_op, rhs_type))) = self.value.compile(vars)? else {
                    todo!("unit assign error")
                };
                if inner_type != rhs_type {
                    return Err(CompilerError::MismatchedAssignTypeError(MismatchedAssignTypeError {
                        dst_range: ranges.0,
                        dst_type: Some(inner_type),
                        src_range: ranges.1,
                        src_type: Some(rhs_type),
                    }))
                }
                let mut result = lhs_asm;
                result.extend(rhs_asm);
                result.push(Instruction::Writeitr(lhs_op, rhs_op));
                vars.end_scope();
                Ok(result)
            }
            _ => todo!("error or generate code")
        }
    }
}

impl Let {
    fn compile_unit(self, vars: &mut Vars) -> Result<Vec<Instruction>, CompilerError> {
        let addr = vars.push(Some(self.name), CeriumType::Any);
        vars.begin_scope();
        let (asm, c_type) = self.value.compile_into(vars, addr.clone())?;
        vars.end_scope();
        let Some(c_type) = c_type else {
            todo!("unit error")
        };
        vars.alter_top_type(c_type.clone());
        Ok(asm)
    }
}