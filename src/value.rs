use core::fmt;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int64(i64),
    Uint64(u64),
    List(RefCell<List>),
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
            Value::List(v) => write!(f, "{}", v.borrow().describe()),
            Value::Null => write!(f, "\0"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct List {
    pub elem: Vec<Rc<Value>>,
}

impl List {
    pub fn describe(&self) -> String {
        let values: Vec<String> = (&self.elem).into_iter().map(|e| e.to_string()).collect();
        return format!("List([{}])", values.join(", "));
    }
    pub fn get_object(&self, pos: usize) -> Result<Rc<Value>, String> {
        if pos >= self.elem.len() {
            return Err(format!("Array index out of range for position {}", pos));
        }
        return Ok(self.elem[pos].clone());
    }
    pub fn set_object(&mut self, pos: usize, obj: Rc<Value>) -> Option<String> {
        if pos >= self.elem.len() {
            //return Some(format!("Array index out of range for position {}", pos));
            self.elem.resize(pos+1, Rc::new(Value::Int64(0)))
        }

        self.elem[pos] = obj;
        return None;
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
            Value::List(_) => true,
            _ => false,
        }
    }
    pub fn get_indexed(&self, idx: &Rc<Value>) -> Result<Rc<Value>, String> {
        match (self, idx.as_ref()) {
            (Value::List(arr), Value::Int64(i)) => {
                if *i < 0 {
                    return Err(format!("Index {} must be greater than or equal to zero", i));
                }
                let result = arr.borrow().get_object(*i as usize);
                if result.is_err() {
                    return Err(result.unwrap_err());
                }

                return Ok(result.unwrap());
            }
            (Value::List(arr), Value::Uint64(i)) => {
                let result = arr.borrow().get_object(*i as usize);
                if result.is_err() {
                    return Err(result.unwrap_err());
                }

                return Ok(result.unwrap());
            }
            (Value::Str(st), Value::Int64(i)) => {
                if *i < 0 {
                    return Err(format!("Index {} must be greater than or equal to zero", i));
                }

                let ch = st.chars().nth(*i as usize);
                if ch.is_none() {
                    return Err(format!("String \"{}\" index out of bounds for {}", st, i));
                }

                return Ok(Rc::new(Value::Str(ch.unwrap().to_string())));
            }
            (Value::Str(st), Value::Uint64(i)) => {
                let ch = st.chars().nth(*i as usize);
                if ch.is_none() {
                    return Err(format!("String \"{}\" index out of bounds for {}", st, i));
                }

                return Ok(Rc::new(Value::Char(ch.unwrap())));
            }
            _ => {
                return Err(format!(
                    "Object of type {:?} does not support indexing of type {:?}", self, idx
                ));
            }
        }
    }
    pub fn set_indexed(&mut self, idx: &Rc<Value>, new_val: Rc<Value>) -> Option<String>{
        match (self, idx.as_ref()) {
            (Value::List(arr), Value::Int64(i)) => {
                arr.borrow_mut().set_object(*i as usize, new_val)
            }
            _ => None
        }
    }
    pub fn len(&self) -> Result<Rc<Value>, String> {
        match self {
            Value::List(arr) => {
                Ok(Rc::new(Value::Int64(arr.borrow().elem.len() as i64)))
            }
            Value::Str(s) => {
                Ok(Rc::new(Value::Int64(s.len() as i64)))
            }
            _ => {
                return Err(format!(
                    "Object of type {:?} does not support indexing", self
                ));
            }
        }
    }
}
