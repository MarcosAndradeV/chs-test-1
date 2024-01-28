use std::ops::Deref;
use std::rc::Rc;

use crate::config::{MEM_CAPACITY, STACK_CAPACITY};
use crate::exeptions::VMError;
use crate::instructions::{Builtin, Bytecode, Instr, Opcode};
use crate::value::Value;
use crate::vm_error;

#[derive(Debug)]
pub struct CHSVM {
    stack: Vec<Rc<Value>>,
    return_stack: Vec<Rc<Value>>,
    memory: Vec<Rc<Value>>,
    ip: usize,
    sp: usize,
    program: Bytecode,
}

impl CHSVM {
    pub fn new(program: Bytecode) -> Self {
        let mut memory = Vec::with_capacity(MEM_CAPACITY);
        memory.resize(MEM_CAPACITY, Rc::new(Value::Nil));
        Self {
            stack: Vec::with_capacity(STACK_CAPACITY),
            return_stack: Vec::with_capacity(STACK_CAPACITY),
            memory,
            sp: 0,
            ip: 0,
            program,
        }
    }
    pub fn execute_instr(&mut self, instr: Instr) -> Result<(), VMError> {
        match instr.kind {
            Opcode::Const => {
                let addrs = match instr.operands {
                    Some(v) => v,
                    None => vm_error!("OPERAND_NOT_PROVIDED"),
                };
                let val = match self.program.consts.get(addrs) {
                    Some(v) => v,
                    None => vm_error!("{:?} operand is not provided.", instr.kind),
                };
                self.push_stack(Rc::new(val.clone()))?;
                self.ip += 1;
                return Ok(());
            }
            Opcode::Dup => {
                let op_1 = self.stack_pop()?;
                self.push_stack(op_1.clone())?;
                self.push_stack(op_1)?;
                self.ip += 1;
                return Ok(());
            }
            Opcode::Dup2 => {
                // a b -> a b a b
                let op_2 = self.stack_pop()?; // b
                let op_1 = self.stack_pop()?; // a
                self.push_stack(op_1.clone())?;
                self.push_stack(op_2.clone())?;
                self.push_stack(op_1)?;
                self.push_stack(op_2)?;
                self.ip += 1;
                return Ok(());
            }
            Opcode::Swap => {
                // a b -> b a
                let op_2 = self.stack_pop()?; // b
                let op_1 = self.stack_pop()?; // a
                self.push_stack(op_2)?; // b
                self.push_stack(op_1)?; // a
                self.ip += 1;
                return Ok(());
            }
            Opcode::Over => {
                // a b -> a b a
                let op_2 = self.stack_pop()?; // b
                let op_1 = self.stack_pop()?; // a
                self.push_stack(op_1.clone())?; // a
                self.push_stack(op_2)?; // b
                self.push_stack(op_1)?; // a
                self.ip += 1;
                return Ok(());
            }
            Opcode::Pop => {
                // a -> _
                let _ = self.stack_pop()?;
                self.ip += 1;
                return Ok(());
            }
            Opcode::Add => {
                let op_2 = self.stack_pop()?;
                let op_1 = self.stack_pop()?;

                if op_1.is_ptr() || op_2.is_ptr() {
                    vm_error!("")
                }

                match *op_1 {
                    Value::Int64(v) => match *op_2 {
                        Value::Int64(o) => self.push_stack(Rc::new(Value::Int64(v + o)))?,
                        Value::Uint64(o) => self.push_stack(Rc::new(Value::Int64(v + o as i64)))?,
                        _ => vm_error!(""),
                    },
                    Value::Uint64(v) => match *op_2 {
                        Value::Uint64(o) => self.push_stack(Rc::new(Value::Uint64(v + o)))?,
                        Value::Int64(o) => self.push_stack(Rc::new(Value::Uint64(v + o as u64)))?,
                        _ => vm_error!(""),
                    },
                    _ => vm_error!(""),
                }
                self.ip += 1;
                return Ok(());
            }
            Opcode::Minus => {
                let op_2 = self.stack_pop()?;
                let op_1 = self.stack_pop()?;

                if op_1.is_ptr() || op_2.is_ptr() {
                    vm_error!("")
                }

                match *op_1 {
                    Value::Int64(v) => match *op_2 {
                        Value::Int64(o) => self.push_stack(Rc::new(Value::Int64(v - o)))?,
                        Value::Uint64(o) => self.push_stack(Rc::new(Value::Int64(v - o as i64)))?,
                        _ => vm_error!(""),
                    },
                    Value::Uint64(v) => match *op_2 {
                        Value::Uint64(o) => self.push_stack(Rc::new(Value::Uint64(v - o)))?,
                        Value::Int64(o) => self.push_stack(Rc::new(Value::Uint64(v - o as u64)))?,
                        _ => vm_error!(""),
                    },
                    _ => vm_error!(""),
                }
                self.ip += 1;
                return Ok(());
            }
            Opcode::Mul => {
                let op_2 = self.stack_pop()?;
                let op_1 = self.stack_pop()?;

                if op_1.is_ptr() || op_2.is_ptr() {
                    vm_error!("")
                }

                match *op_1 {
                    Value::Int64(v) => match *op_2 {
                        Value::Int64(o) => self.push_stack(Rc::new(Value::Int64(v * o)))?,
                        Value::Uint64(o) => self.push_stack(Rc::new(Value::Int64(v * o as i64)))?,
                        _ => vm_error!(""),
                    },
                    Value::Uint64(v) => match *op_2 {
                        Value::Uint64(o) => self.push_stack(Rc::new(Value::Uint64(v * o)))?,
                        Value::Int64(o) => self.push_stack(Rc::new(Value::Uint64(v * o as u64)))?,
                        _ => vm_error!(""),
                    },
                    _ => vm_error!(""),
                }
                self.ip += 1;
                return Ok(());
            }
            Opcode::Div => {
                let op_2 = self.stack_pop()?;
                let op_1 = self.stack_pop()?;

                if op_1.is_ptr() || op_2.is_ptr() {
                    vm_error!("")
                }

                match *op_1 {
                    Value::Int64(v) => match *op_2 {
                        Value::Int64(o) => {
                            if o == 0 {
                                vm_error!("")
                            }
                            self.push_stack(Rc::new(Value::Int64(v / o)))?
                        }
                        Value::Uint64(o) => {
                            if o == 0 {
                                vm_error!("")
                            }
                            self.push_stack(Rc::new(Value::Int64(v / o as i64)))?
                        }
                        _ => vm_error!(""),
                    },
                    Value::Uint64(v) => match *op_2 {
                        Value::Uint64(o) => {
                            if o == 0 {
                                vm_error!("")
                            }
                            self.push_stack(Rc::new(Value::Uint64(v / o)))?
                        }
                        Value::Int64(o) => {
                            if o == 0 {
                                vm_error!("")
                            }
                            self.push_stack(Rc::new(Value::Uint64(v / o as u64)))?
                        }
                        _ => vm_error!(""),
                    },
                    _ => vm_error!(""),
                }
                self.ip += 1;
                return Ok(());
            }
            Opcode::Mod => {
                let op_2 = self.stack_pop()?;
                let op_1 = self.stack_pop()?;

                if op_1.is_ptr() || op_2.is_ptr() {
                    vm_error!("")
                }

                match *op_1 {
                    Value::Int64(v) => match *op_2 {
                        Value::Int64(o) => {
                            if o == 0 {
                                vm_error!("")
                            }
                            self.push_stack(Rc::new(Value::Int64(v % o)))?
                        }
                        Value::Uint64(o) => {
                            if o == 0 {
                                vm_error!("")
                            }
                            self.push_stack(Rc::new(Value::Int64(v % o as i64)))?
                        }
                        _ => vm_error!(""),
                    },
                    Value::Uint64(v) => match *op_2 {
                        Value::Uint64(o) => {
                            if o == 0 {
                                vm_error!("")
                            }
                            self.push_stack(Rc::new(Value::Uint64(v % o)))?
                        }
                        Value::Int64(o) => {
                            if o == 0 {
                                vm_error!("")
                            }
                            self.push_stack(Rc::new(Value::Uint64(v % o as u64)))?
                        }
                        _ => vm_error!(""),
                    },
                    _ => vm_error!(""),
                }
                self.ip += 1;
                return Ok(());
            }
            Opcode::Shr => {
                let op_2 = self.stack_pop_u64()?;
                let op_1 = self.stack_pop_u64()?;
                self.push_stack(Value::Uint64(op_1 >> op_2).into())?;
                self.ip += 1;
                return Ok(());
            }
            Opcode::Shl => {
                let op_2 = self.stack_pop_u64()?;
                let op_1 = self.stack_pop_u64()?;
                self.push_stack(Value::Uint64(op_1 << op_2).into())?;
                self.ip += 1;
                return Ok(());
            }
            Opcode::Lor => {
                let op_2 = self.stack_pop_bool()?;
                let op_1 = self.stack_pop_bool()?;
                self.push_stack(Value::Bool(op_1 || op_2).into())?;
                self.ip += 1;
                return Ok(());
            }
            Opcode::Land => {
                let op_2 = self.stack_pop_bool()?;
                let op_1 = self.stack_pop_bool()?;
                self.push_stack(Value::Bool(op_1 && op_2).into())?;
                self.ip += 1;
                return Ok(());
            }
            Opcode::Bitor => {
                let op_2 = self.stack_pop_u64()?;
                let op_1 = self.stack_pop_u64()?;
                self.push_stack(Value::Uint64(op_1 | op_2).into())?;
                self.ip += 1;
                return Ok(());
            }
            Opcode::Bitand => {
                let op_2 = self.stack_pop_u64()?;
                let op_1 = self.stack_pop_u64()?;
                self.push_stack(Value::Uint64(op_1 & op_2).into())?;
                self.ip += 1;
                return Ok(());
            }
            Opcode::Jmp => {
                let addrs = match instr.operands {
                    Some(v) => v,
                    None => vm_error!("{:?} operand is not provided.", instr.kind),
                };
                if addrs > self.program.len() {
                    vm_error!("Address out of bounds.")
                }
                self.ip = addrs;
                Ok(())
            }
            Opcode::JmpIf => {
                let op_1 = self.stack_pop_bool()?;
                if op_1 {
                    self.ip += 1;
                    return Ok(());
                }
                let addrs = match instr.operands {
                    Some(v) => v,
                    None => vm_error!("{:?} operand is not provided.", instr.kind),
                };
                if addrs > self.program.len() {
                    vm_error!("Address out of bounds.")
                }

                self.ip = addrs;

                Ok(())
            }
            Opcode::Eq => {
                let op_2 = self.stack_pop()?;
                let op_1 = self.stack_pop()?;
                self.push_stack(Value::Bool(op_1 == op_2).into())?;
                self.ip += 1;
                return Ok(());
            }
            Opcode::Neq => {
                let op_2 = self.stack_pop()?;
                let op_1 = self.stack_pop()?;
                self.push_stack(Value::Bool(op_1 != op_2).into())?;
                self.ip += 1;
                return Ok(());
            }
            Opcode::Gt => {
                // a b > -> a > b
                let op_2 = self.stack_pop()?; // b
                let op_1 = self.stack_pop()?; // a
                self.push_stack(Value::Bool(op_1 > op_2).into())?;
                self.ip += 1;
                return Ok(());
            }
            Opcode::Gte => {
                let op_2 = self.stack_pop()?;
                let op_1 = self.stack_pop()?;
                self.push_stack(Value::Bool(op_1 >= op_2).into())?;
                self.ip += 1;
                return Ok(());
            }
            Opcode::Lt => {
                let op_2 = self.stack_pop()?;
                let op_1 = self.stack_pop()?;
                self.push_stack(Value::Bool(op_1 < op_2).into())?;
                self.ip += 1;
                return Ok(());
            }
            Opcode::Lte => {
                let op_2 = self.stack_pop()?;
                let op_1 = self.stack_pop()?;
                self.push_stack(Value::Bool(op_1 <= op_2).into())?;
                self.ip += 1;
                return Ok(());
            }
            Opcode::Bind => {
                let q: usize = match instr.operands {
                    Some(v) => v,
                    None => vm_error!("{:?} operand is not provided.", instr.kind),
                };
                if q <= self.stack.len() {
                    for _ in 0..q {
                        let value = self.stack_pop()?;
                        self.return_stack.push(value);
                    }
                    self.ip += 1;
                    return Ok(());
                }
                vm_error!(
                    "Not enough items on the data stack for bind\nNeed [{}] encountered [{}]",
                    q,
                    self.stack.len()
                )
            }
            Opcode::PushBind => {
                let q: usize = match instr.operands {
                    Some(v) => v,
                    None => vm_error!("{:?} operand is not provided.", instr.kind),
                };
                if let Some(local) = self.return_stack.get(q) {
                    self.push_stack(local.clone())?;
                    self.ip += 1;
                    return Ok(());
                } else {
                    vm_error!("return stack underflow")
                }
            }
            Opcode::Unbind => {
                let q: usize = match instr.operands {
                    Some(v) => v,
                    None => vm_error!("{:?} operand is not provided.", instr.kind),
                };
                if q > self.return_stack.len() {
                    vm_error!("")
                }
                for _ in 0..q {
                    self.return_stack.pop();
                }
                self.ip += 1;
                return Ok(());
            }
            Opcode::Nop => {
                self.ip += 1;
                return Ok(());
            }
            Opcode::GlobalLoad => {
                let addrs = match instr.operands {
                    Some(v) => v,
                    None => vm_error!("OPERAND_NOT_PROVIDED"),
                };
                let val = match self.memory.get(addrs) {
                    Some(v) => v,
                    None => vm_error!("{:?} operand is not provided.", instr.kind),
                };
                self.push_stack(Rc::clone(val))?;
                self.ip += 1;
                Ok(())
            }
            Opcode::GlobalStore => {
                let addrs = match instr.operands {
                    Some(v) => v,
                    None => vm_error!("OPERAND_NOT_PROVIDED"),
                };
                let value = self.stack_pop()?;
                if addrs > self.memory.len() {
                    let mem = self.memory.len();
                    self.memory.resize(mem + MEM_CAPACITY, Rc::new(Value::Nil));
                }
                self.memory[addrs] = value;
                self.ip += 1;
                Ok(())
            }
            Opcode::Buildin => {
                let typ: usize = match instr.operands {
                    Some(v) => v,
                    None => vm_error!("{:?} operand is not provided.", instr.kind),
                };
                let buildin = Builtin::from(typ);
                if buildin.is_invalid() {
                    vm_error!("")
                }
                match buildin {
                    Builtin::IdxGet => {
                        let idx = self.stack_pop()?;
                        let list = self.stack_pop()?;
                        let val = match list.get_indexed(&idx) {
                            Ok(v) => v,
                            Err(e) => vm_error!("{}", e),
                        };
                        self.push_stack(val)?;
                        self.ip += 1;
                        return Ok(());
                    }
                    Builtin::IdxSet => {
                        let new_val = self.stack_pop()?;
                        let idx = self.stack_pop()?;
                        let mut list = self.stack_pop()?.deref().to_owned();
                        list.set_indexed(&idx, new_val);
                        self.push_stack(Rc::new(list))?;
                        self.ip += 1;
                        return Ok(());
                    }
                    Builtin::Len => {
                        let val = self.stack_pop()?;
                        match val.len() {
                            Ok(v) => self.push_stack(v)?,
                            Err(e) => vm_error!("{}", e),
                        }
                        self.ip += 1;
                        return Ok(());
                    }
                    Builtin::Println => {
                        let value = self.stack_pop()?;
                        println!("{}", value.to_string());
                        self.ip += 1;
                        return Ok(());
                    }
                    Builtin::Print => {
                        let val = self.stack_pop()?;
                        print!("{}", val.to_string());
                        self.ip += 1;
                        return Ok(());
                    }
                    Builtin::Debug => {
                        println!(
                            "CHSVM: {:?}, SP: {}, STACK_LEN: {}",
                            self.stack,
                            self.sp,
                            self.stack.len()
                        );
                        self.ip += 1;
                        return Ok(());
                    }
                    Builtin::Length => todo!(),
                    Builtin::Builtins => todo!(),
                    Builtin::TimeUnix => todo!(),
                    Builtin::Args => todo!(),
                    Builtin::Exit => todo!(),
                    Builtin::TypeOf => todo!(),
                    Builtin::CallFunc => todo!(),
                    Builtin::FStat => todo!(),
                    Builtin::FWrite => todo!(),
                    Builtin::FAppend => todo!(),
                    Builtin::FRead => todo!(),
                    Builtin::ReadLine => todo!(),
                    Builtin::SWrite => todo!(),
                    Builtin::SRead => todo!(),
                    Builtin::GetSyscalls => todo!(),
                    Builtin::Syscall => todo!(),
                    Builtin::Invalid => todo!(),
                }
            }
            Opcode::Halt => {
                return Ok(());
            }
        }
    }

