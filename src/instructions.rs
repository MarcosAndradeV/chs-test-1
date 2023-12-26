use crate::value::CHSValue;


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Opcode {
    Halt = 0,
    Push,
    Pop,
    Dup,
    Swap,

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

impl From<i64> for Opcode {
    fn from(value: i64) -> Self {
        match value {
            0 => Opcode::Halt,
            1 => Opcode::Push,
            2 => Opcode::Pop,
            3 => Opcode::Dup,
            4 => Opcode::Swap,
        
            5 => Opcode::Add,
            6 => Opcode::Minus,
            7 => Opcode::Mul,
            8 => Opcode::Div,
        
            9 => Opcode::Jmp,
            10 => Opcode::JmpIf,
        
            11 => Opcode::Eq,
            12 => Opcode::Gt,
            13 => Opcode::Lt,
            14 => Opcode::Gte,
            15 => Opcode::Lte,
        
            16 => Opcode::Print,
            17 => Opcode::Debug,
            _  => Opcode::Nop,
        }
    }
}

impl From<Opcode> for u8 {
    fn from(value: Opcode) -> Self {
        match value {
            Opcode::Halt  => 0,
            Opcode::Push  => 1,
            Opcode::Pop   => 2,
            Opcode::Dup   => 3,
            Opcode::Swap  => 4,
            Opcode::Add   => 5,
            Opcode::Minus => 6,
            Opcode::Mul   => 7,
            Opcode::Div   => 8,
            Opcode::Jmp   => 9,
            Opcode::JmpIf => 10,
            Opcode::Eq    => 11,
            Opcode::Gt    => 12,
            Opcode::Lt    => 13,
            Opcode::Gte   => 14,
            Opcode::Lte   => 15,
            Opcode::Print => 16,
            Opcode::Debug => 17,
            _             => 18,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Instr {
    pub opcode: Opcode,
    pub operands: CHSValue,
}

impl Instr {
    pub fn new(kind: Opcode, operands: CHSValue) -> Self { Self { opcode: kind, operands } }
}
