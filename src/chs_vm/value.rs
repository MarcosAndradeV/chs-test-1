use core::fmt;
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Int64(i64),
    Array(Vec<Self>),
    Bool(bool),
    Str(Vec<char>),
    Char(char),
    Fn(usize, Vec<Self>),
    Nil,
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
            Value::Str(v) => {
                let mut buff = String::new();
                for a in v.iter() { buff.push(*a) }
                write!(f, "{}", buff) 
            },
            Value::Array(v) => {
                let mut buff = String::from("[");
                for a in v.iter() { buff.push_str(&format!("{a} ")) }
                if buff.ends_with(" ") { buff.pop(); }
                buff.push_str(&format!("]"));
                write!(f, "{}", buff.to_string())
            },
            Value::Char(v) => write!(f, "{}", v),
            Value::Fn(v, _) => write!(f, "Fn({})", v),
            Value::Nil => write!(f, "nil"),
        }
    }
}


impl Value {
    pub fn is_int(&self) -> bool {
        match self {
            Value::Int64(_) => true,
            _ => false,
        }
    }
    pub fn is_str(&self) -> bool {
        match self {
            Value::Str(_) => true,
            _ => false,
        }
    }
    pub fn is_char(&self) -> bool {
        match self {
            Value::Char(_) => true,
            _ => false,
        }
    }
    pub fn is_list(&self) -> bool {
        match self {
            Value::Array(_) => true,
            _ => false,
        }
    }
    pub fn is_fn(&self) -> bool {
        match self {
            Value::Fn(_, _) => true,
            _ => false,
        }
    }
    pub fn to_char(self) -> Option<char> {
        None
    }
    pub fn get_indexed(self, idx: Value) -> Value {
        match (self, idx) {
            (Value::Array(arr), Value::Int64(i)) => {
                if i < 0 || i as usize >= arr.len() {
                    return Value::Nil;
                }
                return arr[i as usize].clone();
            }
            (Value::Str(st), Value::Int64(i)) => {
                if i < 0 || i as usize >= st.len() {
                    return Value::Nil;
                }
                Value::Char(st[i as usize])
            }
            (_, _) => Value::Nil
        }
    }

    pub fn tail(self) -> Value {
        match self {
            Value::Array(arr) => {
                if let Some((_, tail)) = arr.split_first() {
                    Value::Array(tail.to_owned())
                } else {
                    Value::Nil
                }
            }
            Value::Str(st) => {
                if let Some((_, tail)) = st.split_first() {
                    Value::Str(tail.to_owned())
                } else {
                    Value::Nil
                }
            }
            _ => Value::Nil
        }
    }

    pub fn head(self) -> Value {
        match self {
            Value::Array(arr) => {
                if let Some((head, _)) = arr.split_first() {
                    head.to_owned()
                } else {
                    Value::Nil
                }
            }
            Value::Str(st) => {
                if let Some((head, _)) = st.split_first() {
                    Value::Char(head.to_owned())
                } else {
                    Value::Nil
                }
            }
            _ => Value::Nil
        }
    }

    pub fn concat(self, other: Value) -> Value {
        match (self, other) {
            (Value::Array(mut arr), Value::Array(o)) => {
                arr.extend(o);
                Value::Array(arr)
            }
            (Value::Str(mut st), Value::Str(o)) => {
                st.extend(o);
                Value::Str(st)
            }
            (_, _) => Value::Nil
        }
    }

    pub fn set_indexed(self, idx: Value, new_val: Value) -> Value {
        match (self, idx) {
            (Value::Array(mut arr), Value::Int64(i)) => {
                if i < 0 || i as usize >= arr.len() {
                    return Value::Nil;
                }
                arr[i as usize] = new_val;
                Value::Array(arr)
            }
            (Value::Str(mut st), Value::Int64(i)) => {
                if i < 0 || i as usize >= st.len() {
                    return Value::Nil;
                }
                if let Some(v) = new_val.to_char() {
                    st[i as usize] = v;
                }
                Value::Str(st)
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
