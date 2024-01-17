use std::rc::Rc;

use crate::exeptions::VMError;
use crate::config::{STACK_CAPACITY, MEM_CAPACITY};
use crate::value::Value;
use crate::instructions::{Instr, Opcode, Bytecode};
use crate::vm_error;


#[derive(Debug)]
pub struct CHSVM {
    pub stack: Vec<Rc<Value>>,
    pub return_stack: Vec<Rc<Value>>,
    pub consts: Rc<[Value]>,
    pub memory: Vec<Rc<Value>>,
    pub is_halted: bool,
    pub ip: usize,
    sp: usize,
    program: Vec<Instr>,
}

impl CHSVM {
    pub fn new(program: Bytecode) -> Self {
        let mut memory = Vec::with_capacity(MEM_CAPACITY);
        memory.resize(MEM_CAPACITY, Rc::new(Value::Null));
        Self {
            stack: Vec::with_capacity(STACK_CAPACITY),
            return_stack: Vec::with_capacity(STACK_CAPACITY),
            consts: program.consts.into(),
            memory,
            sp: 0,
            ip: 0,
            is_halted: false,
            program: program.program,
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
                    None => vm_error!("OPERAND_NOT_PROVIDED"),
                };
                self.push_stack(Value::Ptr(addrs).into())?;
                return Ok(());
            }
            Opcode::Const => {
                let addrs = match instr.operands {
                    Some(v) => v,
                    None => vm_error!("OPERAND_NOT_PROVIDED"),
                };
                let val = match self.consts.get(addrs) {
                    Some(v) => v,
                    None => vm_error!(""),
                };
                self.push_stack(Rc::new(val.clone()))?;
                return Ok(());
            }
            Opcode::Dup => {

                let op_1 = self.pop_stack()?;
                self.push_stack(op_1.clone())?;
                self.push_stack(op_1)?;
                return Ok(());

            }
            Opcode::Dup2 => { // a b -> a b a b
               
                let op_2 = self.pop_stack()?; // b
                let op_1 = self.pop_stack()?; // a
                self.push_stack(op_1.clone())?;
                self.push_stack(op_2.clone())?;
                self.push_stack(op_1)?;
                self.push_stack(op_2)?;
                return Ok(());
            }
            Opcode::Swap => { // a b -> b a
               
                let op_2 = self.pop_stack()?; // b
                let op_1 = self.pop_stack()?; // a
                self.push_stack(op_2)?; // b
                self.push_stack(op_1)?; // a
                return Ok(());
            }
            Opcode::Over => { // a b -> a b a
                let op_2 = self.pop_stack()?; // b
                let op_1 = self.pop_stack()?; // a
                self.push_stack(op_1.clone())?; // a
                self.push_stack(op_2)?; // b
                self.push_stack(op_1)?; // a
                return Ok(());

            }
            Opcode::Pop => { // a -> _
                let _ = self.pop_stack()?;
                return Ok(());

            }
            Opcode::Add => {
                let op_2 = self.pop_stack()?;
                let op_1 = self.pop_stack()?;

                if op_1.is_ptr() || op_2.is_ptr() { vm_error!("") }

                match *op_1 {
                    Value::Int64(v) => {
                        match *op_2 {
                            Value::Int64(o) => { self.push_stack(Rc::new(Value::Int64(v + o)))? }
                            Value::Uint64(o) => { self.push_stack(Rc::new(Value::Uint64(v as u64 + o)))? }
                            _ => vm_error!("")
                        }
                    }
                    Value::Uint64(v) => {
                        match *op_2 {
                            Value::Uint64(o) => { self.push_stack(Rc::new(Value::Uint64(v + o)))? }
                            Value::Int64(o) => { self.push_stack(Rc::new(Value::Uint64(v + o as u64)))? }
                            _ => vm_error!("")
                        }
                    }
                    _ => vm_error!("")
                }

                return Ok(());

            }
            Opcode::Minus => {
                let op_2 = self.stack_pop_i64()?;
                let op_1 = self.stack_pop_i64()?;
                self.push_stack(Value::Int64(op_1 - op_2).into())?;
                return Ok(());
            }
            Opcode::Mul => {
                let op_2 = self.stack_pop_i64()?;
                let op_1 = self.stack_pop_i64()?;
                self.push_stack(Value::Int64(op_1 * op_2).into())?;
                return Ok(());
            }
            Opcode::Div => {
                let op_2 = self.stack_pop_i64()?;
                let op_1 = self.stack_pop_i64()?;
                if op_2 == 0 {
                    vm_error!("");
                }
                self.push_stack(Value::Int64(op_1 / op_2).into())?;
                return Ok(());
            }
            Opcode::Mod => {
                let op_2 = self.stack_pop_i64()?;
                let op_1 = self.stack_pop_i64()?;
                if op_2 == 0 {
                    vm_error!("");
                }
                self.push_stack(Value::Int64(op_1 % op_2).into())?;
                return Ok(());
            }
            Opcode::Shr => {
                let op_2 = self.stack_pop_u64()?;
                let op_1 = self.stack_pop_u64()?;
                self.push_stack(Value::Uint64(op_1 >> op_2).into())?;
                return Ok(());
            }
            Opcode::Shl => {
                let op_2 = self.stack_pop_u64()?;
                let op_1 = self.stack_pop_u64()?;
                self.push_stack(Value::Uint64(op_1 << op_2).into())?;
                return Ok(());
            }
            Opcode::Lor => {
                let op_2 = self.stack_pop_bool()?;
                let op_1 = self.stack_pop_bool()?;
                self.push_stack(Value::Bool(op_1 || op_2).into())?;
                return Ok(());
            }
            Opcode::Land => {
                let op_2 = self.stack_pop_bool()?;
                let op_1 = self.stack_pop_bool()?;
                self.push_stack(Value::Bool(op_1 && op_2).into())?;
                return Ok(());
            }
            Opcode::Bitor => {
                let op_2 = self.stack_pop_u64()?;
                let op_1 = self.stack_pop_u64()?;
                self.push_stack(Value::Uint64(op_1 | op_2).into())?;
                return Ok(());
            }
            Opcode::Bitand => {
                let op_2 = self.stack_pop_u64()?;
                let op_1 = self.stack_pop_u64()?;
                self.push_stack(Value::Uint64(op_1 & op_2).into())?;
                return Ok(());
            }
            Opcode::PushLabel => {
                self.return_stack.push(Rc::new(Value::Ptr(self.ip)));
                return Ok(());
            }
            Opcode::GetLabel => {
                let label = match self.return_stack.pop() {
                    Some(v) => {
                        match v.as_ref() {
                            Value::Ptr(v) => *v + 1,
                            _ => vm_error!("")
                        }
                    },
                    None => vm_error!(""),
                };
                self.push_stack(Value::Ptr(label).into())?;
                return Ok(());
            }
            Opcode::DropLabel => {
                self.return_stack.pop();
                return Ok(());
            }
            Opcode::Jmp => {
                let addrs = match instr.operands {
                    Some(v) => v,
                    None => vm_error!("")
                };
                if addrs > self.program.len() {
                    vm_error!("")
                }
                self.ip = addrs;
                Ok(())
            }
            Opcode::Jmpr => {
                let addrs = self.stack_pop_ptr()?;
                if addrs > self.stack.capacity() { vm_error!("") }
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
                    None => vm_error!("")
                };
                if addrs > self.program.len() {
                    vm_error!("")
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
                if addrs > self.program.len() {
                    vm_error!("")
                }
                self.ip = addrs;
                Ok(())
            }
            Opcode::Eq => {
                let op_2 = self.stack_pop_i64()?;
                let op_1 = self.stack_pop_i64()?;
                self.push_stack(Value::Bool(op_1 == op_2).into())?;
                return Ok(());
            }
            Opcode::Neq => {
                let op_2 = self.stack_pop_i64()?;
                let op_1 = self.stack_pop_i64()?;
                self.push_stack(Value::Bool(op_1 != op_2).into())?;
                return Ok(());
            }
            Opcode::Gt => { // a b > -> a > b 
                let op_2 = self.stack_pop_i64()?; // b
                let op_1 = self.stack_pop_i64()?; // a
                self.push_stack(Value::Bool(op_1 > op_2).into())?;
                return Ok(());
            }
            Opcode::Gte => {
                let op_2 = self.stack_pop_i64()?;
                let op_1 = self.stack_pop_i64()?;
                self.push_stack(Value::Bool(op_1 >= op_2).into())?;
                return Ok(());
            }
            Opcode::Lt => {
                let op_2 = self.stack_pop_i64()?;
                let op_1 = self.stack_pop_i64()?;
                self.push_stack(Value::Bool(op_1 < op_2).into())?;
                return Ok(());
            }
            Opcode::Lte => {
                let op_2 = self.stack_pop_i64()?;
                let op_1 = self.stack_pop_i64()?;
                self.push_stack(Value::Bool(op_1 <= op_2).into())?;
                return Ok(());
            }
            Opcode::Bind => {
                let q: usize = match instr.operands {
                    Some(v) => v,
                    None => vm_error!("")
                };
                if q <= self.stack.len() {
                    for _ in 0..q {
                        let value = self.pop_stack()?;
                        self.return_stack.push(value);
                    }
                    return Ok(());
                }
                vm_error!("Bind")
            }
            Opcode::PushBind => {
                let q: usize = match instr.operands {
                    Some(v) => v,
                    None => vm_error!("PushBind 1")
                };
                let local = match self.return_stack.iter().rev().nth(q-1) {
                    Some(v) => v,
                    None => vm_error!("PushBind 2"),
                };
                self.push_stack(local.clone())?;
                return Ok(());
            }
            Opcode::Unbind => {
                let q: usize = match instr.operands {
                    Some(v) => v,
                    None => vm_error!("")
                };
                if q > self.return_stack.len() {
                    vm_error!("")
                }
                for _ in 0..q {
                    self.return_stack.pop();
                }
                return Ok(());
            }
            Opcode::Println => {
                let value = self.pop_stack()?;
                println!("{}", value.to_string());
                return Ok(());
            }
            Opcode::Print => {
                let val = self.pop_stack()?;
                print!("{}", val.to_string());
                return Ok(());

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
            Opcode::Load => {
                let addrs = self.stack_pop_ptr()?;
                let val = match self.memory.get(addrs) {
                    Some(v) => v,
                    None => vm_error!("")
                };
                self.push_stack(Rc::clone(val))?;
                Ok(())
            }
            Opcode::Store => {
                let value = self.pop_stack()?;
                let addrs = self.stack_pop_ptr()?;
                if addrs > self.memory.len() { vm_error!("") }
                self.memory[addrs] = value;
                Ok(())
            }
            Opcode::IdxGet => {
                let idx = self.pop_stack()?;
                let list = self.pop_stack()?;
                let val = match list.get_indexed(&idx) {
                    Ok(v) => v,
                    Err(e) => vm_error!("{}", e)
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
                    None => vm_error!("") 
                };
                match val.set_indexed(&idx, new_val) {
                    Some(e) => vm_error!("{}", e),
                    _ => {}
                }
                self.push_stack(Rc::new(Value::Ptr(addrs)))?;
                self.push_stack(Rc::new(val))?;
                Ok(())
            },
            Opcode::Len => {
                let val = self.pop_stack()?;
                match val.len() {
                    Ok(v) => self.push_stack(v)?,
                    Err(e) => vm_error!("{}", e)
                }
                Ok(())
            }
            Opcode::Halt => {
                self.is_halted = true;
                return Ok(());
            } //_ => Ok(())
        }
    }

    pub fn run(&mut self) {
        // println!("{}", self.ip);
        // for (i, e) in self.program.iter().enumerate() {
        //     println!("{} -> {:?}", i, e);
        // }
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
            None => vm_error!(""),
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
                    _ => vm_error!("")
                }
            }
            None => vm_error!(""),
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
                    _ => vm_error!("")
                }
            }
            None => vm_error!(""),
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
                    _ => vm_error!("")
                }
            }
            None => vm_error!(""),
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
                    _ => vm_error!("")
                }
            },
            None => vm_error!(""),
        }
    }

    fn push_stack(&mut self, value: Rc<Value>) -> Result<(), VMError> {
        if (self.sp + 1) > self.stack.capacity() {
            vm_error!("")
        }
        self.sp += 1;
        self.stack.push(value);
        Ok(())
    }
}
