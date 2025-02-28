use crate::compiler::assembly::{Instruction, Operand, Register};
use crate::compiler::vars::Vars;
use crate::error::CompilerError;
use crate::parser::ast::{Addition, Expression, Scope};
use crate::parser::cerium_type::CeriumType;

impl Expression {
    pub(crate) fn compile_into(self, vars: &mut Vars, target: Operand) -> Result<(Vec<Instruction>, Option<CeriumType>), CompilerError> {
        match self {
            Expression::Scope(_, scope) => scope.compile_into(vars, target),
            Expression::TypeCast(_, _) => todo!(),
            Expression::TypeAlias(_, _) => todo!(),
            Expression::Integer(_, int) => {
                Ok((vec![Instruction::Mov(target, Operand::Direct(Register::RN(int)))], Some(CeriumType::U16)))
            },
            Expression::Float(_, _) => todo!(),
            Expression::Boolean(_, _) => todo!(),
            Expression::Nullptr(_) => todo!(),
            Expression::Variable(_, var) => {
                match vars.find(&var) {
                    Some((op, c_type)) => {
                        Ok((vec![Instruction::Mov(target, op)], Some(c_type.clone())))
                    }
                    None => todo!("error"),
                }
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
            Expression::Addition(_, addition) => addition.compile_into(vars, target),
            Expression::Subtraction(_, _) => todo!(),
            Expression::Multiplication(_, _) => todo!(),
            Expression::Division(_, _) => todo!(),
            Expression::Borrow(_, _) => todo!(),
            Expression::Negation(_, _) => todo!(),
            Expression::Deref(_, _) => todo!(),
            Expression::Let(_, _) => todo!(),
            Expression::If(_, _) => todo!(),
            Expression::For(_, _) => todo!(),
            Expression::While(_, _) => todo!(),
            Expression::Loop(_, _) => todo!(),
        }
    }
}

impl Scope {
    fn compile_into(self, vars: &mut Vars, target: Operand) -> Result<(Vec<Instruction>, Option<CeriumType>), CompilerError> {
        let mut result = Vec::new();
        vars.begin_scope();
        for instruction in self.instructions {
            result.append(&mut instruction.compile(vars)?.0);
        }
        let return_type = if let Some(value) = self.value {
            let (mut inst, return_type) = value.compile_into(vars, target)?;
            result.append(&mut inst);
            return_type
        } else {
            None
        };
        vars.end_scope();
        Ok((result, return_type))
    }
}

impl Addition {
    fn compile_into(self, vars: &mut Vars, target: Operand) -> Result<(Vec<Instruction>, Option<CeriumType>), CompilerError> {
        let (lhs_inst, lhs_type) = self.lhs.compile_into(vars, target.clone())?;
        let (rhs_inst, rhs_op_type) = self.rhs.compile(vars)?;
        match (lhs_type, rhs_op_type) {
            (Some(CeriumType::U16), Some((rhs_op, CeriumType::U16))) => {
                let mut result = Vec::new();
                result.extend(lhs_inst);
                result.extend(rhs_inst);
                result.push(Instruction::Add(target, rhs_op));
                Ok((result, Some(CeriumType::U16)))
            },
            (Some(CeriumType::I16), Some((rhs_op, CeriumType::I16))) => {
                let mut result = Vec::new();
                result.extend(lhs_inst);
                result.extend(rhs_inst);
                result.push(Instruction::Add(target, rhs_op));
                Ok((result, Some(CeriumType::I16)))
            },
            (Some(CeriumType::F16), Some((rhs_op, CeriumType::F16))) => {
                let mut result = Vec::new();
                result.extend(lhs_inst);
                result.extend(rhs_inst);
                result.push(Instruction::Fadd(target, rhs_op));
                Ok((result, Some(CeriumType::F16)))
            },
            (Some(return_type), Some((rhs_op, CeriumType::U16))) if matches!(return_type, CeriumType::Pointer(_)) => {
                let mut result = Vec::new();
                result.extend(lhs_inst);
                result.extend(rhs_inst);
                result.push(Instruction::Add(target, rhs_op));
                Ok((result, Some(return_type)))
            }
            _ => {
                todo!("error incompatible types for operator 'Plus': 'Some(...)' and 'Some(...)'")
            }
        }
    }
}