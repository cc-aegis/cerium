use crate::compiler::assembly::{Instruction, Operand, Register};
use crate::compiler::error::MismatchedTypesError;
use crate::error::CompilerError;
use crate::parser::ast::{Definition, Expression, Function, Program};
use crate::parser::cerium_type::CeriumType;

mod declarations;
mod compile;
mod compile_mut;
mod compile_into;
mod assembly;
pub mod error;
mod vars;

pub fn compile(program: Program) {
    dbg!(program.parse_structure());
    for def in program.definitions {
        dbg!(def.compile());
    }
}

impl Definition {
    fn compile(self) -> Result<Vec<Instruction>, CompilerError> {
        match self {
            Definition::Function(func) => func.compile(),
            Definition::Struct(_) => Ok(Vec::new()),
        }
    }
}

impl Function {
    fn compile(self) -> Result<Vec<Instruction>, CompilerError> {
        let mut result = vec![Instruction::Label(self.name.to_string())];
        let result_range = match &*self.body {
            Expression::Scope(range, scope) => scope.instructions
                .last()
                .map(|e| e.range())
                .unwrap_or(range.clone()),
            expression => expression.range(),
        };
        let (mut body, return_type) = self.body.compile_into(Operand::Direct(Register::R0))?;
        result.append(&mut body);
        if self.return_type == return_type {
            Ok(result)
        } else {
            Err(CompilerError::MismatchedTypesError(MismatchedTypesError {
                expected: self.return_type,
                actual: return_type,
                range: result_range,
            }))
        }
    }
}