use crate::{exepitons::Trap, instructions::{Instr, Opcode}, value::CHSValue};



const STACK_CAPACITY: usize = 1024;



#[derive(Debug)]
pub struct CHSVM {
    data_stack: Vec<CHSValue>,
    return_stack: Vec<CHSValue>,
    is_halted: bool,
    ip: usize,
    sp: usize,
    program: Vec<Instr>,

}

impl CHSVM {
    pub fn new(program: Vec<Instr>) -> Self {
        Self {
            data_stack: Vec::with_capacity(STACK_CAPACITY),
            return_stack: Vec::with_capacity(STACK_CAPACITY),
            sp: 0,
            ip: 0,
            is_halted: false,
            program
        }
    }
    pub fn execute_next_instr(&mut self) -> Result<(), Trap>{
        let instr = self.decode_instr()?;
        match instr.opcode {
            Opcode::Pushi => {
                let value = match instr.operands {
                    CHSValue::I(v) => instr.operands,
                    CHSValue::None => return Err(Trap::OperandNotProvided),
                    _ => return Err(Trap::OperandTypeNotCorrect)
                };
                self.push_stack(value)
            },
            Opcode::Pushf => {
                let value = match instr.operands {
                    CHSValue::F(v) => instr.operands,
                    CHSValue::None => return Err(Trap::OperandNotProvided),
                    _ => return Err(Trap::OperandTypeNotCorrect)
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

                if (self.sp as i64  - 1 - addrs.as_i64()) < 0 {
                    return Err(Trap::AddersOutOfBounds);
                }

                let value = match self.data_stack.get(self.sp - 1 - addrs.as_usize()) {
                    Some(v) => *v,
                    None => return Err(Trap::StackUnderflow),
                };

                self.push_stack(value)?;

                return  Ok(());
                
            },
            Opcode::Swap => {
                if self.data_stack.len() >= 2 {
                    let op_1 = self.pop_stack()?;
                    let op_2 = self.pop_stack()?;
                    self.push_stack(op_1)?;
                    self.push_stack(op_2)?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            Opcode::Pop => {
                if self.data_stack.len() >= 1 {
                    let _ = self.pop_stack()?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            Opcode::Bind => {
                let q = match instr.operands {
                    v => v,
                    CHSValue::None => return Err(Trap::OperandNotProvided)
                };
                if q.as_usize() <= self.data_stack.len() {
                    for _ in 0..q.as_usize() {
                        let value = self.pop_stack()?;
                        self.return_stack.push(value);
                    }
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            Opcode::BindPush => {
                let q = match instr.operands {
                    v => v,
                    CHSValue::None => return Err(Trap::OperandNotProvided)
                };
                if q.as_usize() > self.data_stack.len() {return  Err(Trap::StackOverflow);}
                let local = match self.return_stack.get(q.as_usize()) {
                    Some(v) => *v,
                    None => return Err(Trap::StackOverflow)
                };
                self.push_stack(local)?;
                return  Ok(());
            },
            Opcode::Unbind => {
                self.return_stack.clear();
                return Ok(());
            },
            Opcode::Add  => {
                if self.data_stack.len() >= 2 {
                    let op_1 = self.pop_stack()?;
                    let op_2 = self.pop_stack()?;
                    self.push_stack(op_1 + op_2)?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            Opcode::Minus  => {
                if self.data_stack.len() >= 2 {
                    let op_2 = self.pop_stack()?;
                    let op_1 = self.pop_stack()?;
                    self.push_stack(op_1 - op_2)?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            Opcode::Mul  => {
                if self.data_stack.len() >= 2 {
                    let op_1 = self.pop_stack()?;
                    let op_2 = self.pop_stack()?;
                    self.push_stack(op_1 * op_2)?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            Opcode::Div  => {
                if self.data_stack.len() >= 2 {
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
                if op_1.as_usize() == 1 {return Ok(());}
                let addrs = match instr.operands {
                    v => v,
                    CHSValue::None => return Err(Trap::OperandNotProvided)
                };
                if addrs.as_usize() > self.program.len() {
                    return Err(Trap::AddersOutOfBounds);
                } 
                self.ip += addrs.as_usize();
                Ok(())
            },
            Opcode::Eq => {
                if self.data_stack.len() >= 2 {
                    let op_1 = self.pop_stack()?;
                    let op_2 = self.pop_stack()?;
                    self.push_stack(CHSValue::B((op_1 == op_2) as u8))?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            Opcode::Gt => {
                if self.data_stack.len() >= 2 {
                    let op_2 = self.pop_stack()?;
                    let op_1 = self.pop_stack()?;
                    self.push_stack(CHSValue::B((op_1 < op_2) as u8))?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            Opcode::Gte => {
                if self.data_stack.len() >= 2 {
                    let op_2 = self.pop_stack()?;
                    let op_1 = self.pop_stack()?;
                    self.push_stack(CHSValue::B((op_1 <= op_2) as u8))?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            Opcode::Lt => {
                if self.data_stack.len() >= 2 {
                    let op_2 = self.pop_stack()?;
                    let op_1 = self.pop_stack()?;
                    self.push_stack(CHSValue::B((op_1 > op_2) as u8))?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            Opcode::Lte => {
                if self.data_stack.len() >= 2 {
                    let op_2 = self.pop_stack()?;
                    let op_1 = self.pop_stack()?;
                    self.push_stack(CHSValue::B((op_1 >= op_2) as u8))?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },

            Opcode::While => {
                if self.data_stack.len() >= 1 {
                    let t_op = self.pop_stack()?;
                    self.return_stack.push(t_op);
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },

            Opcode::Print => {
                if self.data_stack.len() >= 1 {
                    let value = self.pop_stack()?;
                    println!("{}", value);
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            Opcode::Debug => {
                println!("CHSVM: {:?}, SP: {}, STACK_LEN: {}", self.data_stack, self.sp, self.data_stack.len());
                println!("Local: {:?}, STACK_LEN: {}", self.return_stack, self.return_stack.len());
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
                Ok(_) => {} //{println!("Stack: {:?}", self.data_stack);},
                Err(e) => { eprintln!("It's a trap: {:?}", e); break; }
            }
        }
    }

    fn pop_stack(&mut self) -> Result<CHSValue, Trap> {
        if !(self.sp == 0) {self.sp -= 1}
        match self.data_stack.pop() {
            Some(v) => Ok(v),
            None => Err(Trap::StackUnderflow),
        }
    }
    
    fn push_stack(&mut self, value: CHSValue) -> Result<(), Trap> {
        if ((self.sp+1) > self.data_stack.capacity() ) {return Err(Trap::StackOverflow);}
        self.sp += 1;
        self.data_stack.push(value);
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

