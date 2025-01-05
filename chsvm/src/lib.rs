use bytecode::Instruction;
use chs_util::{chs_error, CHSError, CHSResult};
use memory::stack::Stack;

pub mod bytecode;
pub mod memory;

pub enum VMError {
    LocalNotFound,
    StackUnderflow,
    StackOverflow,
    PCOutOfBounds,
    DivByZero,
}

impl ToString for VMError {
    fn to_string(&self) -> String {
        match self {
            VMError::LocalNotFound => "Local not found".to_string(),
            VMError::StackUnderflow => "Stack underflow".to_string(),
            VMError::StackOverflow => "Stack overflow".to_string(),
            VMError::PCOutOfBounds => "Program counter out of bounds".to_string(),
            VMError::DivByZero => "Division by zero".to_string(),
        }
    }
}

pub struct VM {
    stack: Stack<i64>,
    funcs: Stack<usize>,
    ip: usize,
    program: Vec<Instruction>,
}

impl VM {
    pub fn new(program: Vec<Instruction>) -> Self {
        Self {
            stack: Stack::with_capacity(1024),
            funcs: Stack::with_capacity(1024),
            program,
            ip: 0,
        }
    }
    pub fn run(&mut self) -> CHSResult<()> {
        loop {
            match self.program.get(self.ip) {
                Some(Instruction::PushConst(value)) => {
                    self.stack.push(*value);
                    self.ip += 1;
                }
                Some(Instruction::PushLocal(value)) => {
                    if let Some(value) = self.stack.get(*value) {
                        self.stack.push(*value);
                    } else {
                        chs_error!(VMError::LocalNotFound)
                    }
                    self.ip += 1;
                }
                Some(Instruction::SetLocal(addr)) => {
                    let value = self.stack.pop().unwrap();
                    let addr = self.stack.get_mut(*addr).unwrap();
                    *addr = value;

                    self.ip += 1;
                }
                Some(Instruction::Drop(value)) => {
                    self.stack.drop(*value);
                    self.ip += 1;
                }
                Some(Instruction::Add) => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        self.stack.push(a + b);
                    } else {
                        chs_error!(VMError::StackUnderflow);
                    }
                    self.ip += 1;
                }
                Some(Instruction::Sub) => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        self.stack.push(a - b);
                    } else {
                        chs_error!(VMError::StackUnderflow);
                    }
                    self.ip += 1;
                }
                Some(Instruction::Mult) => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        self.stack.push(a * b);
                    } else {
                        chs_error!(VMError::StackUnderflow);
                    }
                    self.ip += 1;
                }
                Some(Instruction::Div) => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        if b == 0 {
                            chs_error!(VMError::DivByZero);
                        }
                        self.stack.push(a / b);
                    } else {
                        chs_error!(VMError::StackUnderflow);
                    }
                    self.ip += 1;
                }
                Some(Instruction::Eq) => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        self.stack.push((a == b) as i64);
                    } else {
                        chs_error!(VMError::StackUnderflow);
                    }
                    self.ip += 1;
                }
                Some(Instruction::NotEq) => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        self.stack.push((a != b) as i64);
                    } else {
                        chs_error!(VMError::StackUnderflow);
                    }
                    self.ip += 1;
                }
                Some(Instruction::Lt) => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        self.stack.push((a < b) as i64);
                    } else {
                        chs_error!(VMError::StackUnderflow);
                    }
                    self.ip += 1;
                }
                Some(Instruction::Halt) => {
                    break Ok(());
                }
                Some(Instruction::Jmp(n)) => {
                    if *n > self.program.len() {
                        chs_error!(VMError::PCOutOfBounds);
                    }
                    self.ip = *n;
                }
                Some(Instruction::JmpIf(n)) => {
                    if *n > self.program.len() {
                        chs_error!(VMError::PCOutOfBounds);
                    }
                    if let Some(a) = self.stack.pop() {
                        if a == 1 {
                            self.ip = *n
                        } else {
                            self.ip += 1;
                        }
                    } else {
                        chs_error!(VMError::StackUnderflow);
                    }
                }
                Some(Instruction::Ret) => {
                    if let Some(n) = self.funcs.pop() {
                        self.ip = n as usize;
                    } else {
                        chs_error!("???")
                    }
                }
                Some(Instruction::Call(n)) => {
                    self.funcs.push(self.ip + 1);
                    if *n > self.program.len() {
                        chs_error!(VMError::PCOutOfBounds);
                    }
                    self.ip = *n;
                }
                Some(Instruction::Print) => {
                    if let Some(a) = self.stack.pop() {
                        dbg!(a);
                        self.ip += 1;
                    } else {
                        chs_error!(VMError::StackUnderflow);
                    }
                }
                None => {
                    chs_error!(VMError::PCOutOfBounds);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vm_1() {
        let mut vm = VM::new(vec![Instruction::Halt]);
        assert!(vm.run().is_ok());
        assert!(vm.stack.pop().is_none())
    }

    #[test]
    fn test_vm_2() {
        let mut vm = VM::new(vec![
            Instruction::PushConst(10),
            Instruction::PushConst(20),
            Instruction::Add,
            Instruction::PushConst(30),
            Instruction::Sub,
            Instruction::Drop(1),
            Instruction::Halt,
        ]);
        assert!(vm.run().is_ok());
        assert!(vm.stack.pop().is_none())
    }

    #[test]
    fn test_vm_3() {
        let mut vm = VM::new(vec![
            Instruction::PushConst(10),
            Instruction::PushLocal(0),
            Instruction::PushConst(20),
            Instruction::Add,
            Instruction::SetLocal(0),
            Instruction::Halt,
        ]);
        assert!(vm.run().is_ok());
        assert!(vm.stack.pop().is_some_and(|a| a == 30))
    }

    #[test]
    fn test_vm_4() {
        let mut vm = VM::new(vec![
            Instruction::PushConst(10),
            Instruction::PushConst(20),
            Instruction::PushLocal(0),
            Instruction::PushLocal(1),
            Instruction::Call(6),
            Instruction::Halt,
            Instruction::Add,
            Instruction::Ret,
        ]);
        assert!(vm.run().is_ok());
        assert!(vm.stack.pop().is_some_and(|a| a == 30))
    }

    #[test]
    fn test_vm_5() {
        let mut vm = VM::new(vec![
            Instruction::Jmp(2),
            Instruction::PushConst(10),
            Instruction::Halt,
        ]);
        assert!(vm.run().is_ok());
        assert!(vm.stack.pop().is_none())
    }

    #[test]
    fn test_vm_6() {
        let mut vm = VM::new(vec![
            Instruction::PushConst(1),
            Instruction::PushConst(2),
            Instruction::Add,
            Instruction::PushConst(3),
            Instruction::Eq,
            Instruction::JmpIf(7),
            Instruction::PushConst(10),
            Instruction::Halt,
        ]);
        assert!(vm.run().is_ok());
        assert!(vm.stack.pop().is_none())
    }
}
