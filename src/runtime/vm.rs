use crate::{exepitons::Trap, instructions::{Instr, InstrKind}, value::CHSValue};



const STACK_CAPACITY: usize = 1024;


#[derive(Debug)]
pub struct CHSVM {
    pub stack: Vec<CHSValue>,
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
                let addrs = match instr.operands {
                    Some(v) => v,
                    None => return Err(Trap::OperandNotProvided)
                };

                if Into::<usize>::into(addrs) > self.program.len() {
                    return Err(Trap::StackOverflow);
                }

                let value = match self.stack.get(self.sp - 1 - Into::<usize>::into(addrs)) {
                    Some(v) => *v,
                    None => return Err(Trap::StackUnderflow),
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
                    if op_2.is_zero() {return Err(Trap::DivByZero);}
                    self.push_stack(op_1 / op_2)?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            InstrKind::Jmp => {
                let addrs = match instr.operands {
                    Some(v) => v,
                    None => return Err(Trap::OperandNotProvided),
                };
                if Into::<usize>::into(addrs) > self.program.len() {
                    return Err(Trap::AddersOutOfBounds);
                } 
                self.ip = addrs.into();
                Ok(())
            },
            InstrKind::JmpIf => {
                let op_1 = self.pop_stack()?;
                if Into::<usize>::into(op_1) != 1 {return Ok(());}
                let addrs = match instr.operands {
                    Some(v) => v,
                    None => return Err(Trap::OperandNotProvided),
                };
                if Into::<usize>::into(addrs) > self.program.len() {
                    return Err(Trap::AddersOutOfBounds);
                } 
                self.ip = Into::<usize>::into(addrs);
                Ok(())
            },
            InstrKind::Eq => {
                if self.stack.len() >= 2 {
                    let op_1 = self.pop_stack()?;
                    let op_2 = self.pop_stack()?;
                    self.push_stack(CHSValue::B((op_1 == op_2) as u8))?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            InstrKind::Gt => {
                if self.stack.len() >= 2 {
                    let op_2 = self.pop_stack()?;
                    let op_1 = self.pop_stack()?;
                    self.push_stack(CHSValue::B((op_1 < op_2) as u8))?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            InstrKind::Gte => {
                if self.stack.len() >= 2 {
                    let op_2 = self.pop_stack()?;
                    let op_1 = self.pop_stack()?;
                    self.push_stack(CHSValue::B((op_1 <= op_2) as u8))?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            InstrKind::Lt => {
                if self.stack.len() >= 2 {
                    let op_2 = self.pop_stack()?;
                    let op_1 = self.pop_stack()?;
                    self.push_stack(CHSValue::B((op_1 > op_2) as u8))?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            InstrKind::Lte => {
                if self.stack.len() >= 2 {
                    let op_2 = self.pop_stack()?;
                    let op_1 = self.pop_stack()?;
                    self.push_stack(CHSValue::B((op_1 >= op_2) as u8))?;
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
                Ok(_) => {}     //{println!("Stack: {:?}", self.stack);},
                Err(e) => { eprintln!("It's a trap: {:?}", e); break; }
            }
        }
    }

    fn pop_stack(&mut self) -> Result<CHSValue, Trap> {
        if !(self.sp == 0) {self.sp -= 1}
        match self.stack.pop() {
            Some(v) => Ok(v),
            None => Err(Trap::StackUnderflow),
        }
    }
    
    fn push_stack(&mut self, value: CHSValue) -> Result<(), Trap> {
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