use std::{io::Write, rc::Rc};

use crate::{
    exepitons::Trap,
    bytecode::instructions::{Instr, Opcode},
    bytecode::{value::CHSValue, ByteCode, object::CHSObj},
};

const STACK_CAPACITY: usize = 10240;

#[derive(Debug)]
pub struct CHSVM {
    data_stack: Vec<Rc<CHSValue>>, 
    return_stack: Vec<usize>, 
    constanst_pool: Vec<Rc<CHSValue>>,
    global_pool: Vec<Rc<CHSValue>>,
    is_halted: bool,
    ip: usize,
    sp: usize,
    program: Vec<Instr>,
}

impl CHSVM {
    pub fn new(program: ByteCode) -> Self {
        Self {
            data_stack: Vec::with_capacity(STACK_CAPACITY),
            return_stack: Vec::with_capacity(STACK_CAPACITY),
            constanst_pool: program.constants,
            global_pool: Vec::with_capacity(STACK_CAPACITY),
            sp: 0,
            ip: 0,
            is_halted: false,
            program: program.code,
        }
    }

    pub fn run(&mut self) {
        loop {
            if self.is_halted || self.ip >= self.program.len() { break; }
            let instr = self.program[self.ip];
            match instr.opcode {
                Opcode::Halt => self.is_halted = true,
                Opcode::Nop => self.ip += 1,
                Opcode::Iconst => {
                    let v = self.constanst_pool.get(instr.operand).unwrap();
                    self.data_stack.push(v.clone());
                }
                Opcode::Pop => {
                    self.data_stack.pop();
                }
                _ => break
            }
        }
    }
}
