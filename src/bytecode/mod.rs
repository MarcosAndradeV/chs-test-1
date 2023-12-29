use serde::{Serialize, Deserialize};

use self::instructions::Instr;

pub mod value;
pub mod instructions;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct ByteCode {
    pub code: Vec<Instr>
}