use core::fmt;

pub const STACK_CAPACITY: usize = 10240;
pub const MEM_CAPACITY: usize = 10240;

pub type Word = i64;

#[derive(Debug, Clone)]
pub enum Value { 
    Int64(i64),
    Uint64(u64),
    Ptr(usize),
    Bool(bool),
    Char(char),
    Str(String),
    Null,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int64(v) => write!(f, "{}", v),
            Value::Uint64(v) => write!(f, "{}", v),
            Value::Ptr(v) => write!(f, "{}", v),
            Value::Bool(v) => write!(f, "{}", v),
            Value::Str(v) => write!(f, "{}", v),
            Value::Char(v) => write!(f, "{}", v),
            Value::Null => write!(f, "\0"),
        }
    }
}

impl Value {
    pub fn as_char(self) -> char {
        match self {
            Value::Int64(v) => v as u8 as char,
            Value::Char(v) => v,
            _ => '\0'
        }
    }
    pub fn is_ptr(&self) -> bool {
        match self {
            Value::Ptr(_) => true,
            _ => false,
        }
    }
}