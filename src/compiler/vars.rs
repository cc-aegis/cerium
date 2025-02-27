use std::ops::Sub;
use crate::compiler::assembly::{Operand, Register};
use crate::parser::ast::Qualifier;
use crate::parser::cerium_type::CeriumType;

struct Vars {
    globals: (),
    parameters: Vec<(Option<Qualifier>, CeriumType)>,
    vars: Vec<(Option<Qualifier>, CeriumType)>,
    max_size: usize,
    scopes: Vec<usize>,
}

fn var_op(idx: usize) -> Operand {
    match idx {
        0 => Operand::Direct(Register::R1),
        1 => Operand::Direct(Register::R2),
        2 => Operand::Direct(Register::R3),
        3 => Operand::Direct(Register::R4),
        4 => Operand::Direct(Register::R5),
        5 => Operand::Direct(Register::R6),
        6 => Operand::Direct(Register::R7),
        7.. => Operand::Indirect(Register::RN(idx.sub(7).to_string())),
    }
}

impl Vars {
    fn push(&mut self, name: Option<Qualifier>, var_type: CeriumType) -> Operand {
        self.vars.push((name, var_type));
        self.max_size = self.max_size.max(self.vars.len());
        var_op(self.vars.len() - 1)
    }

    fn pop(&mut self) {
        let _ = self.vars.pop();
    }

    fn find(&self, name: &Qualifier) -> Option<(Operand, &CeriumType)> {
        for (idx, (var_name, var_type)) in self.vars.iter().enumerate().rev() {
            if var_name.as_ref().is_some_and(|it| *it == *name) {
                return Some((var_op(idx), var_type));
            }
        }
        for (idx, (var_name, var_type)) in self.parameters.iter().enumerate().rev() {
            if var_name.as_ref().is_some_and(|it| *it == *name) {
                return Some((Operand::Indirect(Register::RN((idx as isize).sub(3).to_string())), var_type));
            }
        }
        //todo: globals
        None
    }

    fn begin_scope(&mut self) {
        self.scopes.push(self.vars.len());
    }

    fn end_scope(&mut self) {
        if let Some(len) = self.scopes.pop() {
            self.scopes.truncate(len);
        }
    }
}