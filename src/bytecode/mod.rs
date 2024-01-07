
use std::rc::Rc;

use self::{instructions::Instr, value::CHSValue};

pub mod value;
pub mod instructions;

#[derive(Debug)]
pub struct ByteCode {
    pub code: Vec<Instr>,
    //pub constants: Vec<Rc<CHSValue>>
}