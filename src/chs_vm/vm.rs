
use crate::{config::STACK_CAPACITY, exeptions::VMError, vm_error};

use super::{instructions::{Bytecode, Instr, Opcode}, value::Value};



#[derive(Debug)]
pub struct CHSVM {
    stack: Vec<Value>,
    temp_stack: Vec<Value>,
    return_stack: Vec<usize>,
    ip: usize,
    sp: usize,
    program: Bytecode,
}

impl CHSVM {
    pub fn new(program: Bytecode) -> Self {
        Self {
            stack: Vec::with_capacity(STACK_CAPACITY),
            temp_stack: Vec::with_capacity(STACK_CAPACITY),
            return_stack: Vec::with_capacity(STACK_CAPACITY),
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
                self.push_stack(val.clone())?;
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

                match (op_1, op_2) {
                    (Value::Int64(v), Value::Int64(o)) => {
                        self.push_stack(Value::Int64(v + o))?;
                    }
                    (v, o) => vm_error!("Cannot perform {} + {}", v, o),
                }
                self.ip += 1;
                return Ok(());
            }
            Opcode::Minus => {
                let op_2 = self.stack_pop()?;
                let op_1 = self.stack_pop()?;

                match (op_1, op_2) {
                    (Value::Int64(v), Value::Int64(o)) => {
                        self.push_stack(Value::Int64(v - o))?;
                    }
                    (v, o) => vm_error!("Cannot perform {} - {}", v, o),
                }
                self.ip += 1;
                return Ok(());
            }
            Opcode::Mul => {
                let op_2 = self.stack_pop()?;
                let op_1 = self.stack_pop()?;

                match (op_1, op_2) {
                    (Value::Int64(v), Value::Int64(o)) => {
                        self.push_stack(Value::Int64(v * o))?;
                    }
                    (v, o) => vm_error!("Cannot perform {} * {}", v, o),
                }
                self.ip += 1;
                return Ok(());
            }
            Opcode::Div => {
                let op_2 = self.stack_pop()?;
                let op_1 = self.stack_pop()?;

                match (op_1, op_2) {
                    (_, Value::Int64(0)) => {
                        vm_error!("Cannot divide by zero")
                    }
                    (Value::Int64(v), Value::Int64(o)) => {
                        self.push_stack(Value::Int64(v / o))?;
                    }
                    (v, o) => vm_error!("Cannot perform {} / {}", v, o),
                }
                self.ip += 1;
                return Ok(());
            }
            Opcode::Mod => {
                let op_2 = self.stack_pop()?;
                let op_1 = self.stack_pop()?;

                match (op_1, op_2) {
                    (_, Value::Int64(0)) => {
                        vm_error!("Cannot divide by zero")
                    }
                    (Value::Int64(v), Value::Int64(o)) => {
                        self.push_stack(Value::Int64(v % o))?;
                    }
                    (v, o) => vm_error!("Cannot perform {} % {}", v, o),
                }
                self.ip += 1;
                return Ok(());
            }
            Opcode::Shr => {
                let op_2 = self.stack_pop_i64()?;
                let op_1 = self.stack_pop_i64()?;
                self.push_stack(Value::Int64(op_1 >> op_2))?;
                self.ip += 1;
                return Ok(());
            }
            Opcode::Shl => {
                let op_2 = self.stack_pop_i64()?;
                let op_1 = self.stack_pop_i64()?;
                self.push_stack(Value::Int64(op_1 << op_2))?;
                self.ip += 1;
                return Ok(());
            }
            Opcode::Lor => {
                let op_2 = self.stack_pop_bool()?;
                let op_1 = self.stack_pop_bool()?;
                self.push_stack(Value::Bool(op_1 || op_2))?;
                self.ip += 1;
                return Ok(());
            }
            Opcode::Land => {
                let op_2 = self.stack_pop_bool()?;
                let op_1 = self.stack_pop_bool()?;
                self.push_stack(Value::Bool(op_1 && op_2))?;
                self.ip += 1;
                return Ok(());
            }
            Opcode::Bitor => {
                let op_2 = self.stack_pop_i64()?;
                let op_1 = self.stack_pop_i64()?;
                self.push_stack(Value::Int64(op_1 | op_2))?;
                self.ip += 1;
                return Ok(());
            }
            Opcode::Bitand => {
                let op_2 = self.stack_pop_i64()?;
                let op_1 = self.stack_pop_i64()?;
                self.push_stack(Value::Int64(op_1 & op_2))?;
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
                        self.temp_stack.push(value);
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
                if let Some(local) = self.temp_stack.get(q) {
                    self.push_stack(local.clone())?;
                    self.ip += 1;
                    return Ok(());
                } else {
                    vm_error!("return stack underflow")
                }
            }
            Opcode::SetBind => {
                let q: usize = match instr.operands {
                    Some(v) => v,
                    None => vm_error!("{:?} operand is not provided.", instr.kind),
                };
                let value = self.stack_pop()?;
                if let Some(_) = self.temp_stack.get(q) {
                    self.temp_stack[q] = value;
                    self.ip += 1;
                    return Ok(());
                }
                vm_error!("return stack underflow");
            }
            Opcode::Unbind => {
                let q: usize = match instr.operands {
                    Some(v) => v,
                    None => vm_error!("{:?} operand is not provided.", instr.kind),
                };
                if q > self.temp_stack.len() {
                    vm_error!("")
                }
                for _ in 0..q {
                    self.temp_stack.pop();
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
                let val = match self.temp_stack.get(addrs) {
                    Some(v) => v,
                    None => vm_error!("{:?} operand is not provided.", instr.kind),
                };
                self.push_stack(val.clone())?;
                self.ip += 1;
                Ok(())
            }
            Opcode::GlobalStore => {
                let addrs = match instr.operands {
                    Some(v) => v,
                    None => vm_error!("OPERAND_NOT_PROVIDED"),
                };
                let value = self.stack_pop()?;
                if addrs >= self.temp_stack.len() {
                    self.temp_stack.push(value);
                    self.ip += 1;
                    return Ok(());
                }
                self.temp_stack[addrs] = value;
                self.ip += 1;
                Ok(())
            }
            Opcode::SkipFn => {
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
            Opcode::CallFn => {
                let addrs = match instr.operands {
                    Some(v) => v,
                    None => vm_error!("{:?} operand is not provided.", instr.kind),
                };
                if addrs > self.program.len() {
                    vm_error!("Address out of bounds.")
                }
                self.return_stack.push(self.ip + 1);
                self.ip = addrs;
                Ok(())
            }
            Opcode::RetFn => {
                let addrs = match self.return_stack.pop() {
                    Some(v) => v,
                    _ => vm_error!("Cannot return from inside of a peek block."),
                };
                if addrs > self.program.len() {
                    vm_error!("Address out of bounds.")
                }
                self.ip = addrs;
                Ok(())
            }
            Opcode::Debug  => {
                println!("Debug:\nData Stack: {:?}\nTemp Stack: {:?}", self.stack, self.temp_stack);
                self.ip += 1;
                Ok(())
            }
            Opcode::Exit   => {
                self.ip += 1;
                Ok(())
            }
            Opcode::Print  => {
                let val = self.stack_pop()?;
                print!("{val}");
                self.ip += 1;
                Ok(())
            }
            Opcode::IdxGet => {
                let idx = self.stack_pop()?;
                let val = self.stack_pop()?;
                if !val.is_list() && !val.is_str() {
                    vm_error!("Cannot index {} with {}", val, idx)
                }
                self.push_stack(val.get_indexed(idx))?;
                self.ip += 1;
                Ok(())
            }
            Opcode::IdxSet => {
                let new_val = self.stack_pop()?;
                let idx = self.stack_pop()?;
                let val = self.stack_pop()?;
                if !val.is_list() {
                    vm_error!("Cannot index {} with {}", val, idx)
                }
                self.push_stack(val.set_indexed(idx, new_val))?;
                self.ip += 1;
                Ok(())
            }
            Opcode::Len    => {
                let val = self.stack_pop()?;
                if !val.is_list() && !val.is_str() {
                    vm_error!("Cannot get length of {}", val)
                }
                self.push_stack(val.len())?;
                self.ip += 1;
                Ok(())
            }
            Opcode::Concat    => {
                let other = self.stack_pop()?;
                let val = self.stack_pop()?;
                match (&val, &other) {
                    (Value::Array(_), Value::Array(_)) => {},
                    (Value::Str(_), Value::Str(_)) => {},
                    (v, o) => vm_error!("Cannot concat {} with {}", v, o),
                }
                self.push_stack(val.concat(other))?;
                self.ip += 1;
                Ok(())
            }
            Opcode::Tail    => {
                let val = self.stack_pop()?;
                if !val.is_list() && !val.is_str() {
                    vm_error!("Cannot get tail of {}", val)
                }
                self.push_stack(val.tail())?;
                self.ip += 1;
                Ok(())
            }
            Opcode::Head    => {
                let val = self.stack_pop()?;
                if !val.is_list() && !val.is_str() {
                    vm_error!("Cannot get head of {}", val)
                }
                self.push_stack(val.head())?;
                self.ip += 1;
                Ok(())
            }
            Opcode::Call => {
                let val = self.stack_pop()?;
                match val {
                    Value::Fn(v, _) => {
                        self.return_stack.push(self.ip + 1);
                        self.ip = v;
                        Ok(())
                    },
                    v => vm_error!("Cannot call {}", v),
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
                Ok(_) => {} // {println!("{:?} at {}", self.stack, self.ip);}
                Err(e) => {
                    eprintln!(
                        "It's a trap: {} at {} Instr({})",
                        e, self.ip, self.program.program[self.ip]
                    );
                    break;
                }
            }
        }
    }

    fn stack_pop(&mut self) -> Result<Value, VMError> {
        if !(self.sp == 0) {
            self.sp -= 1
        }
        match self.stack.pop() {
            Some(v) => Ok(v),
            None => vm_error!("Stack uderflow"),
        }
    }

    fn stack_pop_i64(&mut self) -> Result<i64, VMError> {
        if !(self.sp == 0) {
            self.sp -= 1
        }
        match self.stack.pop() {
            Some(v) => match v {
                Value::Int64(v) => Ok(v),
                Value::Bool(v) => Ok(v as i64),
                Value::Nil => Ok(0),
                a => vm_error!("{} cannot be dynanmic converted into int", a),
            },
            None => vm_error!("Stack uderflow"),
        }
    }

    fn stack_pop_bool(&mut self) -> Result<bool, VMError> {
        if !(self.sp == 0) {
            self.sp -= 1
        }
        match self.stack.pop() {
            Some(v) => match v {
                Value::Bool(v) => Ok(v),
                Value::Nil => Ok(false),
                Value::Array(v) => Ok(!v.is_empty()),
                Value::Str(v) => Ok(!v.is_empty()),
                Value::Int64(v) => Ok(v != 0),
                Value::Char(v) => Ok(v != '\0'),
                _ => Ok(false),
            },
            None => vm_error!("Stack underflow"),
        }
    }

    fn push_stack(&mut self, value: Value) -> Result<(), VMError> {
        if (self.sp + 1) > self.stack.capacity() {
            vm_error!("Stack overflow")
        }
        self.sp += 1;
        self.stack.push(value);
        Ok(())
    }
}
