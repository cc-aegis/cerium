use std::fmt::{write, Display, Formatter};

#[derive(Debug)]
pub enum Instruction {
    Label(String),
    Define(String, Operand),
    Dw(Vec<Operand>),
    Mov(Operand, Operand),
    Add(Operand, Operand),
    Sub(Operand, Operand),
    Fadd(Operand, Operand),
    Fsub(Operand, Operand),
    Jmp(Operand),
    Jrnzdec(Operand, Operand),
    Readitr(Operand, Operand),
    Writeitr(Operand, Operand),
    Write(Operand, Operand),
    Read(Operand, Operand),
    Ret,
}

#[derive(Debug, Clone)]
pub enum Operand {
    Direct(Register),
    Indirect(Register),
}

#[derive(Debug, Clone)]
pub enum Register {
    R0, R1, R2, R3, R4, R5, R6, R7,
    RR, RI, RB, RS, RG, RD, RF, RN(String),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Label(label) => write!(f, "{label}:"),
            Instruction::Define(name, op) => write!(f, "#define {name} {op}"),
            Instruction::Dw(operands) => {
                write!(f, "    dw")?;
                for operand in operands {
                    write!(f, " {}", operand)?;
                }
                Ok(())
            },
            Instruction::Mov(lhs, rhs) => write!(f, "    mov {lhs} {rhs}"),
            Instruction::Add(lhs, rhs) => write!(f, "    add {lhs} {rhs}"),
            Instruction::Sub(lhs, rhs) => write!(f, "    sub {lhs} {rhs}"),
            Instruction::Fadd(lhs, rhs) => write!(f, "    fadd {lhs} {rhs}"),
            Instruction::Fsub(lhs, rhs) => write!(f, "    fsub {lhs} {rhs}"),
            Instruction::Jmp(label) => write!(f, "    jmp {label}"),
            Instruction::Jrnzdec(dst, src) => write!(f, "    jrnzdec {dst} {src}"),
            Instruction::Readitr(dst, src) => write!(f, "    readitr {dst} {src}"),
            Instruction::Writeitr(cond, label) => write!(f, "    writeitr {cond} {label}"),
            Instruction::Write(cond, label) => write!(f, "    write {cond} {label}"),
            Instruction::Read(cond, label) => write!(f, "    read {cond} {label}"),
            Instruction::Ret => write!(f, "    ret"),
        }
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Direct(op) => write!(f, "{op}"),
            Operand::Indirect(op) => write!(f, "[{op}]"),
        }
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Register::R0 => write!(f, "r0"),
            Register::R1 => write!(f, "r1"),
            Register::R2 => write!(f, "r2"),
            Register::R3 => write!(f, "r3"),
            Register::R4 => write!(f, "r4"),
            Register::R5 => write!(f, "r5"),
            Register::R6 => write!(f, "r6"),
            Register::R7 => write!(f, "r7"),
            Register::RR => write!(f, "rr"),
            Register::RI => write!(f, "ri"),
            Register::RB => write!(f, "rb"),
            Register::RS => write!(f, "rs"),
            Register::RG => write!(f, "rg"),
            Register::RD => write!(f, "rd"),
            Register::RF => write!(f, "rf"),
            Register::RN(r) => write!(f, "{r}"),
        }
    }
}