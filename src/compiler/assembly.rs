#[derive(Debug)]
pub enum Instruction {
    Label(String),
    Mov(Operand, Operand),
    Add(Operand, Operand),
    Ret,
}

#[derive(Debug)]
pub enum Operand {
    Direct(Register),
    Indirect(Register),
}

#[derive(Debug)]
pub enum Register {
    R0, R1, R2, R3, R4, R5, R6, R7,
    RR, RI, RB, RS, RG, RD, RF, RN(String),
}