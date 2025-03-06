use crate::compiler::assembly::{Instruction, Operand, Register};
use crate::compiler::error::MismatchedReturnTypeError;
use crate::compiler::vars::Vars;
use crate::error::CompilerError;
use crate::{lexer, parser};
use crate::parser::ast::{Definition, Expression, Function, Program, Scope};
use crate::parser::cerium_type::CeriumType;

mod declarations;
mod compile;
mod compile_mut;
mod compile_into;
mod assembly;
pub mod error;
mod vars;
mod compile_unit;

pub fn compile(code: &str) -> Result<Vec<Instruction>, CompilerError> {
    let lexer = lexer::Lexer::new(code);
    let mut parser = parser::Parser::new(lexer);
    let program = dbg!(parser.parse()?);
    let _ = program.parse_structure();
    let mut result = Vec::new();
    for def in program.definitions {
        result.extend(def.compile()?);
    }
    Ok(result)
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
            Expression::Scope(_, Scope { value: Some(value), .. }) => value.range(),
            expression => expression.range(),
        };
        let mut vars = Vars::new(self.parameters);
        let (mut body, return_type) = self.body.compile_into(&mut vars, Operand::Direct(Register::R0))?;
        result.append(&mut body);
        result.push(Instruction::Ret);
        if self.return_type == return_type {
            Ok(result)
        } else {
            Err(CompilerError::MismatchedReturnTypeError(MismatchedReturnTypeError {
                function_name: self.name,
                expected: self.return_type,
                actual: return_type,
                range: result_range,
            }))
        }
    }
}