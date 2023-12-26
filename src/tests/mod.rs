#[cfg(test)]
mod test {
    use std::fs::File;

    use crate::{instructions::{Instr, Opcode}, value::CHSValue, runtime::vm::CHSVM};

    use super::*;
   #[test]
   fn test(){
        let p: Vec<i64> = vec![1, 1, 0];
        let program = vec![
        Instr::new(Opcode::from(p[0]), CHSValue::from(p[1])),    // push 1
        Instr::new(Opcode::from(p[0]), CHSValue::from(1.1)),     // push 1.1
        Instr::new(Opcode::from(p[0]), CHSValue::from(10usize)), // push %10
        Instr::new(Opcode::Print, CHSValue::None),               // print
        Instr::new(Opcode::Print, CHSValue::None),               // print
        Instr::new(Opcode::Print, CHSValue::None),               // print
        Instr::new(Opcode::from(p[2]), CHSValue::None),          // halt
        ];

        let mut vm = CHSVM::new(program);

        vm.run();
   }
   
}