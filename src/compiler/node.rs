use std::cmp::{Eq, PartialEq};
use std::path::PathBuf;

use crate::bytecode::instructions::Instr;

#[derive(Debug)]
pub struct Program {
    pub stmt: Vec<Instr>,
    pub file: PathBuf,
}
