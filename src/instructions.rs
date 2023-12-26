use crate::value::CHSValue;


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Opcode {
    Halt = 0,
    Pushi,
    Pushf,
    Pop,
    Dup,
    Swap,
    Bind,
    BindPush,
    Unbind,

    Add,
    Minus,
    Mul,
    Div,

    Jmp,
    JmpIf,

    Eq,
    Gt,
    Lt,
    Gte,
    Lte,

    Print,
    Debug,
    Nop,
    
}


#[derive(Debug, Clone, Copy)]
pub struct Instr {
    pub opcode: Opcode,
    pub operands: CHSValue,
}

impl Instr {
    pub fn new(kind: Opcode, operands: CHSValue) -> Self { Self { opcode: kind, operands } }
}
