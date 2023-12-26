use crate::{exepitons::Trap, instructions::{Instr, Opcode}, value::CHSValue};



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
        let instr = self.decode_instr()?;
        match instr.opcode {
            Opcode::Push => {
                let value = match instr.operands {
                    v => v,
                    CHSValue::None => return Err(Trap::OperandNotProvided)
                };
                self.push_stack(value)
            },
            Opcode::Dup => {
                let addrs = match instr.operands {
                    v => v,
                    CHSValue::None => return Err(Trap::OperandNotProvided)
                };

                if addrs.as_usize() > self.program.len() {
                    return Err(Trap::StackOverflow);
                }

                let value = match self.stack.get(self.sp - 1 - addrs.as_usize()) {
                    Some(v) => *v,
                    None => return Err(Trap::StackUnderflow),
                };

                self.push_stack(value)?;

                return  Ok(());
                
            },
            Opcode::Swap => {
                if self.stack.len() >= 2 {
                    let op_1 = self.pop_stack()?;
                    let op_2 = self.pop_stack()?;
                    self.push_stack(op_1)?;
                    self.push_stack(op_2)?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            Opcode::Pop => {
                if self.stack.len() >= 1 {
                    let _ = self.pop_stack()?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            Opcode::Add  => {
                if self.stack.len() >= 2 {
                    let op_1 = self.pop_stack()?;
                    let op_2 = self.pop_stack()?;
                    self.push_stack(op_1 + op_2)?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            Opcode::Minus  => {
                if self.stack.len() >= 2 {
                    let op_2 = self.pop_stack()?;
                    let op_1 = self.pop_stack()?;
                    self.push_stack(op_1 - op_2)?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            Opcode::Mul  => {
                if self.stack.len() >= 2 {
                    let op_1 = self.pop_stack()?;
                    let op_2 = self.pop_stack()?;
                    self.push_stack(op_1 * op_2)?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            Opcode::Div  => {
                if self.stack.len() >= 2 {
                    let op_2 = self.pop_stack()?;
                    let op_1 = self.pop_stack()?;
                    if op_2.is_zero() {return Err(Trap::DivByZero);}
                    self.push_stack(op_1 / op_2)?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            Opcode::Jmp => {
                let addrs = match instr.operands {
                    v => v,
                    CHSValue::None => return Err(Trap::OperandNotProvided)
                };
                if addrs.as_usize() > self.program.len() {
                    return Err(Trap::AddersOutOfBounds);
                } 
                self.ip = addrs.as_usize();
                Ok(())
            },
            Opcode::JmpIf => {
                let op_1 = self.pop_stack()?;
                if op_1.as_usize() != 1 {return Ok(());}
                let addrs = match instr.operands {
                    v => v,
                    CHSValue::None => return Err(Trap::OperandNotProvided)
                };
                if addrs.as_usize() > self.program.len() {
                    return Err(Trap::AddersOutOfBounds);
                } 
                self.ip = addrs.as_usize();
                Ok(())
            },
            Opcode::Eq => {
                if self.stack.len() >= 2 {
                    let op_1 = self.pop_stack()?;
                    let op_2 = self.pop_stack()?;
                    self.push_stack(CHSValue::B((op_1 == op_2) as u8))?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            Opcode::Gt => {
                if self.stack.len() >= 2 {
                    let op_2 = self.pop_stack()?;
                    let op_1 = self.pop_stack()?;
                    self.push_stack(CHSValue::B((op_1 < op_2) as u8))?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            Opcode::Gte => {
                if self.stack.len() >= 2 {
                    let op_2 = self.pop_stack()?;
                    let op_1 = self.pop_stack()?;
                    self.push_stack(CHSValue::B((op_1 <= op_2) as u8))?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            Opcode::Lt => {
                if self.stack.len() >= 2 {
                    let op_2 = self.pop_stack()?;
                    let op_1 = self.pop_stack()?;
                    self.push_stack(CHSValue::B((op_1 > op_2) as u8))?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            Opcode::Lte => {
                if self.stack.len() >= 2 {
                    let op_2 = self.pop_stack()?;
                    let op_1 = self.pop_stack()?;
                    self.push_stack(CHSValue::B((op_1 >= op_2) as u8))?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },

            Opcode::Print => {
                if self.stack.len() >= 1 {
                    let value = self.pop_stack()?;
                    println!("Output: {}", value);
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            Opcode::Debug => {
                println!("CHSVM: {:?}, SP: {}, STACK_LEN: {}", self.stack, self.sp, self.stack.len());
                return Ok(());
            },
            Opcode::Nop => {return Ok(());}
            Opcode::Halt => {
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

    fn decode_instr(&mut self) -> Result<Instr, Trap> {
        self.ip+=1;
        if self.ip > self.program.len() {
            return Err(Trap::ProgramEndWithoutHalt);
        }
        Ok(self.program[self.ip-1])

    }
}

