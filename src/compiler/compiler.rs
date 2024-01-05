use std::rc::Rc;

use crate::{exepitons::GenericError, generic_error, bytecode::{ByteCode, value::CHSValue, object::CHSObj, instructions::{Instr, Opcode}}};

use super::ast::*;

pub struct Compiler {
    ast: Program
}


impl Compiler {
    pub fn new(program: Program) -> Self {
        Self { ast: program }
    }

    pub fn compile(&self) -> Result<ByteCode, GenericError> {
        // let ast = self.ast.clone();
        // for stmt in ast.top_level_stmts.into_iter() {
        //     match stmt {
        //         TopLevelStmt::PrintStmt(v) => {
        //             match *v {
        //                 Expression::StrLit(s) => {
        //                     let val = CHSValue::Obj(Rc::new(CHSObj::Str(s.value)));
        //                     let instr = Instr { opcode: Opcode::Print, operand: 0 };
        //                     return Ok(ByteCode { code: vec![instr], constants: vec![val.into()]  });
        //                 },
        //                 _ => return generic_error!("")
        //             }
        //         },
        //         _ => return generic_error!("???")
        //     }
        // }

        generic_error!("Not Implemeted yet")
    }
}