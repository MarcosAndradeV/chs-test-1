use super::value::CHSValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub enum Opcode {
    Halt = 0,
    Pushi,
    Pushf,
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

    Add,
    Minus,
    Mul,
    Div,
    Inc,
    Mod,
    Lgor,

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
#[derive(Serialize, Deserialize)]
pub struct Instr {
    pub opcode: Opcode,
    pub operands: CHSValue,
}

impl Instr {
    pub fn new(kind: Opcode, operands: CHSValue) -> Self { Self { opcode: kind, operands } }
}
