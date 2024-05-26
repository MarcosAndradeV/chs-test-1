
use super::value::Value;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Instr {
    Halt,

    Const(Value),

    Pop,
    Dup,
    Swap,
    Over,
    Rot,

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
    Lnot,

    Bind(usize),
    PushBind(usize),
    SetBind(usize),
    Unbind(usize),

    Jmp(isize),
    JmpIf(isize),

    Eq,
    Neq,
    Gt,
    Lt,
    Gte,
    Lte,

    Nop,

    GlobalStore(usize),
    GlobalLoad(usize),
    
    CallFn(usize),
    RetFn,

    Debug,
    Exit,
    Print,
    IdxSet,
    IdxGet,
    Len,
    Concat,
    Head,
    Tail,
    Call,
    MakeList(usize),
}


/*
#[derive(Debug, Clone)]
pub struct Instr {
    pub kind: Opcode,
    pub operands: Option<usize>,
}

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(v) = self.operands {
            write!(f, "{:?}({})", self.kind, v)
        } else {
            write!(f, "{:?}", self.kind)
        }
    }
}

impl Instr {
    pub fn new(kind: Opcode, operands: Option<usize>) -> Self {
        Self { kind, operands }
    }
}
*/
#[derive(Debug, Clone)]
pub struct Bytecode {
    pub program: Vec<Instr>,
    pub consts: Vec<Value>,
}

impl Bytecode {
    pub fn new(program: Vec<Instr>, consts: Vec<Value>) -> Self {
        Self { program, consts }
    }
    pub fn len(&self) -> usize {
        self.program.len()
    }
}
