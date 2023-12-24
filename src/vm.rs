
const STACK_CAPACITY: usize = 1024;

type Word = i64;

#[derive(Debug, PartialEq, Eq)]
pub enum Trap {
    StackOverflow,
    StackUnderflow,
    DivByZero,
    OperandNotProvided,
    AddersOutOfBounds,
    ProgramEndWithoutHalt,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InstrKind {
    Halt = 0,
    Push,
    Pop,
    Dup,
    Swap,

    Add,
    Minus,
    Mul,
    Div,

    Jmp,
    JmpIf,

    Eq,
    Gt,
    Lt,
    Gte,
    Lte,

    Print,
    Debug,
    Nop,

}

impl From<u8> for InstrKind {
    fn from(value: u8) -> Self {
        match value {
            0 => InstrKind::Halt,
            1 => InstrKind::Push,
            2 => InstrKind::Pop,
            3 => InstrKind::Dup,
            4 => InstrKind::Swap,
        
            5 => InstrKind::Add,
            6 => InstrKind::Minus,
            7 => InstrKind::Mul,
            8 => InstrKind::Div,
        
            9 => InstrKind::Jmp,
            10 => InstrKind::JmpIf,
        
            11 => InstrKind::Eq,
            12 => InstrKind::Gt,
            13 => InstrKind::Lt,
            14 => InstrKind::Gte,
            15 => InstrKind::Lte,
        
            16 => InstrKind::Print,
            17 => InstrKind::Debug,
            _  => InstrKind::Nop,
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Instr {
    kind: InstrKind,
    operands: Option<Word>,
}

impl Instr {
    pub fn new(kind: InstrKind, operands: Option<Word>) -> Self { Self { kind, operands } }
}

#[derive(Debug)]
pub struct CHSVM {
    pub stack: Vec<Word>,
    pub is_halted: bool,
    pub ip: usize,
    sp: usize,
    program: Vec<Instr>,

}

impl CHSVM {
    pub fn new(program: Vec<Instr>) -> Self {
        Self { stack: Vec::with_capacity(STACK_CAPACITY), sp: 0, ip: 0, is_halted: false, program }
    }
    pub fn execute_next_instr(&mut self) -> Result<(), Trap>{
        self.ip+=1;
        if self.ip > self.program.len() {
            return Err(Trap::ProgramEndWithoutHalt);
        }
        let instr = self.program[self.ip - 1];
        match instr.kind {
            InstrKind::Push => {

                let value = match instr.operands {
                    Some(v) => v,
                    None => return Err(Trap::OperandNotProvided)
                };
                self.push_stack(value)
            },
            InstrKind::Dup => {
                // if self.stack.len() >= 1 {
                //     let op_1 = self.pop_stack()?;
                //     self.push_stack(op_1)?;
                //     self.push_stack(op_1)?;
                // }
                // return Err(Trap::StackUnderflow);

                let addr = match instr.operands {
                    Some(v) => v,
                    None => return Err(Trap::OperandNotProvided)
                };

                if (addr as usize) > self.program.len() {
                    return Err(Trap::StackOverflow);
                }

                if (self.sp as i64 - addr <= 0) {
                    return Err(Trap::StackUnderflow);
                }

                let value = match self.stack.get(self.sp - 1 - (addr as usize)) {
                    Some(v) => *v,
                    None => todo!(),
                };

                self.push_stack(value)?;

                return  Ok(());
                
            },
            InstrKind::Swap => {
                if self.stack.len() >= 2 {
                    let op_1 = self.pop_stack()?;
                    let op_2 = self.pop_stack()?;
                    self.push_stack(op_1)?;
                    self.push_stack(op_2)?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            InstrKind::Pop => {
                if self.stack.len() >= 1 {
                    let _ = self.pop_stack()?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            InstrKind::Add  => {
                if self.stack.len() >= 2 {
                    let op_1 = self.pop_stack()?;
                    let op_2 = self.pop_stack()?;
                    self.push_stack(op_1 + op_2)?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            InstrKind::Minus  => {
                if self.stack.len() >= 2 {
                    let op_2 = self.pop_stack()?;
                    let op_1 = self.pop_stack()?;
                    self.push_stack(op_1 - op_2)?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            InstrKind::Mul  => {
                if self.stack.len() >= 2 {
                    let op_1 = self.pop_stack()?;
                    let op_2 = self.pop_stack()?;
                    self.push_stack(op_1 * op_2)?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            InstrKind::Div  => {
                if self.stack.len() >= 2 {
                    let op_2 = self.pop_stack()?;
                    let op_1 = self.pop_stack()?;
                    if op_2 == 0 {return Err(Trap::DivByZero);}
                    self.push_stack(op_1 / op_2)?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            InstrKind::Jmp => {
                let addrs = match instr.operands {
                    Some(v) => v,
                    None => todo!()
                };
                if addrs < 0 || (addrs as usize) > self.program.len() {
                    return Err(Trap::AddersOutOfBounds);
                } 
                self.ip = addrs as usize;
                Ok(())
            },
            InstrKind::JmpIf => {
                let op_1 = self.pop_stack()?;
                let addrs = match instr.operands {
                    Some(v) => v,
                    None => todo!()
                };
                if addrs < 0 || (addrs as usize) > self.program.len() {
                    return Err(Trap::AddersOutOfBounds);
                } 
                if op_1 == 1 {self.ip = addrs as usize}
                Ok(())
            },
            InstrKind::Eq => {
                if self.stack.len() >= 2 {
                    let op_1 = self.pop_stack()?;
                    let op_2 = self.pop_stack()?;
                    self.push_stack((op_1 == op_2) as i64)?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            InstrKind::Gt => {
                if self.stack.len() >= 2 {
                    let op_2 = self.pop_stack()?;
                    let op_1 = self.pop_stack()?;
                    self.push_stack((op_1 < op_2) as i64)?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            InstrKind::Gte => {
                if self.stack.len() >= 2 {
                    let op_2 = self.pop_stack()?;
                    let op_1 = self.pop_stack()?;
                    self.push_stack((op_1 <= op_2) as i64)?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            InstrKind::Lt => {
                if self.stack.len() >= 2 {
                    let op_2 = self.pop_stack()?;
                    let op_1 = self.pop_stack()?;
                    self.push_stack((op_1 > op_2) as i64)?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            InstrKind::Lte => {
                if self.stack.len() >= 2 {
                    let op_2 = self.pop_stack()?;
                    let op_1 = self.pop_stack()?;
                    self.push_stack((op_1 >= op_2) as i64)?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },

            InstrKind::Print => {
                if self.stack.len() >= 1 {
                    let value = self.pop_stack()?;
                    println!("Output: {}", value);
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            InstrKind::Debug => {
                println!("CHSVM: {:?}, SP: {}, STACK_LEN: {}", self.stack, self.sp, self.stack.len());
                return Ok(());
            },
            InstrKind::Nop => {return Ok(());}
            InstrKind::Halt => {
                self.is_halted = true;
                return Ok(());
            }
            //_ => Ok(())
        }
    }

    pub fn run(&mut self) {
        while !self.is_halted {
            match self.execute_next_instr() {
                Ok(_) => {}//{println!("Stack: {:?}", self.stack);},
                Err(e) => { eprintln!("It's a trap: {:?}", e); break; }
            }
        }
    }

    fn pop_stack(&mut self) -> Result<Word, Trap> {
        if !(self.sp == 0) {self.sp -= 1}
        match self.stack.pop() {
            Some(v) => Ok(v),
            None => Err(Trap::StackUnderflow),
        }
    }
    
    fn push_stack(&mut self, value: Word) -> Result<(), Trap> {
        if ((self.sp+1) > self.stack.capacity() ) {return Err(Trap::StackOverflow);}
        self.sp += 1;
        self.stack.push(value);
        Ok(())
    }
}


#[cfg(test)]
mod test {
    use super::CHSVM;


}