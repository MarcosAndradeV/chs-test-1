

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Opcode {
    Halt = 0,

    Const,
    PushPtr,

    Pop,
    Dup,
    Dup2,
    Swap,
    Over,
    
    Add,
    Minus,
    Mul,
    Div,
    Mod,
    Shr,
    Shl,
    Bitor,
    Bitand,
    Lor,
    
    GetLabel,
    PushLabel,
    DropLabel,
    
    Jmp,
    Jmpr,
    JmpIf,
    JmpIfr,
    
    Eq,
    Neq,
    Gt,
    Lt,
    Gte,
    Lte,
    
    Print,
    Pstr,
    Debug,
    Nop,
    
    Store,
    Load,
    Write,
}

#[derive(Debug, Clone, Copy)]
pub struct Instr {
    pub kind: Opcode,
    pub operands: Option<usize>,
}

impl Instr {
    pub fn new(kind: Opcode, operands: Option<usize>) -> Self {
        Self { kind, operands }
    }
}