use std::{io::Write, rc::Rc};

use crate::{
    exepitons::Trap,
    bytecode::instructions::{Instr, Opcode},
    bytecode::{value::CHSValue, ByteCode},
};

const STACK_CAPACITY: usize = 1024;

#[derive(Debug)]
pub struct CHSVM {
    data_stack: Vec<Rc<CHSValue>>, 
    return_stack: [usize; STACK_CAPACITY],
    is_halted: bool,
    ip: usize,
    rp: usize,
    program: Vec<Instr>,
}

impl CHSVM {
    pub fn new(program: ByteCode) -> Self {
        Self {
            data_stack: Vec::new(),
            return_stack: [0; STACK_CAPACITY],
            ip: 0,
            rp: 0,
            is_halted: false,
            program: program.code,
        }
    }

    pub fn execute_next_instr(&mut self) -> Result<(), Trap> {
        let instr = self.decode_instr()?;
        match instr.opcode {
            Opcode::Halt => {
                self.is_halted = true;
                return Ok(());
            } //_ => Ok(())
        }
    }

    pub fn run(&mut self) {
        // for i in self.program.iter().enumerate() {
        //     println!("{} -> {:?}", i.0, i.1);
        // }
        while !self.is_halted {
            match self.execute_next_instr() {
                Ok(_) => {} //{println!("{:?}\n{:?}\n{:?}\n{:?}\n", self.data_stack, self.return_stack, self.ip, self.sp)}
                Err(e) => {
                    eprintln!(
                        "It's a trap: {:?} at {}\nCurrent stack: {:?}\nCurrent Instr: {:?}\nCurrent Return Pointer: {}\nCurrent Local Stack: {:?}",
                        e, self.ip-1, self.data_stack, self.program[self.ip-1], self.rp, self.return_stack);
                    break;
                }
            }
        }
    }

    fn decode_instr(&mut self) -> Result<Instr, Trap> {
        self.ip += 1;
        if self.ip > self.program.len() {
            return Err(Trap::ProgramEndWithoutHalt);
        }
        //println!("{:?}", self.program.code[self.ip - 1]);
        //println!("DATA: {:?}", self.data_stack);
        Ok(self.program[self.ip - 1])
    }

}
