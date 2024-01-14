use std::rc::Rc;

use crate::exeptions::VMError;
use crate::config::{STACK_CAPACITY, MEM_CAPACITY};
use crate::value::Value;
use crate::instructions::{Instr, Opcode};


#[derive(Debug)]
pub struct CHSVM {
    pub stack: Vec<Rc<Value>>,
    pub return_stack: Vec<usize>,
    pub consts: Rc<[Value]>,
    pub memory: Vec<Rc<Value>>,
    pub is_halted: bool,
    pub ip: usize,
    sp: usize,
    program: Vec<Instr>,
}

impl CHSVM {
    pub fn new(program: Vec<Instr>, consts: Vec<Value> ) -> Self {
        let mut memory = Vec::with_capacity(MEM_CAPACITY);
        memory.resize(MEM_CAPACITY, Rc::new(Value::Null));
        Self {
            stack: Vec::with_capacity(STACK_CAPACITY),
            return_stack: Vec::with_capacity(STACK_CAPACITY),
            consts: consts.into(),
            memory,
            sp: 0,
            ip: 0,
            is_halted: false,
            program,
        }
    }
    pub fn execute_next_instr(&mut self) -> Result<(), VMError> {
        self.ip += 1;
        if self.ip > self.program.len() {
            // return Err(VMError::ProgramEndWithoutHalt);
            self.is_halted = true;
            return Ok(());
        }
        let instr = self.program[self.ip - 1];
        match instr.kind {
            Opcode::PushPtr => {
                let addrs = match instr.operands {
                    Some(v) => v,
                    None => return Err(VMError::OperandNotProvided),
                };
                self.push_stack(Value::Ptr(addrs).into())?;
                return Ok(());
            }
            Opcode::Const => {
                let addrs = match instr.operands {
                    Some(v) => v,
                    None => return Err(VMError::OperandNotProvided),
                };
                let val = match self.consts.get(addrs) {
                    Some(v) => v,
                    None => return Err(VMError::AddersOutOfBounds),
                };
                self.push_stack(Rc::new(val.clone()))?;
                return Ok(());
            }
            Opcode::Dup => {
                if self.stack.len() >= 1 {
                    let op_1 = self.pop_stack()?;
                    self.push_stack(op_1.clone())?;
                    self.push_stack(op_1)?;
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Dup2 => {
                if self.stack.len() >= 1 { // a b -> a b a b
                    let op_2 = self.pop_stack()?; // b
                    let op_1 = self.pop_stack()?; // a
                    self.push_stack(op_1.clone())?;
                    self.push_stack(op_2.clone())?;
                    self.push_stack(op_1)?;
                    self.push_stack(op_2)?;
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Swap => {
                if self.stack.len() >= 2 { // a b -> b a
                    let op_2 = self.pop_stack()?; // b
                    let op_1 = self.pop_stack()?; // a
                    self.push_stack(op_2)?; // b
                    self.push_stack(op_1)?; // a
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Over => { // a b -> a b a
                if self.stack.len() >= 2 {
                    let op_2 = self.pop_stack()?; // b
                    let op_1 = self.pop_stack()?; // a
                    self.push_stack(op_1.clone())?; // a
                    self.push_stack(op_2)?; // b
                    self.push_stack(op_1)?; // a
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Pop => {
                if self.stack.len() >= 1 {
                    let _ = self.pop_stack()?;
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Add => {
                if self.stack.len() >= 2 {
                    let op_2 = self.pop_stack()?;
                    let op_1 = self.pop_stack()?;

                    if op_1.is_ptr() || op_2.is_ptr() { return Err(VMError::TypeIncorrect); }

                    match *op_1 {
                        Value::Int64(v) => {
                            match *op_2 {
                                Value::Int64(o) => { self.push_stack(Rc::new(Value::Int64(v + o)))? }
                                Value::Uint64(o) => { self.push_stack(Rc::new(Value::Uint64(v as u64 + o)))? }
                                _ => {return Err(VMError::TypeIncorrect);}
                            }
                        }
                        Value::Uint64(v) => {
                            match *op_2 {
                                Value::Uint64(o) => { self.push_stack(Rc::new(Value::Uint64(v + o)))? }
                                Value::Int64(o) => { self.push_stack(Rc::new(Value::Uint64(v + o as u64)))? }
                                _ => {return Err(VMError::TypeIncorrect);}
                            }
                        }
                        _ => {return Err(VMError::TypeIncorrect);}
                    }

                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Minus => {
                if self.stack.len() >= 2 {
                    let op_2 = self.stack_pop_i64()?;
                    let op_1 = self.stack_pop_i64()?;
                    self.push_stack(Value::Int64(op_1 - op_2).into())?;
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Mul => {
                if self.stack.len() >= 2 {
                    let op_2 = self.stack_pop_i64()?;
                    let op_1 = self.stack_pop_i64()?;
                    self.push_stack(Value::Int64(op_1 * op_2).into())?;
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Div => {
                if self.stack.len() >= 2 {
                    let op_2 = self.stack_pop_i64()?;
                    let op_1 = self.stack_pop_i64()?;
                    if op_2 == 0 {
                        return Err(VMError::DivByZero);
                    }
                    self.push_stack(Value::Int64(op_1 / op_2).into())?;
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Mod => {
                if self.stack.len() >= 2 {
                    let op_2 = self.stack_pop_i64()?;
                    let op_1 = self.stack_pop_i64()?;
                    if op_2 == 0 {
                        return Err(VMError::DivByZero);
                    }
                    self.push_stack(Value::Int64(op_1 % op_2).into())?;
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Shr => {
                if self.stack.len() >= 2 {
                    let op_2 = self.stack_pop_u64()?;
                    let op_1 = self.stack_pop_u64()?;
                    self.push_stack(Value::Uint64(op_1 >> op_2).into())?;
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Shl => {
                if self.stack.len() >= 2 {
                    let op_2 = self.stack_pop_u64()?;
                    let op_1 = self.stack_pop_u64()?;
                    self.push_stack(Value::Uint64(op_1 << op_2).into())?;
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Lor => {
                if self.stack.len() >= 2 {
                    let op_2 = self.stack_pop_bool()?;
                    let op_1 = self.stack_pop_bool()?;
                    self.push_stack(Value::Bool(op_1 || op_2).into())?;
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Bitor => {
                if self.stack.len() >= 2 {
                    let op_2 = self.stack_pop_u64()?;
                    let op_1 = self.stack_pop_u64()?;
                    self.push_stack(Value::Uint64(op_1 | op_2).into())?;
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Bitand => {
                if self.stack.len() >= 2 {
                    let op_2 = self.stack_pop_u64()?;
                    let op_1 = self.stack_pop_u64()?;
                    self.push_stack(Value::Uint64(op_1 & op_2).into())?;
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::PushLabel => {
                self.return_stack.push(self.ip);
                return Ok(());
                //return Err(VMError::NotImplemeted);
            }
            Opcode::GetLabel => {
                let addrs = match instr.operands {
                    Some(v) => v,
                    None => return Err(VMError::OperandNotProvided),
                };
                if addrs > self.program.len() {
                    return Err(VMError::AddersOutOfBounds);
                }
                let label = match self.return_stack.get(addrs) {
                    Some(v) => *v,
                    None => return Err(VMError::StackUnderflow),
                };
                self.push_stack(Value::Ptr(label).into())?;
                return Ok(());
                //return Err(VMError::NotImplemeted);
            }
            Opcode::DropLabel => {
                self.return_stack.pop();
                return Ok(());
                //return Err(VMError::NotImplemeted);
            }
            Opcode::Jmp => {
                let addrs = match instr.operands {
                    Some(v) => v,
                    None => return Err(VMError::OperandNotProvided),
                };
                if addrs > self.program.len() {
                    return Err(VMError::AddersOutOfBounds);
                }
                self.ip = addrs;
                Ok(())
            }
            Opcode::Jmpr => {
                let addrs = self.stack_pop_ptr()?;
                if addrs > self.stack.capacity() { return Err(VMError::AddersOutOfBounds); }
                self.ip = addrs;
                Ok(())
            }
            Opcode::JmpIf => {
                let op_1 = self.stack_pop_bool()?;
                if op_1 {
                    return Ok(());
                }
                let addrs = match instr.operands {
                    Some(v) => v,
                    None => return Err(VMError::OperandNotProvided),
                };
                if addrs > self.program.len() {
                    return Err(VMError::AddersOutOfBounds);
                }

                self.ip = addrs;

                Ok(())
            }
            Opcode::JmpIfr => {
                let addrs = self.stack_pop_ptr()?;
                let op_1 = self.stack_pop_bool()?;
                if op_1 {
                    return Ok(());
                }
                self.ip = addrs;
                Ok(())
            }
            Opcode::Eq => {
                if self.stack.len() >= 2 {
                    let op_2 = self.stack_pop_i64()?;
                    let op_1 = self.stack_pop_i64()?;
                    self.push_stack(Value::Bool(op_1 == op_2).into())?;
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Neq => {
                if self.stack.len() >= 2 {
                    let op_2 = self.stack_pop_i64()?;
                    let op_1 = self.stack_pop_i64()?;
                    self.push_stack(Value::Bool(op_1 != op_2).into())?;
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Gt => {
                if self.stack.len() >= 2 { // a b > -> a > b 
                    let op_2 = self.stack_pop_i64()?; // b
                    let op_1 = self.stack_pop_i64()?; // a
                    self.push_stack(Value::Bool(op_1 > op_2).into())?;
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Gte => {
                if self.stack.len() >= 2 {
                    let op_2 = self.stack_pop_i64()?;
                    let op_1 = self.stack_pop_i64()?;
                    self.push_stack(Value::Bool(op_1 >= op_2).into())?;
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Lt => {
                if self.stack.len() >= 2 {
                    let op_2 = self.stack_pop_i64()?;
                    let op_1 = self.stack_pop_i64()?;
                    self.push_stack(Value::Bool(op_1 < op_2).into())?;
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Lte => {
                if self.stack.len() >= 2 {
                    let op_2 = self.stack_pop_i64()?;
                    let op_1 = self.stack_pop_i64()?;
                    self.push_stack(Value::Bool(op_1 <= op_2).into())?;
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }

            Opcode::Print => {
                if self.stack.len() >= 1 {
                    let value = self.pop_stack()?;
                    println!("{}", value);
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Pstr => {
                if self.stack.len() >= 1 {
                    let val = self.stack_pop_str()?;
                    print!("{}", val);
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Debug => {
                println!(
                    "CHSVM: {:?}, SP: {}, STACK_LEN: {}",
                    self.stack,
                    self.sp,
                    self.stack.len()
                );
                return Ok(());
            }
            Opcode::Nop => {
                return Ok(());
            }
            Opcode::Write => { // TODO: Need to change this
                return Err(VMError::StackUnderflow);
            }
            Opcode::Load => {
                let addrs = self.stack_pop_ptr()?;
                let val = match self.memory.get(addrs) {
                    Some(v) => v,
                    None => return Err(VMError::AddersOutOfBounds)
                };
                self.push_stack(Rc::clone(val))?;
                Ok(())
            }
            Opcode::Store => {
                let value = self.pop_stack()?;
                let addrs = self.stack_pop_ptr()?;
                if addrs > self.memory.len() { return Err(VMError::AddersOutOfBounds); }
                self.memory[addrs] = value;
                Ok(())
            }
            Opcode::IdxGet => {
                let idx = self.pop_stack()?;
                let list = self.pop_stack()?;
                let val = match list.get_indexed(&idx) {
                    Ok(v) => v,
                    Err(e) => {
                        println!("{}", e);
                        return Err(VMError::ProgramEndWithoutHalt);
                    }
                };
                self.push_stack(val)?;
                Ok(())
            },
            Opcode::IdxSet => {
                let new_val = self.pop_stack()?;
                let idx = self.pop_stack()?;
                let addrs = self.stack_pop_ptr()?;
                let mut val = match self.memory.get(addrs) {
                    Some(v) => v.as_ref().clone(),
                    None => return Err(VMError::AddersOutOfBounds)
                };
                val.set_indexed(&idx, new_val);
                self.push_stack(Rc::new(Value::Ptr(addrs)))?;
                self.push_stack(Rc::new(val))?;
                Ok(())
            },
            Opcode::Halt => {
                self.is_halted = true;
                return Ok(());
            } //_ => Ok(())
        }
    }

    pub fn run(&mut self) {
        for (i, e) in self.program.iter().enumerate() {
            println!("{} -> {:?}", i, e);
        }
        while !self.is_halted {
            match self.execute_next_instr() {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("It's a trap: {} at {} {:?}", e, self.ip-1, self.program[self.ip-1]);
                    break;
                }
            }
        }
    }

    fn pop_stack(&mut self) -> Result<Rc<Value>, VMError> {
        if !(self.sp == 0) {
            self.sp -= 1
        }
        match self.stack.pop() {
            Some(v) => Ok(v),
            None => Err(VMError::StackUnderflow),
        }
    }

    fn stack_pop_i64(&mut self) -> Result<i64, VMError> {
        if !(self.sp == 0) {
            self.sp -= 1
        }
        match self.stack.pop() {
            Some(v) => {
                match *v {
                    Value::Int64(v) => Ok(v),
                    _ => Err(VMError::OperandNotProvided)
                }
            }
            None => Err(VMError::StackUnderflow),
        }
    }

    fn stack_pop_u64(&mut self) -> Result<u64, VMError> {
        if !(self.sp == 0) {
            self.sp -= 1
        }
        match self.stack.pop() {
            Some(v) => {
                match *v {
                    Value::Uint64(v) => Ok(v),
                    Value::Int64(v) => Ok(v as u64),
                    _ => Err(VMError::OperandNotProvided)
                }
            }
            None => Err(VMError::StackUnderflow),
        }
    }

    fn stack_pop_ptr(&mut self) -> Result<usize, VMError> {
        if !(self.sp == 0) {
            self.sp -= 1
        }
        match self.stack.pop() {
            Some(v) => {
                match *v {
                    Value::Ptr(v) => Ok(v),
                    _ => Err(VMError::OperandNotProvided)
                }
            }
            None => Err(VMError::StackUnderflow),
        }
    }

    fn stack_pop_str(&mut self) -> Result<String, VMError> {
        if !(self.sp == 0) {
            self.sp -= 1
        }
        match self.stack.pop() {
            Some(v) => Ok(v.to_string()),
            None => Err(VMError::StackUnderflow),
        }
    }

    fn stack_pop_bool(&mut self) -> Result<bool, VMError> {
        if !(self.sp == 0) {
            self.sp -= 1
        }
        match self.stack.pop() {
            Some(v) => {
                match *v {
                    Value::Bool(v) => Ok(v),
                    _ => Err(VMError::OperandNotProvided)
                }
            },
            None => Err(VMError::StackUnderflow),
        }
    }

    fn push_stack(&mut self, value: Rc<Value>) -> Result<(), VMError> {
        if (self.sp + 1) > self.stack.capacity() {
            return Err(VMError::StackOverflow);
        }
        self.sp += 1;
        self.stack.push(value);
        Ok(())
    }
}
