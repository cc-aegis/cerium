use crate::compiler::asm::{Instruction, IntermediateOperand};
use crate::compiler::context::CompilerContext;
use crate::error::CompilerError;
use crate::parser::ast::Definition;
use crate::parser::cerium_type::CeriumType;

mod asm;
mod error;
mod compile_into;
mod context;

/*
trait CompileRef {
    fn compile(self, ctx: &mut CompilerContext) -> Result<(Vec<Instruction<IntermediateOperand>>, Option<(IntermediateOperand, CeriumType)>), CompilerError>;
}

trait CompileMut {
    fn compile_mut(self, ctx: &mut CompilerContext) -> Result<(Vec<Instruction<IntermediateOperand>>, Option<(IntermediateOperand, CeriumType)>), CompilerError>;
}

trait CompileInto {
    fn compile_into(self, target: IntermediateOperand, ctx: &mut CompilerContext) -> Result<(Vec<Instruction<IntermediateOperand>>, Option<CeriumType>), CompilerError>;
}

trait CompileUnit {
    fn compile_unit(self, ctx: &mut CompilerContext) -> Result<Vec<Instruction<IntermediateOperand>>, CompilerError>;
}
 */

impl Definition {
    pub fn compile(self, context: &mut CompilerContext) -> Result<Vec<Instruction<IntermediateOperand>>, CompilerError> {
        match self {
            Definition::Function(function) => todo!(),
            Definition::Const(_) => todo!(),
            Definition::Struct(_) => todo!(),
        }
    } 
}