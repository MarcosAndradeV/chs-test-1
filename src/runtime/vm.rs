use std::io::Write;

use crate::{
    exepitons::Trap,
    bytecode::instructions::{Instr, Opcode},
    bytecode::{value::CHSValue, ByteCode},
};

const STACK_CAPACITY: usize = 1024;
const MEMORY_CAPACITY: usize = 60000;

#[derive(Debug)]
pub struct CHSVM {
    data_stack: Vec<CHSValue>,
    return_stack: Vec<CHSValue>,
    memory: [u8; MEMORY_CAPACITY],
    is_halted: bool,
    ip: usize,
    sp: usize,
    program: ByteCode,
}

impl CHSVM {
    pub fn new(program: ByteCode) -> Self {
        Self {
            data_stack: Vec::with_capacity(STACK_CAPACITY),
            return_stack: Vec::with_capacity(STACK_CAPACITY),
            memory: [0; MEMORY_CAPACITY],
            sp: 0,
            ip: 0,
            is_halted: false,
            program,
        }
    }
    pub fn execute_next_instr(&mut self) -> Result<(), Trap> {
        let instr = self.decode_instr()?;
        match instr.opcode {
            Opcode::Pushi => {
                let value = match instr.operands {
                    CHSValue::I(v) => instr.operands,
                    CHSValue::P(v) => instr.operands,
                    CHSValue::None => return Err(Trap::OperandNotProvided),
                    _ => return Err(Trap::OperandTypeNotCorrect),
                };
                self.push_stack(value)
            }
            Opcode::Pushf => {
                let value = match instr.operands {
                    CHSValue::F(v) => instr.operands,
                    CHSValue::None => return Err(Trap::OperandNotProvided),
                    _ => return Err(Trap::OperandTypeNotCorrect),
                };
                self.push_stack(value)
            }
            Opcode::Over => {
                let addrs = match instr.operands {
                    v => v,
                    CHSValue::None => return Err(Trap::OperandNotProvided),
                };

                if addrs.as_usize() > self.program.code.len() {
                    return Err(Trap::StackOverflow);
                }

                if (self.sp as i64 - 1 - addrs.as_i64()) < 0 {
                    return Err(Trap::AddersOutOfBounds);
                }

                let value = match self.data_stack.get(self.sp - 1 - addrs.as_usize()) {
                    Some(v) => *v,
                    None => return Err(Trap::StackUnderflow),
                };

                self.push_stack(value)?;

                return Ok(());
            }
            Opcode::Dup => {
                if self.data_stack.len() >= 1 {
                    let value = match self.data_stack.get(0) {
                        Some(v) => *v,
                        None => return Err(Trap::StackUnderflow),
                    };

                    self.push_stack(value)?;

                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            }
            Opcode::Swap => {
                if self.data_stack.len() >= 2 {
                    let op_1 = self.pop_stack()?;
                    let op_2 = self.pop_stack()?;
                    self.push_stack(op_1)?;
                    self.push_stack(op_2)?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            }
            Opcode::Pop => {
                if self.data_stack.len() >= 1 {
                    let _ = self.pop_stack()?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            }
            Opcode::Bind => {
                let q = match instr.operands {
                    v => v,
                    CHSValue::None => return Err(Trap::OperandNotProvided),
                };
                if q.as_usize() <= self.data_stack.len() {
                    for _ in 0..q.as_usize() {
                        let value = self.pop_stack()?;
                        self.return_stack.push(value);
                    }
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            }
            Opcode::BindPush => {
                let q = match instr.operands {
                    v => v,
                    CHSValue::None => return Err(Trap::OperandNotProvided),
                };
                if q.as_usize() > self.data_stack.len() {
                    return Err(Trap::StackOverflow);
                }
                let local = match self.return_stack.get(q.as_usize()) {
                    Some(v) => *v,
                    None => return Err(Trap::StackOverflow),
                };
                self.push_stack(local)?;
                return Ok(());
            }
            Opcode::Unbind => {
                let q = match instr.operands {
                    v => v,
                    CHSValue::None => return Err(Trap::OperandNotProvided),
                };
                self.return_stack.truncate(self.return_stack.len()-q.as_usize());
                return Ok(());
            }
            Opcode::Store => {
                if self.data_stack.len() >= 2 {
                    let op_2 = self.pop_stack()?; // value
                    let op_1 = self.pop_stack()?; // ptr
                    if op_1.as_usize() > MEMORY_CAPACITY {return Err(Trap::StackUnderflow);}
                    self.store(op_1, op_2)?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            Opcode::Load => {
                if self.data_stack.len() >= 1 {
                    let op_1 = self.pop_stack()?; // ptr
                    if op_1.as_usize() > MEMORY_CAPACITY {return Err(Trap::StackUnderflow);}
                    self.load(op_1)?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            Opcode::Add => {
                if self.data_stack.len() >= 2 {
                    let op_2 = self.pop_stack()?;
                    let op_1 = self.pop_stack()?;
                    self.push_stack(op_1 + op_2)?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            }
            Opcode::Minus => {
                if self.data_stack.len() >= 2 {
                    let op_2 = self.pop_stack()?;
                    let op_1 = self.pop_stack()?;
                    self.push_stack(op_1 - op_2)?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            }
            Opcode::Mul => {
                if self.data_stack.len() >= 2 {
                    let op_2 = self.pop_stack()?;
                    let op_1 = self.pop_stack()?;
                    self.push_stack(op_1 * op_2)?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            }
            Opcode::Lgor => {
                if self.data_stack.len() >= 2 {
                    let op_2 = self.pop_stack()?;
                    let op_1 = self.pop_stack()?;
                    self.push_stack(CHSValue::B((op_1.as_bool() || op_2.as_bool()) as u8))?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            }
            Opcode::Div => {
                if self.data_stack.len() >= 2 {
                    let op_2 = self.pop_stack()?;
                    let op_1 = self.pop_stack()?;
                    if op_2.is_zero() {
                        return Err(Trap::DivByZero);
                    }
                    self.push_stack(op_1 / op_2)?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            }
            Opcode::Mod => {
                if self.data_stack.len() >= 2 {
                    let op_2 = self.pop_stack()?;
                    let op_1 = self.pop_stack()?;
                    if op_2.is_zero() {
                        return Err(Trap::DivByZero);
                    }
                    self.push_stack(op_1 % op_2)?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            }
            Opcode::Inc => {
                if self.data_stack.len() >= 1 {
                    let v = self.pop_stack()?;
                    self.push_stack( CHSValue::I(v.as_i64()+1) );
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            }
            Opcode::Jmp => {
                let addrs = match instr.operands {
                    v => v,
                    CHSValue::None => return Err(Trap::OperandNotProvided),
                };
                if addrs.as_usize() > self.program.code.len() {
                    return Err(Trap::AddersOutOfBounds);
                }
                self.ip = addrs.as_usize();
                Ok(())
            }
            Opcode::JmpIf => {
                let op_1 = self.pop_stack()?;
                if op_1.as_usize() == 1 {
                    return Ok(());
                }
                let addrs = match instr.operands {
                    v => v,
                    CHSValue::None => return Err(Trap::OperandNotProvided),
                };
                if addrs.as_usize() > self.program.code.len() {
                    return Err(Trap::AddersOutOfBounds);
                }
                self.ip = addrs.as_usize();
                Ok(())
            },
            Opcode::JmpWhile => {
                let addrs = match self.return_stack.last() {
                    Some(v) => *v,
                    None => return Err(Trap::OperandNotProvided)
                };
                if addrs.as_usize() > self.program.code.len() {
                    return Err(Trap::AddersOutOfBounds);
                }
                self.ip = addrs.as_usize();
                return Ok(());
            },
            Opcode::End => {
                self.ip += 1;
                Ok(())
            }
            Opcode::Eq => {
                if self.data_stack.len() >= 2 {
                    let op_2 = self.pop_stack()?;
                    let op_1 = self.pop_stack()?;
                    self.push_stack(CHSValue::B((op_1 == op_2) as u8))?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            }
            Opcode::Gt => {
                if self.data_stack.len() >= 2 {
                    let op_2 = self.pop_stack()?;
                    let op_1 = self.pop_stack()?;
                    self.push_stack(CHSValue::B((op_1 < op_2) as u8))?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            }
            Opcode::Gte => {
                if self.data_stack.len() >= 2 {
                    let op_2 = self.pop_stack()?;
                    let op_1 = self.pop_stack()?;
                    self.push_stack(CHSValue::B((op_1 <= op_2) as u8))?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            }
            Opcode::Lt => {
                if self.data_stack.len() >= 2 {
                    let op_2 = self.pop_stack()?;
                    let op_1 = self.pop_stack()?;
                    self.push_stack(CHSValue::B((op_1 > op_2) as u8))?;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            }
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
                self.return_stack.push(CHSValue::P(self.ip));
                return Ok(());
            },

            Opcode::Call => {
                self.return_stack.push(CHSValue::P(self.ip));
                if self.data_stack.len() >= 1 {
                    let value = self.pop_stack()?;
                    self.ip = value.as_usize();
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            Opcode::Ret => {
                let addrs = match self.return_stack.pop() {
                    Some(v) => v,
                    None => return Err(Trap::OperandNotProvided)
                };
                if addrs.as_usize() > self.program.code.len() {
                    return Err(Trap::AddersOutOfBounds);
                }
                self.ip = addrs.as_usize();
                return Ok(());
            },
            Opcode::PreProc => {
                Ok(())
            },
            Opcode::Write => {
                if self.data_stack.len() >= 2 {
                    let op_2 = self.pop_stack()?;
                    let op_1 = self.pop_stack()?;
                    let mut w = String::new();
                    for i in op_1.as_usize()..=op_2.as_usize() {
                        w.push(self.memory[i] as char);
                    }
                    println!("{}", w);
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            }
            Opcode::Print => {
                if self.data_stack.len() >= 1 {
                    let value = self.pop_stack()?;
                    println!("{}", value);
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            }
            Opcode::Debug => {
                println!(
                    "CHSVM: {:?}, SP: {}, STACK_LEN: {}",
                    self.data_stack,
                    self.sp,
                    self.data_stack.len()
                );
                println!(
                    "Local: {:?}, STACK_LEN: {}",
                    self.return_stack,
                    self.return_stack.len()
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
        // for i in self.program.code.iter().enumerate() {
        //     println!("{} -> {:?}", i.0, i.1);
        // }
        while !self.is_halted {
            match self.execute_next_instr() {
                Ok(_) => {} // {println!("{:?}\n{:?}\n{:?}\n{:?}\n", self.data_stack, self.return_stack, self.ip, self.sp)}
                Err(e) => {
                    eprintln!("It's a trap: {:?} at {}", e, self.ip);
                    break;
                }
            }
        }
    }

    fn pop_stack(&mut self) -> Result<CHSValue, Trap> {
        if !(self.sp == 0) {
            self.sp -= 1
        }
        match self.data_stack.pop() {
            Some(v) => Ok(v),
            None => Err(Trap::StackUnderflow),
        }
    }

    fn push_stack(&mut self, value: CHSValue) -> Result<(), Trap> {
        if ((self.sp + 1) > self.data_stack.capacity()) {
            return Err(Trap::StackOverflow);
        }
        self.sp += 1;
        self.data_stack.push(value);
        Ok(())
    }

    fn decode_instr(&mut self) -> Result<Instr, Trap> {
        self.ip += 1;
        if self.ip > self.program.code.len() {
            return Err(Trap::ProgramEndWithoutHalt);
        }
        //println!("{:?}", self.program[self.ip - 1]);
        //println!("DATA: {:?}", self.data_stack);
        Ok(self.program.code[self.ip - 1])
    }

    fn store(&mut self, op_1: CHSValue, op_2: CHSValue) -> Result<(), Trap> {
        self.memory[op_1.as_usize()] = op_2.as_u8();
        Ok(())
    }
    fn load(&mut self, op_1: CHSValue) -> Result<(), Trap> {
        self.push_stack(CHSValue::I(self.memory[op_1.as_usize()] as i64))?;
        Ok(())
    }
}
