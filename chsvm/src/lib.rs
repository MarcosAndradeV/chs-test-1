#[derive(Debug, Default, Clone, Copy)]
pub enum Instruction {
    #[default]
    Halt,
    PushConst(i64),
    Drop(usize),
    Add,
    Sub,
}

pub struct MainStack {
    inner: Vec<i64>,
}

impl MainStack {
    pub fn new(size: usize) -> Self {
        Self {
            inner: Vec::with_capacity(size),
        }
    }
    pub fn push(&mut self, value: i64) {
        self.inner.push(value);
    }
    pub fn pop(&mut self) -> Option<i64> {
        self.inner.pop()
    }
    pub fn drop(&mut self, n: usize) {
        let n = self.inner.len() - n;
        self.inner.truncate(n);
    }
}

pub struct VM {
    stack: MainStack,
    ip: usize,
    program: Vec<Instruction>
}

impl VM {
    pub fn new(program: Vec<Instruction>) -> Self {
        Self {
            stack: MainStack::new(1024),
            program,
            ip: 0,
        }
    }
    pub fn run(&mut self) {
        loop {
            match self.program.get(self.ip) {
                Some(Instruction::PushConst(value)) => {
                    self.stack.push(*value);
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
                        panic!("Stack underflow during Add");
                    }
                    self.ip += 1;
                }
                Some(Instruction::Sub) => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        self.stack.push(a - b);
                    } else {
                        panic!("Stack underflow during Sub");
                    }
                    self.ip += 1;
                }
                Some(Instruction::Halt) => {
                    break;
                }
                None => panic!("Program counter out of bounds"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vm_1() {
        let mut vm = VM::new(vec![
            Instruction::Halt
        ]);
        vm.run();
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
            Instruction::Halt
        ]);
        vm.run();
        assert!(vm.stack.pop().is_none())
    }
}
