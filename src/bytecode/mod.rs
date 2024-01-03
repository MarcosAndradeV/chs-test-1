
use self::instructions::Instr;

pub mod value;
pub mod instructions;

#[derive(Debug)]
pub struct ByteCode {
    pub code: Vec<Instr>
}