#[derive(Debug, Default, Clone, Copy)]
enum Instruction {
    #[default]
    Nop,
    PushConst(i64),
    Drop(usize),

    AllocLocal(usize),
    DeallocLocal(usize),
    SetLocal(usize),
    ISetLocal(usize, i64),
    GetLocal(usize),
    RefLocal(usize),
    Deref,

    Call(isize),
    Ret,
}

#[derive(Debug, Default)]
struct VM {
    stack: Vec<i64>,
    local_stack: Vec<i64>,
    call_stack: Vec<usize>,
    program: Vec<Instruction>,
    ip: usize,
}

fn jump(addr: usize, rel: isize) -> usize {
    (addr as isize + rel) as usize
}
fn jump_to(addr: usize, other: usize) -> isize {
    other as isize - addr as isize
}

impl VM {
    #[inline]
    fn exec_instr(&mut self, instr: Instruction) {
        dbg!(&self);
        match instr {
            Instruction::Nop => {}
            Instruction::PushConst(con) => self.stack.push(con),
            Instruction::Drop(n) => self.stack.truncate(self.stack.len() - n),
            Instruction::AllocLocal(n) => {
                self.local_stack.resize(self.local_stack.len() + n, 0);
            }
            Instruction::DeallocLocal(n) => {
                self.local_stack.truncate(self.local_stack.len() - n);
            }
            Instruction::SetLocal(offset) => {
                let a = self.stack.pop().unwrap();
                let n = self.local_stack.len() - 1 - offset;
                if let Some(b) = self.local_stack.get_mut(n) {
                    *b = a;
                }
            }
            Instruction::ISetLocal(offset, con) => {
                let n = self.local_stack.len() - 1 - offset;
                if let Some(b) = self.local_stack.get_mut(n) {
                    *b = con;
                }
            }
            Instruction::GetLocal(offset) => {
                let n = self.local_stack.len() - 1 - offset;
                if let Some(b) = self.local_stack.get_mut(n) {
                    self.stack.push(*b);
                }
            }
            Instruction::RefLocal(offset) => {
                self.stack.push(offset as i64);
            }
            Instruction::Deref => {
                let a = self.stack.last_mut().unwrap();
                if let Some(b) = self.local_stack.get(*a as usize) {
                    *a = *b;
                }
            }
            Instruction::Call(n) => {
                self.call_stack.push(self.ip);
                self.ip = jump(self.ip, n);
            },
            Instruction::Ret => {
                if let Some(addr) = self.call_stack.pop() {
                    self.ip = addr;
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vm() {
        let mut vm = VM::default();
        vm.exec_instr(Instruction::AllocLocal(1));
        vm.exec_instr(Instruction::ISetLocal(0, 3));
        vm.exec_instr(Instruction::GetLocal(0));
        vm.exec_instr(Instruction::DeallocLocal(1));
        assert!(vm.stack.last().is_some_and(|a| *a == 3))
    }
}
