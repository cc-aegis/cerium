use crate::compiler::assembly::{Instruction, Operand, Register};
use crate::compiler::vars::Vars;
use crate::error::CompilerError;
use crate::parser::ast::{Addition, Deref, Expression, Iter, Subtraction, TypeAlias};
use crate::parser::cerium_type::CeriumType;

//TODO: rename to compile_ref?

impl Expression {
    pub(crate) fn compile(self, vars: &mut Vars) -> Result<(Vec<Instruction>, Option<(Operand, CeriumType)>), CompilerError> {
        match self {
            Expression::Scope(_, _) => todo!(),
            Expression::TypeCast(_, _) => todo!(),
            Expression::TypeAlias(_, type_alias) => type_alias.compile(vars),
            Expression::Integer(_, int) => {
                Ok((Vec::new(), Some((Operand::Direct(Register::RN(int)), CeriumType::U16))))
            },
            Expression::Float(_, _) => todo!(),
            Expression::Boolean(_, boolean) => Ok((Vec::new(), Some((Operand::Direct(Register::RN(format!("#{boolean}"))), CeriumType::Bool)))),
            Expression::Nullptr(_) => todo!(),
            Expression::Variable(_, var) => {
                match vars.find(&var) {
                    Some((op, c_type)) => Ok((Vec::new(), Some((op, c_type.clone())))),
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
            Expression::Addition(_, add) => add.compile(vars),
            Expression::Subtraction(_, sub) => sub.compile(vars),
            Expression::Multiplication(_, _) => todo!(),
            Expression::Division(_, _) => todo!(),
            Expression::Borrow(_, _) => todo!(),
            Expression::Negation(_, _) => todo!(),
            Expression::Deref(_, deref) => deref.compile(vars),
            Expression::Iter(_, iter) => iter.compile(vars),
            Expression::Inversion(_, _) => todo!(),
            Expression::Let(_, _) => todo!(),
            Expression::If(_, _) => todo!(),
            Expression::For(_, _) => todo!(),
            Expression::While(_, _) => todo!(),
            Expression::Loop(_, _) => todo!(),
        }
    }
}

impl TypeAlias {
    pub fn compile(self, vars: &mut Vars) -> Result<(Vec<Instruction>, Option<(Operand, CeriumType)>), CompilerError> {
        let (asm, Some((op, _))) = self.value.compile(vars)? else {
            todo!("error: unit type")
        };
        Ok((asm, Some((op, *self.target_type))))
    }
}

impl Addition {
    pub fn compile(self, vars: &mut Vars) -> Result<(Vec<Instruction>, Option<(Operand, CeriumType)>), CompilerError> {
        let (mut result_asm, Some((lhs_op, lhs_type))) = self.lhs.compile_mut(vars)? else {
            todo!("error")
        };
        vars.begin_scope();
        let (rhs_asm, Some((rhs_op, rhs_type))) = self.rhs.compile(vars)? else {
            todo!()
        };
        vars.end_scope();
        let final_inst = match (&lhs_type, rhs_type) {
            (CeriumType::U16, CeriumType::U16) => Instruction::Add(lhs_op.clone(), rhs_op),
            (CeriumType::I16, CeriumType::I16) => Instruction::Add(lhs_op.clone(), rhs_op),
            (CeriumType::F16, CeriumType::F16) => Instruction::Fadd(lhs_op.clone(), rhs_op),
            (CeriumType::Pointer(_), CeriumType::U16) => Instruction::Add(lhs_op.clone(), rhs_op),
            _ => todo!("error")
        };
        result_asm.extend(rhs_asm);
        result_asm.push(final_inst);
        Ok((result_asm, Some((lhs_op, lhs_type))))

    }
}

impl Subtraction {
    pub fn compile(self, vars: &mut Vars) -> Result<(Vec<Instruction>, Option<(Operand, CeriumType)>), CompilerError> {
        let (mut result_asm, Some((lhs_op, lhs_type))) = self.lhs.compile_mut(vars)? else {
            todo!("error")
        };
        vars.begin_scope();
        let (rhs_asm, Some((rhs_op, rhs_type))) = self.rhs.compile(vars)? else {
            todo!()
        };
        vars.end_scope();
        let final_inst = match (&lhs_type, rhs_type) {
            (CeriumType::U16, CeriumType::U16) => Instruction::Sub(lhs_op.clone(), rhs_op),
            (CeriumType::I16, CeriumType::I16) => Instruction::Sub(lhs_op.clone(), rhs_op),
            (CeriumType::F16, CeriumType::F16) => Instruction::Fsub(lhs_op.clone(), rhs_op),
            (CeriumType::Pointer(_), CeriumType::U16) => Instruction::Sub(lhs_op.clone(), rhs_op),
            _ => todo!("error")
        };
        result_asm.extend(rhs_asm);
        result_asm.push(final_inst);
        Ok((result_asm, Some((lhs_op, lhs_type))))

    }
}

impl Deref {
    pub fn compile(self, vars: &mut Vars) -> Result<(Vec<Instruction>, Option<(Operand, CeriumType)>), CompilerError> {
        self.compile_mut(vars)
    }
}

impl Iter {
    pub fn compile(self, vars: &mut Vars) -> Result<(Vec<Instruction>, Option<(Operand, CeriumType)>), CompilerError> {
        let (mut asm, Some((op, outer_type))) = self.inner.compile(vars)? else {
            todo!("error")
        };
        let CeriumType::Pointer(box inner_type) = outer_type else {
            todo!("error")
        };
        let target_op = vars.push(None, inner_type.clone());
        asm.push(Instruction::Readitr(target_op.clone(), op));
        Ok((asm, Some((target_op, inner_type))))
    }
}