use crate::compiler::assembly::{Instruction, Operand, Register};
use crate::compiler::vars::Vars;
use crate::error::CompilerError;
use crate::parser::ast::{Addition, Expression, For, Inversion, Scope, Subtraction};
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
            Expression::Subtraction(_, subtraction) => subtraction.compile_into(vars, target),
            Expression::Multiplication(_, _) => todo!(),
            Expression::Division(_, _) => todo!(),
            Expression::Borrow(_, _) => todo!(),
            Expression::Negation(_, _) => todo!(),
            Expression::Deref(_, _) => todo!(),
            Expression::Iter(_, _) => todo!(),
            Expression::Inversion(_, _) => todo!(),
            Expression::Let(_, _) => todo!(),
            Expression::If(_, _) => todo!(),
            Expression::For(_, for_loop) => for_loop.compile_into(vars, target),
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
            result.append(&mut instruction.compile_unit(vars)?);
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

impl Subtraction {
    fn compile_into(self, vars: &mut Vars, target: Operand) -> Result<(Vec<Instruction>, Option<CeriumType>), CompilerError> {
        let (lhs_inst, lhs_type) = self.lhs.compile_into(vars, target.clone())?;
        let (rhs_inst, rhs_op_type) = self.rhs.compile(vars)?;
        match (lhs_type, rhs_op_type) {
            (Some(CeriumType::U16), Some((rhs_op, CeriumType::U16))) => {
                let mut result = Vec::new();
                result.extend(lhs_inst);
                result.extend(rhs_inst);
                result.push(Instruction::Sub(target, rhs_op));
                Ok((result, Some(CeriumType::U16)))
            },
            (Some(CeriumType::I16), Some((rhs_op, CeriumType::I16))) => {
                let mut result = Vec::new();
                result.extend(lhs_inst);
                result.extend(rhs_inst);
                result.push(Instruction::Sub(target, rhs_op));
                Ok((result, Some(CeriumType::I16)))
            },
            (Some(CeriumType::F16), Some((rhs_op, CeriumType::F16))) => {
                let mut result = Vec::new();
                result.extend(lhs_inst);
                result.extend(rhs_inst);
                result.push(Instruction::Fsub(target, rhs_op));
                Ok((result, Some(CeriumType::F16)))
            },
            (Some(return_type), Some((rhs_op, CeriumType::U16))) if matches!(return_type, CeriumType::Pointer(_)) => {
                let mut result = Vec::new();
                result.extend(lhs_inst);
                result.extend(rhs_inst);
                result.push(Instruction::Sub(target, rhs_op));
                Ok((result, Some(return_type)))
            }
            _ => {
                todo!("error incompatible types for operator 'Plus': 'Some(...)' and 'Some(...)'")
            }
        }
    }
}

impl For {
    fn compile_into(self, vars: &mut Vars, target: Operand) -> Result<(Vec<Instruction>, Option<CeriumType>), CompilerError> {
        // limit 0 step -1: [init-setup -> $0] jmp .cond; .loop: [body] .cond: jrnzdec $1 .loop
        dbg!(&self);
        assert!(
            self.initialization.is_none()
                && self.limit.is_some_and(|it| matches!(it, box Expression::Integer(_, num) if &num == "0"))
                && self.step.is_some_and(|it| matches!(it, box Expression::Inversion(_, Inversion { inner: box Expression::Integer(_, num) }) if &num == "1"))
        );
        let (counter_addr, counter_type) = vars.find(&self.counter).ok_or_else(|| {
            todo!()
        })?;
        let (CeriumType::U16 | CeriumType::I16 | CeriumType::Pointer(_)) = &counter_type else {
            todo!()
        };

        vars.begin_scope();
        let body = self.body.compile_unit(vars)?;
        vars.end_scope();

        let l_cond = vars.label();
        let o_cond = Operand::Direct(Register::RN(l_cond.clone()));
        let l_loop = vars.label();
        let o_loop = Operand::Direct(Register::RN(l_loop.clone()));

        let mut full_body = vec![Instruction::Jmp(o_cond), Instruction::Label(l_loop)];
        full_body.extend(body);
        full_body.extend(vec![Instruction::Label(l_cond), Instruction::Jrnzdec(counter_addr, o_loop)]);

        Ok((
            full_body,
            None
        ))
    }
}