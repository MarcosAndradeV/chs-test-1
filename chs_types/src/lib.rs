use core::fmt;
use std::collections::BTreeMap;


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Primitive {
    Void,
    Int,
    Bool,
    Char,
}

impl fmt::Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Primitive::Void => write!(f, "void"),
            Primitive::Int => write!(f, "int"),
            Primitive::Bool => write!(f, "bool"),
            Primitive::Char => write!(f, "char"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CHSType {
    Custom(String),
    Primitive(Primitive),
    Pointer(Box<CHSType>),
    Func(Vec<CHSType>, Box<CHSType>),
    Record(BTreeMap<String, CHSType>),
}

impl CHSType {
    pub fn custom(s: String) -> Self {
        Self::Custom(s)
    }
    pub fn void() -> Self {
        Self::Primitive(Primitive::Void)
    }
    pub fn int() -> Self {
        Self::Primitive(Primitive::Int)
    }
    pub fn bool() -> Self {
        Self::Primitive(Primitive::Bool)
    }
    pub fn char() -> Self {
        Self::Primitive(Primitive::Char)
    }
    pub fn ptr(t: CHSType) -> Self {
        Self::Pointer(t.into())
    }

    #[must_use]
    pub fn is_void(&self) -> bool {
        matches!(self, Self::Primitive(Primitive::Void))
    }
}

impl fmt::Display for CHSType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CHSType::Custom(n) => write!(f, "{n}"),
            CHSType::Primitive(n) => write!(f, "{n}"),
            CHSType::Pointer(a) => write!(f, "*{a}"),
            CHSType::Func(args, ret) => {
                write!(f, "fn(")?;
                for (i, item) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, ") -> {}", ret)
            }
            CHSType::Record(args) => {
                write!(f, "record (")?;
                for (i, item) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, "\n")?;
                    }
                    write!(f, "{}: {}", item.0, item.1)?;
                }
                write!(f, "\n)",)
            }
        }
    }
}
