use core::fmt;
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Int64(i64),
    Array(Vec<Self>),
    Bool(bool),
    Char(char),
    Ptr(usize),
    Nil,
}

impl Default for Value {
    fn default() -> Self {
        Self::Nil
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Int64(v), Value::Int64(o)) => {
                Some(v.cmp(o))
            },
            _ => None
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int64(v) => write!(f, "{}", v),
            Value::Bool(v) => write!(f, "{}", v),
            //Value::Str(v) => {
            //    let mut buff = String::new();
            //    for a in v.iter() { buff.push(*a) }
            //    write!(f, "{}", buff) 
            //},
            Value::Array(v) => {
                let mut buff = String::from("[");
                for a in v.iter() { buff.push_str(&format!("{a} ")) }
                if buff.ends_with(" ") { buff.pop(); }
                buff.push_str(&format!("]"));
                write!(f, "{}", buff.to_string())
            },
            Value::Char(v) => write!(f, "{}", v),
            Value::Ptr(v) => write!(f, "Ptr({})", v),
            Value::Nil => write!(f, "nil"),
        }
    }
}


impl Value {
    pub fn int(self) -> i64 {
        if let Self::Int64(i) = self {
            i
        } else {
            panic!("Expect value int")
        }
    }
    pub fn bool(self) -> bool {
        if let Self::Bool(i) = self {
            i
        } else {
            panic!("Expect value bool")
        }
    }
    pub fn array(self) -> Vec<Self> {
        if let Self::Array(i) = self {
            i
        } else {
            panic!("Expect value array")
        }
    }
    pub fn ptr(self) -> usize {
        if let Self::Ptr(i) = self {
            i
        } else {
            panic!("Expect value ptr")
        }
    }
    pub fn char(self) -> char {
        if let Self::Char(i) = self {
            i
        } else {
            panic!("Expect value char")
        }
    }
}

// Read stdin input:
pub fn stdin_read() -> Result<Vec<u8>, String> {
    let mut buffer = vec![];
    let result = io::stdin().read(&mut buffer);
    if result.is_err() {
        return Err(format!("IO Error"));
    }

    return Ok(buffer);
}

// Read a line as string:
pub fn read_line() -> Result<String, String> {
    let mut string_buffer = String::new();
    let result = io::stdin().read_line(&mut string_buffer);
    if result.is_err() {
        return Err(format!("IO Error"));
    }

    string_buffer = string_buffer.replace("\n", "");
    return Ok(string_buffer);
}

// Write stdout output:
pub fn stdout_write(data: &Vec<u8>) -> Result<usize, String> {
    let stdout = io::stdout();
    let mut lock = stdout.lock();
    let mut result = lock.write_all(&data);

    if result.is_err() {
        return Err(format!("IO Error"));
    }

    result = lock.flush();
    if result.is_err() {
        return Err(format!("IO Error"));
    }

    return Ok(data.len());
}
