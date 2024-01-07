
use std::rc::Rc;

use self::{instructions::Instr, object::CHSObj, value::CHSValue};

pub mod value;
pub mod instructions;
pub mod object;

#[derive(Debug)]
pub struct ByteCode {
    pub code: Vec<Instr>,
    pub constants: Vec<Rc<CHSValue>>
}