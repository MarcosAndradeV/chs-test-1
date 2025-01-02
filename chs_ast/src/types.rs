use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Primitive {
    Void,
    Int64,
    Bool,
    Char,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CHSType {
    Primitive(Primitive),
    Pointer(Box<CHSType>),
    Func(Vec<CHSType>, Box<CHSType>),
}

impl CHSType {
    pub fn void() -> Self {
        Self::Primitive(Primitive::Void)
    }
    pub fn int() -> Self {
        Self::Primitive(Primitive::Int64)
    }
    pub fn bool() -> Self {
        Self::Primitive(Primitive::Bool)
    }
    pub fn char() -> Self {
        Self::Primitive(Primitive::Char)
    }
}

impl fmt::Display for CHSType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CHSType::Primitive(n) => write!(f, "{n:?}"),
            CHSType::Pointer(a) => write!(f, "*{a}"),
            _ => todo!(),
        }
    }
}
