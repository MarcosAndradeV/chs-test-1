use super::value::CHSValue;


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Opcode {
    Halt = 0,
    Iconst,
    Oconst,
    Pop,
    Dup,
    Over,
    Swap,

    Bind,
    BindPush,
    Unbind,

    Call,
    Ret,
    PreProc,

    Iadd,
    IMinus,
    IMul,
    IDiv,
    IInc,
    IMod,

    Jmp,
    JmpIf,
    JmpWhile,
    End,
    While,

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
    pub operand: usize,
}

impl Instr {
    pub fn new(kind: Opcode, operand: usize) -> Self { Self { opcode: kind, operand } }
}
