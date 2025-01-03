use bytecode::Instruction;
use stack::MainStack;

pub mod bytecode;
mod stack;

pub struct VM {
    stack: MainStack,
    funcs: MainStack,
    ip: usize,
    program: Vec<Instruction>
}

impl VM {
    pub fn new(program: Vec<Instruction>) -> Self {
        Self {
            stack: MainStack::new(1024),
            funcs: MainStack::new(1024),
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
                Some(Instruction::PushLocal(value)) => {
                    if let Some(value) = self.stack.get(*value) {
                        self.stack.push(*value);
                    } else {
                        panic!("Local not found!")
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
                Some(Instruction::Ret) => {
                    if let Some(n) = self.funcs.pop() {
                        self.ip = n as usize;
                    } else {
                        break; // End of the program
                    }
                }
                Some(Instruction::Call(n)) => {
                    self.funcs.push(self.ip as i64 + 1);
                    self.ip = *n;
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

    #[test]
    fn test_vm_3() {
        let mut vm = VM::new(vec![
            Instruction::PushConst(10),
            Instruction::PushLocal(0),
            Instruction::PushConst(20),
            Instruction::Add,
            Instruction::SetLocal(0),
            Instruction::Halt
        ]);
        vm.run();
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
        vm.run();
        assert!(vm.stack.pop().is_some_and(|a| a == 30))
    }
}
