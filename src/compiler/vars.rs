use std::collections::HashMap;
use std::ops::Sub;
use crate::compiler::assembly::{Annotation, Instruction, Operand, Register};
use crate::parser::ast::Qualifier;
use crate::parser::cerium_type::CeriumType;

pub struct Vars<'a> {
    globals: &'a HashMap<Qualifier, CeriumType>,
    parameters: Vec<(Qualifier, CeriumType)>,
    vars: Vec<(Option<Qualifier>, CeriumType)>,
    max_size: usize,
    scopes: Vec<usize>,
    label_counter: usize,
}

impl <'a> Vars<'a> {
    pub fn new(globals: &'a HashMap<Qualifier, CeriumType>, parameters: Vec<(Qualifier, CeriumType)>) -> Self {
        Vars {
            globals,
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

impl Vars<'_> {
    pub fn label(&mut self) -> String {
        self.label_counter += 1;
        format!(".L{}", self.label_counter - 1)
    }
    pub fn push(&mut self, name: Option<Qualifier>, var_type: CeriumType) -> Operand {
        self.vars.push((name, var_type));
        dbg!(self.vars.len());
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
        for (global_name, global_type) in self.globals {
            if *global_name == *name {
                return Some((Operand::Direct(Register::RN(global_name.to_string())), global_type))
            }
        }
        None
    }

    pub fn begin_scope(&mut self) {
        self.scopes.push(self.vars.len());
    }

    pub fn end_scope(&mut self) {
        if let Some(len) = self.scopes.pop() {
            self.vars.truncate(len);
        }
    }
    
    pub fn collect_affixes(&mut self) -> (Vec<Instruction>, Vec<Instruction>) {
        // collecting push $0 push $1 into pusht $0 $1 will be done by post-optimizer
        // ..push; add rb <offset>; ..; sub rb <offset>; ..pop
        
        //TODO: problem: push to store overlaps with push for args -> ??? add to bp to adjust??? ????
        let mut push: Vec<_> = (0..=6)
            .filter(|idx| *idx < self.max_size)
            .map(|idx| Instruction::Push(var_op(idx)))
            .collect();
        if self.max_size > 7 {
            push.push(Instruction::Add(
                Operand::Direct(Register::RS),
                Operand::Direct(Register::RN(self.max_size.sub(7).to_string())),
            ));
        }
        if self.max_size > 0 {
            push.push(Instruction::Annotation(Annotation::NonNegativeStackOffset(
                self.max_size.min(7) as isize
            )))
        }
        let mut pop = Vec::new();
        if self.max_size > 0 {
            pop.push(Instruction::Annotation(Annotation::NonNegativeStackOffset(
                -(self.max_size.min(7) as isize)
            )))
        }
        if self.max_size > 7 {
            pop.push(Instruction::Sub(
                Operand::Direct(Register::RS),
                Operand::Direct(Register::RN(self.max_size.sub(7).to_string())),
            ));
        }
        pop.extend((0..=6)
            .filter(|idx| *idx < self.max_size)
            .map(|idx| Instruction::Pop(var_op(idx))));

        (push, pop)
    }
}