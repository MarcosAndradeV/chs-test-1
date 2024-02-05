use core::fmt;
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int64(i64),
    Uint64(u64),
    Array(Vec<Value>),
    Ptr(usize),
    Bool(bool),
    Char(char),
    Str(String),
    Nil,
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            Value::Int64(v) => {
                match other {
                    Value::Int64(o) => {Some(v.cmp(o))}
                    Value::Uint64(o) => {Some(v.cmp(&(*o as i64)))}
                    _ => None
                }
            },
            Value::Uint64(v) => {
                match other {
                    Value::Int64(o) => {Some(v.cmp(&(*o as u64)))}
                    Value::Uint64(o) => {Some(v.cmp(o))}
                    _ => None
                }
            },
            _ => None
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int64(v) => write!(f, "{}", v),
            Value::Uint64(v) => write!(f, "{}", v),
            Value::Ptr(_) => write!(f, "Ptr"),
            Value::Bool(v) => write!(f, "{}", v),
            Value::Str(v) => write!(f, "{}", v),
            Value::Char(v) => write!(f, "{}", v),
            Value::Array(v) => {
                let mut buff = String::from("[");
                for a in v.iter() { buff.push_str(&format!("{a} ")) }
                buff.pop();
                buff.push_str(&format!("]"));
                write!(f, "{}", buff.to_string())
            },
            Value::Nil => write!(f, "nil"),
        }
    }
}


impl Value {
    pub fn as_char(self) -> char {
        match self {
            Value::Int64(v) => v as u8 as char,
            Value::Char(v) => v,
            _ => '\0',
        }
    }
    pub fn is_ptr(&self) -> bool {
        match self {
            Value::Ptr(_) => true,
            _ => false,
        }
    }
    pub fn is_int(&self) -> bool {
        match self {
            Value::Int64(_) => true,
            _ => false,
        }
    }
    pub fn is_uint(&self) -> bool {
        match self {
            Value::Uint64(_) => true,
            _ => false,
        }
    }
    pub fn is_list(&self) -> bool {
        match self {
            Value::Array(_) => true,
            _ => false,
        }
    }
    pub fn get_indexed(self, idx: Value) -> Value {
        match (self, idx) {
            (Value::Array(arr), Value::Int64(i)) => {
                if i < 0 || i as usize >= arr.len() {
                    println!("Index {} is out of bounds", i);
                    return Value::Nil;
                }
                return arr[i as usize].clone();
            }
            (Value::Str(st), Value::Int64(i)) => {
                let b = st.into_bytes();
                if i < 0 || i as usize >= b.len() {
                    println!("Index {} is out of bounds", i);
                    return Value::Nil;
                }
                Value::Char(b[i as usize] as char)
            }
            (Value::Str(st), Value::Uint64(i)) => {
                if i as usize >= st.len() {
                    println!("Index {} is out of bounds", i);
                    return Value::Nil;
                }
                let b = st.into_bytes();
                Value::Char(b[i as usize] as char)
            }
            (v, i) => {
                println!("Cannot index {} from {}", i, v);
                Value::Nil
            }
        }
    }

    pub fn set_indexed(self, idx: Value, new_val: Value) -> Value {
        match (self, idx) {
            (Value::Array(mut arr), Value::Int64(i)) => {
                if i < 0 || i as usize > arr.len() {
                    println!("Index {} is out of bounds", i);
                    return Value::Nil;
                }
                arr[i as usize] = new_val;
                Value::Array(arr)
            }
            _ => Value::Nil
        }
    }
    pub fn len(&self) -> Value {
        match self {
            Value::Array(arr) => {
                Value::Int64(arr.len() as i64)
            }
            Value::Str(s) => {
                Value::Int64(s.len() as i64)
            }
            _ => Value::Nil
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
