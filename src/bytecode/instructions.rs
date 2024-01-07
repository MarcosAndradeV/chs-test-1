use super::value::CHSValue;


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Opcode {
    Halt = 0,
}


#[derive(Debug, Clone, Copy)]
pub struct Instr {
    pub opcode: Opcode,
    pub operand: usize,
}

impl Instr {
    pub fn new(kind: Opcode, operand: usize) -> Self { Self { opcode: kind, operand } }
}