    pub fn run(&mut self, debug: bool) {
        if debug {
            for (i, e) in self.program.program.iter().enumerate() {
                println!("{} -> {}", i, e);
            }
        }
        loop {
            if self.ip >= self.program.len() {
                break;
            }
            let instr = self.program.program[self.ip];
            match self.execute_instr(instr) {
                Ok(_) => {} // {println!("{:?} at {}", self.return_stack, self.ip);}
                Err(e) => {
                    eprintln!(
                        "It's a trap: {} at {} {}",
                        e, self.ip, self.program.program[self.ip]
                    );
                    break;
                }
            }
        }
    }

    fn stack_pop(&mut self) -> Result<Rc<Value>, VMError> {
        if !(self.sp == 0) {
            self.sp -= 1
        }
        match self.stack.pop() {
            Some(v) => Ok(v),
            None => vm_error!(""),
        }
    }

    fn stack_pop_u64(&mut self) -> Result<u64, VMError> {
        if !(self.sp == 0) {
            self.sp -= 1
        }
        match self.stack.pop() {
            Some(v) => match *v {
                Value::Uint64(v) => Ok(v),
                Value::Int64(v) => Ok(v as u64),
                _ => vm_error!(""),
            },
            None => vm_error!(""),
        }
    }

    fn stack_pop_bool(&mut self) -> Result<bool, VMError> {
        if !(self.sp == 0) {
            self.sp -= 1
        }
        match self.stack.pop() {
            Some(v) => match v.as_ref() {
                Value::Bool(v) => Ok(*v),
                e => vm_error!("Expected BOOL found {}", e),
            },
            None => vm_error!("Stack underflow"),
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
