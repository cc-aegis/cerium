use std::rc::Rc;

pub enum IntermediateOperand {
    Parameter(usize),
    Variable(usize),
    Argument(usize),
    Result,
    F16(f32),
    I16(i16),
    U16(u16),
    Bool(bool),
    Char(u8),
    Const(Rc<str>),
}

pub enum Instruction<Operand> {
    Mov(Operand, Operand),
}
