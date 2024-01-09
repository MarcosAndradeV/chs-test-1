use core::fmt;

const STACK_CAPACITY: usize = 1024;

type Word = i64;

#[derive(Debug, PartialEq, Eq)]
pub enum VMError {
    StackOverflow,
    StackUnderflow,
    DivByZero,
    OperandNotProvided,
    AddersOutOfBounds,
    ProgramEndWithoutHalt,
    NotImplemeted,
}

impl fmt::Display for VMError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VMError::StackOverflow => write!(f, "StackOverflow"),
            VMError::StackUnderflow => write!(f, "StackUnderflow"),
            VMError::DivByZero => write!(f, "DivByZero"),
            VMError::OperandNotProvided => write!(f, "OperandNotProvided"),
            VMError::AddersOutOfBounds => write!(f, "AddersOutOfBounds"),
            VMError::ProgramEndWithoutHalt => write!(f, "ProgramEndWithoutHalt"),
            VMError::NotImplemeted => write!(f, "NotImplemeted"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Opcode {
    Halt = 0,
    Push,
    Pop,
    Dup,
    Swap,

    Add,
    Minus,
    Mul,
    Div,

    GetLabel,
    PushLabel,
    DropLabel,

    Jmp,
    Jmpr,
    JmpIf,
    JmpIfr,

    Eq,
    Gt,
    Lt,
    Gte,
    Lte,

    Print,
    Debug,
    Nop,
}


#[derive(Debug, Clone, Copy)]
pub struct Instr {
    kind: Opcode,
    operands: Option<Word>,
}

impl Instr {
    pub fn new(kind: Opcode, operands: Option<Word>) -> Self {
        Self { kind, operands }
    }
}

#[derive(Debug)]
pub struct CHSVM {
    pub stack: Vec<Word>,
    pub return_stack: Vec<usize>,
    pub is_halted: bool,
    pub ip: usize,
    sp: usize,
    program: Vec<Instr>,
}

impl CHSVM {
    pub fn new(program: Vec<Instr>) -> Self {
        Self {
            stack: Vec::with_capacity(STACK_CAPACITY),
            return_stack: Vec::with_capacity(STACK_CAPACITY),
            sp: 0,
            ip: 0,
            is_halted: false,
            program,
        }
    }
    pub fn execute_next_instr(&mut self) -> Result<(), VMError> {
        self.ip += 1;
        if self.ip > self.program.len() {
            return Err(VMError::ProgramEndWithoutHalt);
        }
        let instr = self.program[self.ip - 1];
        match instr.kind {
            Opcode::Push => {
                let value = match instr.operands {
                    Some(v) => v,
                    None => return Err(VMError::OperandNotProvided),
                };
                self.push_stack(value)
            }
            Opcode::Dup => {
                if self.stack.len() >= 1 {
                    let op_1 = self.pop_stack()?;
                    self.push_stack(op_1)?;
                    self.push_stack(op_1)?;
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Swap => {
                if self.stack.len() >= 2 {
                    let op_1 = self.pop_stack()?;
                    let op_2 = self.pop_stack()?;
                    self.push_stack(op_1)?;
                    self.push_stack(op_2)?;
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
                    let op_1 = self.pop_stack()?;
                    let op_2 = self.pop_stack()?;
                    self.push_stack(op_1 + op_2)?;
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Minus => {
                if self.stack.len() >= 2 {
                    let op_2 = self.pop_stack()?;
                    let op_1 = self.pop_stack()?;
                    self.push_stack(op_1 - op_2)?;
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Mul => {
                if self.stack.len() >= 2 {
                    let op_1 = self.pop_stack()?;
                    let op_2 = self.pop_stack()?;
                    self.push_stack(op_1 * op_2)?;
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Div => {
                if self.stack.len() >= 2 {
                    let op_2 = self.pop_stack()?;
                    let op_1 = self.pop_stack()?;
                    if op_2 == 0 {
                        return Err(VMError::DivByZero);
                    }
                    self.push_stack(op_1 / op_2)?;
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::PushLabel => {
                self.return_stack.push(self.ip);
                return Err(VMError::NotImplemeted);
            }
            Opcode::GetLabel => {
                let addrs = match instr.operands {
                    Some(v) => {
                        if v < 0 { return Err(VMError::OperandNotProvided); }
                        v as usize
                    },
                    None => return Err(VMError::OperandNotProvided),
                };
                if addrs > self.program.len() {
                    return Err(VMError::AddersOutOfBounds);
                }
                let label = match self.return_stack.get(addrs) {
                    Some(v) => *v,
                    None => return Err(VMError::StackUnderflow)
                };
                self.push_stack(label as i64)?;
                return Err(VMError::NotImplemeted);
            }
            Opcode::DropLabel => {
                self.return_stack.pop();
                return Err(VMError::NotImplemeted);
            }
            Opcode::Jmp => {
                let addrs = match instr.operands {
                    Some(v) => {
                        if v < 0 { return Err(VMError::OperandNotProvided); }
                        v as usize
                    },
                    None => return Err(VMError::OperandNotProvided),
                };
                if addrs > self.program.len() {
                    return Err(VMError::AddersOutOfBounds);
                }
                self.ip = addrs;
                Ok(())
            }
            Opcode::Jmpr => {
                self.ip = self.pop_stack()? as usize;
                Ok(())
            }
            Opcode::JmpIf => {
                let op_1 = self.pop_stack()?;
                if op_1 == 0 || op_1 == 1 {return Err(VMError::OperandNotProvided);}
                if op_1 == 0 {return Ok(());}
                let addrs = match instr.operands {
                    Some(v) => {
                        if v < 0 { return Err(VMError::OperandNotProvided); }
                        v as usize
                    },
                    None => return Err(VMError::OperandNotProvided),
                };
                if addrs > self.program.len() {
                    return Err(VMError::AddersOutOfBounds);
                }
                
                self.ip = addrs;
                
                Ok(())
            }
            Opcode::JmpIfr => {
                let addrs: i64 = self.pop_stack()?;
                let op_1: i64 = self.pop_stack()?;
                if op_1 == 0 || op_1 == 1 {return Err(VMError::OperandNotProvided);}
                if op_1 == 0 {return Ok(());}
                self.ip = addrs as usize;
                Ok(())
            }
            Opcode::Eq => {
                if self.stack.len() >= 2 {
                    let op_1 = self.pop_stack()?;
                    let op_2 = self.pop_stack()?;
                    self.push_stack((op_1 == op_2) as i64)?;
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Gt => {
                if self.stack.len() >= 2 {
                    let op_2 = self.pop_stack()?;
                    let op_1 = self.pop_stack()?;
                    self.push_stack((op_1 < op_2) as i64)?;
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Gte => {
                if self.stack.len() >= 2 {
                    let op_2 = self.pop_stack()?;
                    let op_1 = self.pop_stack()?;
                    self.push_stack((op_1 <= op_2) as i64)?;
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Lt => {
                if self.stack.len() >= 2 {
                    let op_2 = self.pop_stack()?;
                    let op_1 = self.pop_stack()?;
                    self.push_stack((op_1 > op_2) as i64)?;
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }
            Opcode::Lte => {
                if self.stack.len() >= 2 {
                    let op_2 = self.pop_stack()?;
                    let op_1 = self.pop_stack()?;
                    self.push_stack((op_1 >= op_2) as i64)?;
                    return Ok(());
                }
                return Err(VMError::StackUnderflow);
            }

            Opcode::Print => {
                if self.stack.len() >= 1 {
                    let value = self.pop_stack()?;
                    println!("Output: {}", value);
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
            Opcode::Halt => {
                self.is_halted = true;
                return Ok(());
            } //_ => Ok(())
        }
    }

    pub fn run(&mut self) {
        while !self.is_halted {
            match self.execute_next_instr() {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("It's a trap: {}", e);
                    break;
                }
            }
        }
    }

    fn pop_stack(&mut self) -> Result<Word, VMError> {
        if !(self.sp == 0) {
            self.sp -= 1
        }
        match self.stack.pop() {
            Some(v) => Ok(v),
            None => Err(VMError::StackUnderflow),
        }
    }

    fn push_stack(&mut self, value: Word) -> Result<(), VMError> {
        if (self.sp + 1) > self.stack.capacity() {
            return Err(VMError::StackOverflow);
        }
        self.sp += 1;
        self.stack.push(value);
        Ok(())
    }
}
