use std::collections::HashMap;
use crate::compiler::assembly::{Instruction, Operand, Register};
use crate::compiler::error::MismatchedReturnTypeError;
use crate::compiler::vars::Vars;
use crate::error::CompilerError;
use crate::{lexer, parser};
use crate::parser::ast::{Borrow, Const, Definition, Expression, Function, Program, Qualifier, Scope};
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
    let (_, globals) = program.parse_structure();
    let mut result = Vec::new();
    for def in program.definitions {
        result.extend(def.compile(&globals)?);
    }
    Ok(result)
}

impl Definition {
    fn compile(self, globals: &HashMap<Qualifier, CeriumType>) -> Result<Vec<Instruction>, CompilerError> {
        match self {
            Definition::Function(func) => func.compile(globals),
            Definition::Struct(_) => Ok(Vec::new()),
            Definition::Const(const_val) => const_val.compile(),
        }
    }
}

impl Function {
    fn compile(self, globals: &HashMap<Qualifier, CeriumType>) -> Result<Vec<Instruction>, CompilerError> {
        let mut result = vec![Instruction::Label(self.name.to_string())];
        let result_range = match &*self.body {
            Expression::Scope(_, Scope { value: Some(value), .. }) => value.range(),
            expression => expression.range(),
        };
        let mut vars = Vars::new(globals, self.parameters);
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

impl Const {
    fn compile(self) -> Result<Vec<Instruction>, CompilerError> {
        /*let mut vars = Vars::new(Vec::new());

        let (asm, Some((op, c_type))) = self.value.compile(&mut vars)? else {
            todo!("error: unit constant")
        };
        if c_type != self.const_type {
            todo!("error: mismatched type")
        };
        if !asm.is_empty() {

        }*/
        let (asm, op, c_type) = Self::const_compile(self.value, &mut 0)?;
        if c_type != self.const_type {
            todo!("error: mismatched types")
        };
        let mut result = vec![
            Instruction::Define(self.name.to_string(), op)
        ];
        result.extend(asm);
        Ok(result)
    }

    fn const_compile(expression: Expression, counter: &mut usize) -> Result<(Vec<Instruction>, Operand, CeriumType), CompilerError> {
        match expression {
            Expression::Borrow(_, Borrow { inner }) => {
                let id = *counter;
                *counter += 1;
                let label = format!(".L{id}");
                let (inner_asm, op, c_type) = Self::const_compile(*inner, counter)?;
                let mut asm = vec![
                    Instruction::Label(label.clone()),
                    Instruction::Dw(vec![op]),
                ];
                asm.extend(inner_asm);
                Ok((asm, Operand::Direct(Register::RN(label)), CeriumType::Pointer(Box::new(c_type))))
            },
            Expression::Integer(_, int) => Ok((
                Vec::new(),
                Operand::Direct(Register::RN(int)),
                CeriumType::Pointer(Box::new(CeriumType::U16))
            )),
            Expression::Float(_, float) => Ok((
                Vec::new(),
                Operand::Direct(Register::RN(float)),
                CeriumType::Pointer(Box::new(CeriumType::F16))
            )),
            Expression::Boolean(_, boolean) => Ok((
                Vec::new(),
                Operand::Direct(Register::RN(String::from(if boolean { "true" } else { "false" }))),
                CeriumType::Pointer(Box::new(CeriumType::Bool))
            )),
            Expression::Nullptr(_) => Ok((
                Vec::new(),
                Operand::Direct(Register::RN(String::from("0"))),
                CeriumType::Pointer(Box::new(CeriumType::Any))
            )),
            _ => todo!("error: unexpected or not yet implemented")
            // todo: add support for const expressions / accept list of globals as param
        }
    }
}