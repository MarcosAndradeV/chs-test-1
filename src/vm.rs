
const STACK_CAPACITY: usize = 1024;

type Word = i64;

#[derive(Debug, PartialEq, Eq)]
pub enum Trap {
    StackOverflow,
    StackUnderflow,
    DivByZero,
    OperandNotProvided
}

#[derive(Debug, PartialEq, Eq)]
pub enum InstrKind {
    Push,
    Add,
    Minus,
    Mul,
    Div,
}
#[derive(Debug)]
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
    sp: usize
}

impl CHSVM {
    pub fn new() -> Self {
        Self { stack: Vec::with_capacity(STACK_CAPACITY), sp: 0}
    }
    pub fn execute_instr(&mut self, instr: Instr) -> Result<(), Trap>{
        match instr.kind {
            InstrKind::Push => {
                if self.sp < self.stack.capacity() {
                    self.sp += 1;
                    let value = match instr.operands {
                        Some(v) => v,
                        None => return Err(Trap::OperandNotProvided)
                    };
                    self.stack.push(value);
                    return Ok(());
                }
                Err(Trap::StackOverflow)
            },
            InstrKind::Add  => {
                if self.stack.len() >= 2 {
                    let op_1 = match self.stack.pop() { Some(op) => op, None => return Err(Trap::StackUnderflow)  };
                    let op_2 = match self.stack.pop() { Some(op) => op, None => return Err(Trap::StackUnderflow)  };
                    self.stack.push(op_1 + op_2);
                    self.sp -= 2;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            InstrKind::Minus  => {
                if self.stack.len() >= 2 {
                    let op_1 = match self.stack.pop() { Some(op) => op, None => return Err(Trap::StackUnderflow)  };
                    let op_2 = match self.stack.pop() { Some(op) => op, None => return Err(Trap::StackUnderflow)  };
                    self.stack.push(op_1 - op_2);
                    self.sp -= 2;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            InstrKind::Mul  => {
                if self.stack.len() >= 2 {
                    let op_1 = match self.stack.pop() { Some(op) => op, None => return Err(Trap::StackUnderflow)  };
                    let op_2 = match self.stack.pop() { Some(op) => op, None => return Err(Trap::StackUnderflow)  };
                    self.stack.push(op_1 * op_2);
                    self.sp -= 2;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            InstrKind::Div  => {
                if self.stack.len() >= 2 {
                    let op_2 = match self.stack.pop() { Some(op) => op, None => return Err(Trap::StackUnderflow)  };
                    let op_1 = match self.stack.pop() { Some(op) => op, None => return Err(Trap::StackUnderflow)  };
                    if op_2 == 0 {return Err(Trap::DivByZero);}
                    self.stack.push(op_1 / op_2);
                    self.sp -= 2;
                    return Ok(());
                }
                return Err(Trap::StackUnderflow);
            },
            //_ => Ok(())
        }
    }
}


#[cfg(test)]
mod test {
    use super::CHSVM;

    #[test]
    fn execute_instr() {
        let mut vm = CHSVM::new();
        assert!(vm.execute_instr(super::Instr::new(super::InstrKind::Push, Some(1))).is_ok());
        assert!(vm.execute_instr(super::Instr::new(super::InstrKind::Push, Some(1))).is_ok());
        assert!(vm.execute_instr(super::Instr::new(super::InstrKind::Add,  None   )).is_ok());

        println!("{:?}", vm);

    }
}