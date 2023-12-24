#![allow(unused)]
pub mod vm;

#[cfg(test)]
mod test {
    use super::vm::*;

   #[test]
   fn test_vm() {
        let mut vm = CHSVM::new();
        let program = vec![
            Instr::new(InstrKind::Push, Some(4)),
            Instr::new(InstrKind::Push, Some(2)),
            Instr::new(InstrKind::Div, None),
            ];

        for i in program {
            match vm.execute_instr(i) {
                Ok(_) => {println!("{:?}", vm.stack)},
                Err(trap) => {
                    eprintln!("ERROR: {:?}", trap);
                    break;
                },
            }

        }
   }
}