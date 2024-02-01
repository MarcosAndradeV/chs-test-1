use core::fmt;

use crate::{compiler::ir::Operation, value::Value};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Opcode {
    Halt = 0,

    Const,

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

    Bind,
    PushBind,
    SetBind,
    Unbind,

    Jmp,
    JmpIf,

    Eq,
    Neq,
    Gt,
    Lt,
    Gte,
    Lte,

    Nop,

    GlobalStore,
    GlobalLoad,

    Buildin,
    
    SkipFn,
    CallFn,
    RetFn,
}

impl From<&Operation> for Opcode {
    fn from(item: &Operation) -> Self {
        match item {
            Operation::Pop => Self::Pop,
            Operation::Dup => Self::Dup,
            Operation::Dup2 => Self::Dup2,
            Operation::Swap => Self::Swap,
            Operation::Over => Self::Over,
            Operation::Add => Self::Add,
            Operation::Minus => Self::Minus,
            Operation::Mul => Self::Mul,
            Operation::Div => Self::Div,
            Operation::Mod => Self::Mod,
            Operation::Eq => Self::Eq,
            Operation::Neq => Self::Neq,
            Operation::Gt => Self::Gt,
            Operation::Gte => Self::Gte,
            Operation::Lte => Self::Lte,
            Operation::Lt => Self::Lt,
            Operation::Land => Self::Land,
            Operation::Lor => Self::Lor,
            Operation::Shl => Self::Shl,
            Operation::Shr => Self::Shr,
            Operation::Bitand => Self::Bitand,
            Operation::Bitor => Self::Bitor,
        }
    }
}

#[derive(PartialEq, Clone, Debug, Eq, Copy, PartialOrd)]
pub enum Builtin {
    IdxGet = 0,
    IdxSet,
    Len,
    Println,
    Print,
    Debug,
    Fill,
    Builtins,
    TimeUnix,
    Args,
    Exit,
    TypeOf,
    CallFunc,
    FStat,
    FWrite,
    FAppend,
    FRead,
    ReadLine,
    SWrite,
    SRead,
    GetSyscalls,
    Syscall,
    Range,
    Invalid,
}

impl Builtin {
    pub fn is_invalid(&self) -> bool {
        *self == Builtin::Invalid
    }
}

impl From<usize> for Builtin {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::IdxGet,
            1 => Self::IdxSet,
            2 => Self::Len,
            3 => Self::Println,
            4 => Self::Print,
            5 => Self::Debug,
            6 => Self::Fill,
            7 => Self::Builtins,
            8 => Self::TimeUnix,
            9 => Self::Args,
            10 => Self::Exit,
            11 => Self::TypeOf,
            12 => Self::CallFunc,
            13 => Self::FStat,
            14 => Self::FWrite,
            15 => Self::FAppend,
            16 => Self::FRead,
            17 => Self::ReadLine,
            18 => Self::SWrite,
            19 => Self::SRead,
            20 => Self::GetSyscalls,
            21 => Self::Syscall,
            22 => Self::Range,
            _ => Self::Invalid,
        }
    }
}

#[derive(Debug, Clone, Copy)]
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
