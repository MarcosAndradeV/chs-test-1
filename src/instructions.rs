use crate::value::Value;



#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Opcode {
    Halt = 0,

    Const,
    PushPtr,
    Call,
    Ret,

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
    Land,

    IdxGet,
    IdxSet,
    Len,

    Bind,
    PushBind,
    Unbind,
    
    PushLabel,
    DropLabel,
    
    Jmp,
    JmpIf,
    
    Eq,
    Neq,
    Gt,
    Lt,
    Gte,
    Lte,
    
    Println,
    Print,
    Debug,
    Nop,
    
    GlobalStore,
    InlineStore,
    GlobalLoad,
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
#[derive(Debug, Clone)]
pub struct Bytecode {
    pub program: Vec<Instr>,
    pub consts: Vec<Value>,
}

impl Bytecode {
    pub fn new(program: Vec<Instr>, consts: Vec<Value>) -> Self { Self { program, consts } }
}