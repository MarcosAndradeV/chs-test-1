
type Word = i64;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InstrKind {
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

impl From<u8> for InstrKind {
    fn from(value: u8) -> Self {
        match value {
            0 => InstrKind::Halt,
            1 => InstrKind::Push,
            2 => InstrKind::Pop,
            3 => InstrKind::Dup,
            4 => InstrKind::Swap,
        
            5 => InstrKind::Add,
            6 => InstrKind::Minus,
            7 => InstrKind::Mul,
            8 => InstrKind::Div,
        
            9 => InstrKind::Jmp,
            10 => InstrKind::JmpIf,
        
            11 => InstrKind::Eq,
            12 => InstrKind::Gt,
            13 => InstrKind::Lt,
            14 => InstrKind::Gte,
            15 => InstrKind::Lte,
        
            16 => InstrKind::Print,
            17 => InstrKind::Debug,
            _  => InstrKind::Nop,
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Instr {
    pub kind: InstrKind,
    pub operands: Option<Word>,
}

impl Instr {
    pub fn new(kind: InstrKind, operands: Option<Word>) -> Self { Self { kind, operands } }
}
