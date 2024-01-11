use crate::exeptions::VMError;
use crate::config::{Word, STACK_CAPACITY, MEM_CAPACITY, Value};
use crate::instructions::{Instr, Opcode};


#[derive(Debug)]
pub struct CHSVM { // TODO: global
    pub stack: Vec<Value>,
    pub return_stack: Vec<usize>,
    pub consts: Vec<Value>,
    pub memory: Vec<Value>,
    pub is_halted: bool,
    pub ip: usize,
    sp: usize,
    program: Vec<Instr>,
}

impl CHSVM {
    pub fn new(program: Vec<Instr>, consts: Vec<Value> ) -> Self {
        let mut memory = Vec::with_capacity(MEM_CAPACITY);
        memory.resize(MEM_CAPACITY, Value::Null);
        Self {
            stack: Vec::with_capacity(STACK_CAPACITY),
            return_stack: Vec::with_capacity(STACK_CAPACITY),
            consts,
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
            Opcode::Pushi => {
                let addrs = match instr.operands {
                    Some(v) => v,
                    None => return Err(VMError::OperandNotProvided),
                };
                let val = match self.consts.get(addrs) {
                    Some(Value::Int64(v)) => *v,
                    None => return Err(VMError::AddersOutOfBounds),
                    _ => return Err(VMError::OperandNotProvided),
                };
                self.push_stack(Value::Int64(val))?;
                return Ok(());
            }
            Opcode::PushStr => {
                let addrs = match instr.operands {
                    Some(v) => v,
                    None => return Err(VMError::OperandNotProvided),
                };
                let val = match self.consts.get(addrs) {
                    Some(Value::Str(v)) => v,
                    None => return Err(VMError::AddersOutOfBounds),
                    _ => return Err(VMError::OperandNotProvided),
                };
                self.push_stack(Value::Str(val.to_string()))?;
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
                    let op_2 = self.pop_stack_as_i64()?;
                    let op_1 = self.pop_stack_as_i64()?;
                    self.push_stack(Value::Int64(op_1 + op_2))?;
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Minus => {
                if self.stack.len() >= 2 {
                    let op_2 = self.pop_stack_as_i64()?;
                    let op_1 = self.pop_stack_as_i64()?;
                    self.push_stack(Value::Int64(op_1 - op_2))?;
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Mul => {
                if self.stack.len() >= 2 {
                    let op_2 = self.pop_stack_as_i64()?;
                    let op_1 = self.pop_stack_as_i64()?;
                    self.push_stack(Value::Int64(op_1 * op_2))?;
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Div => {
                if self.stack.len() >= 2 {
                    let op_2 = self.pop_stack_as_i64()?;
                    let op_1 = self.pop_stack_as_i64()?;
                    if op_2 == 0 {
                        return Err(VMError::DivByZero);
                    }
                    self.push_stack(Value::Int64(op_1 / op_2))?;
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Mod => {
                if self.stack.len() >= 2 {
                    let op_2 = self.pop_stack_as_i64()?;
                    let op_1 = self.pop_stack_as_i64()?;
                    if op_2 == 0 {
                        return Err(VMError::DivByZero);
                    }
                    self.push_stack(Value::Int64(op_1 % op_2))?;
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Shr => {
                if self.stack.len() >= 2 {
                    let op_2 = self.pop_stack_as_i64()?;
                    let op_1 = self.pop_stack_as_i64()?;
                    self.push_stack(Value::Int64(op_1 >> op_2))?;
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Shl => {
                if self.stack.len() >= 2 {
                    let op_2 = self.pop_stack_as_i64()?;
                    let op_1 = self.pop_stack_as_i64()?;
                    self.push_stack(Value::Int64(op_1 << op_2))?;
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Bitor => {
                if self.stack.len() >= 2 {
                    let op_2 = self.pop_stack_as_i64()?;
                    let op_1 = self.pop_stack_as_i64()?;
                    self.push_stack(Value::Int64(op_1 | op_2))?;
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Bitand => {
                if self.stack.len() >= 2 {
                    let op_2 = self.pop_stack_as_i64()?;
                    let op_1 = self.pop_stack_as_i64()?;
                    self.push_stack(Value::Int64(op_1 & op_2))?;
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
                self.push_stack(Value::Int64(label as i64))?;
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
                let addrs = self.pop_stack_as_i64()?;
                if addrs < 0 || addrs > self.stack.capacity() as i64 { return Err(VMError::AddersOutOfBounds); }
                self.ip = addrs as usize;
                Ok(())
            }
            Opcode::JmpIf => {
                let op_1 = self.pop_stack_as_bool()?;
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
                let addrs: i64 = self.pop_stack_as_i64()?;
                let op_1: i64 = self.pop_stack_as_i64()?;
                if op_1 != 0 || op_1 != 1 {
                    return Err(VMError::OperandNotProvided);
                }
                if op_1 == 0 {
                    return Ok(());
                }
                self.ip = addrs as usize;
                Ok(())
            }
            Opcode::Eq => {
                if self.stack.len() >= 2 {
                    let op_2 = self.pop_stack_as_i64()?;
                    let op_1 = self.pop_stack_as_i64()?;
                    self.push_stack(Value::Int64((op_1 == op_2) as i64))?;
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Gt => {
                if self.stack.len() >= 2 { // a b > -> a > b 
                    let op_2 = self.pop_stack_as_i64()?; // b
                    let op_1 = self.pop_stack_as_i64()?; // a
                    self.push_stack(Value::Int64((op_1 > op_2) as i64))?;
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Gte => {
                if self.stack.len() >= 2 {
                    let op_2 = self.pop_stack_as_i64()?;
                    let op_1 = self.pop_stack_as_i64()?;
                    self.push_stack(Value::Int64((op_1 >= op_2) as i64))?;
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Lt => {
                if self.stack.len() >= 2 {
                    let op_2 = self.pop_stack_as_i64()?;
                    let op_1 = self.pop_stack_as_i64()?;
                    self.push_stack(Value::Int64((op_1 < op_2) as i64))?;
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Lte => {
                if self.stack.len() >= 2 {
                    let op_2 = self.pop_stack_as_i64()?;
                    let op_1 = self.pop_stack_as_i64()?;
                    self.push_stack(Value::Int64((op_1 <= op_2) as i64))?;
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
                    let val = self.pop_stack_as_str()?;
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
            Opcode::Mem => {
                self.push_stack(Value::Int64(0))?;
                return Ok(());
            }
            Opcode::Write => {
                if self.stack.len() >= 2 {
                    let op_2 = self.pop_stack_as_i64()?;
                    let op_1 = self.pop_stack_as_i64()?;
                    let mut w = String::new();
                    for i in (op_1 as usize)..=(op_2 as usize) {
                        w.push_str(&format!("{}", self.memory[i].clone().as_char()));
                    }
                    print!("{}", w);
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Load => {
                let addrs = self.pop_stack_as_i64()?;
                self.push_stack(self.memory[addrs as usize].clone())?;
                Ok(())
            }
            Opcode::Store => {
                let value = self.pop_stack()?;
                let addrs = self.pop_stack_as_i64()?;
                self.memory[addrs as usize] = value;
                Ok(())
            }
            Opcode::Halt => {
                self.is_halted = true;
                return Ok(());
            } //_ => Ok(())
        }
    }

    pub fn run(&mut self) {
        // for (i, e) in self.program.iter().enumerate() {
        //     println!("{} -> {:?}", i, e);
        // }
        while !self.is_halted {
            match self.execute_next_instr() {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("It's a trap: {} at {}", e, self.ip-1);
                    break;
                }
            }
        }
    }

    fn pop_stack(&mut self) -> Result<Value, VMError> {
        if !(self.sp == 0) {
            self.sp -= 1
        }
        match self.stack.pop() {
            Some(v) => Ok(v),
            None => Err(VMError::StackUnderflow),
        }
    }

    fn pop_stack_as_i64(&mut self) -> Result<Word, VMError> {
        if !(self.sp == 0) {
            self.sp -= 1
        }
        match self.stack.pop() {
            Some(Value::Int64(v)) => Ok(v),
            Some(Value::Null) => Ok(0),
            None => Err(VMError::StackUnderflow),
            _ => Err(VMError::OperandNotProvided)
        }
    }

    fn pop_stack_as_str(&mut self) -> Result<String, VMError> {
        if !(self.sp == 0) {
            self.sp -= 1
        }
        match self.stack.pop() {
            Some(Value::Str(v)) => Ok(v),
            None => Err(VMError::StackUnderflow),
            _ => Err(VMError::OperandNotProvided)
        }
    }

    fn pop_stack_as_bool(&mut self) -> Result<bool, VMError> {
        if !(self.sp == 0) {
            self.sp -= 1
        }
        match self.stack.pop() {
            Some(Value::Int64(v)) => Ok(v != 0 && v == 1),
            Some(Value::Uint64(v)) => Ok(v != 0 && v == 1),
            Some(Value::Str(v)) => Ok(v.len() == 0),
            Some(Value::Char(v)) => Ok(v != '\0'),
            Some(Value::Null) => Ok(false),
            None => Err(VMError::StackUnderflow),
        }
    }

    fn push_stack(&mut self, value: Value) -> Result<(), VMError> {
        if (self.sp + 1) > self.stack.capacity() {
            return Err(VMError::StackOverflow);
        }
        self.sp += 1;
        self.stack.push(value);
        Ok(())
    }
}
