use std::ops::Sub;
use crate::compiler::assembly::{Operand, Register};
use crate::parser::ast::Qualifier;
use crate::parser::cerium_type::CeriumType;

pub struct Vars {
    globals: (),
    parameters: Vec<(Qualifier, CeriumType)>,
    vars: Vec<(Option<Qualifier>, CeriumType)>,
    max_size: usize,
    scopes: Vec<usize>,
    label_counter: usize,
}

impl Vars {
    pub fn new(parameters: Vec<(Qualifier, CeriumType)>) -> Self {
        Vars {
            globals: (),
            parameters,
            vars: Vec::new(),
            max_size: 0,
            scopes: Vec::new(),
            label_counter: 0,
        }
    }
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
    pub fn label(&mut self) -> String {
        self.label_counter += 1;
        format!(".L{}", self.label_counter - 1)
    }
    pub fn push(&mut self, name: Option<Qualifier>, var_type: CeriumType) -> Operand {
        self.vars.push((name, var_type));
        self.max_size = self.max_size.max(self.vars.len());
        var_op(self.vars.len() - 1)
    }
    
    pub fn alter_top_type(&mut self, var_type: CeriumType) {
        if let Some((_, last)) = self.vars.last_mut() {
            *last = var_type;
        }
    }

    pub fn pop(&mut self) {
        let _ = self.vars.pop();
    }

    pub fn find(&self, name: &Qualifier) -> Option<(Operand, &CeriumType)> {
        for (idx, (var_name, var_type)) in self.vars.iter().enumerate().rev() {
            if var_name.as_ref().is_some_and(|it| *it == *name) {
                return Some((var_op(idx), var_type));
            }
        }
        let offset = self.parameters.len() as isize + 2;
        for (idx, (var_name, var_type)) in self.parameters.iter().enumerate().rev() {
            if *var_name == *name {
                return Some((Operand::Indirect(Register::RN((idx as isize).sub(offset).to_string())), var_type));
            }
        }
        //todo: globals
        None
    }

    pub fn begin_scope(&mut self) {
        self.scopes.push(self.vars.len());
    }

    pub fn end_scope(&mut self) {
        if let Some(len) = self.scopes.pop() {
            self.scopes.truncate(len);
        }
    }
}