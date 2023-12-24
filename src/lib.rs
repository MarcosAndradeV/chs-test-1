#![allow(unused)]
pub mod vm;
pub mod assembler;
pub mod instructions;



#[cfg(test)]
mod test {
    use super::vm::*;
    use crate::instructions::{Instr, InstrKind};


    #[test]
    fn test_vm() {
        let program = vec![
            // start:
            Instr::new(InstrKind::Push, Some(0)),
            Instr::new(InstrKind::Push, Some(1)),
            Instr::new(InstrKind::Dup, Some(1)),
            Instr::new(InstrKind::Dup, Some(1)),
            Instr::new(InstrKind::Add, None),

            Instr::new(InstrKind::Dup, Some(0)),
            Instr::new(InstrKind::Push, Some(2584)),
            Instr::new(InstrKind::Lte, None),
            Instr::new(InstrKind::JmpIf, Some(10)),
            Instr::new(InstrKind::Jmp, Some(2)),

            Instr::new(InstrKind::Print, None),
            Instr::new(InstrKind::Halt, None),
        ];
        let mut vm = CHSVM::new(program);

        vm.run();
    }
}
