use std::path::PathBuf;

use crate::{exepitons::GenericError, generic_error};

use self::{node::Program, parser::{ParseError, Parser}};



pub mod lexer;
pub mod node;
pub mod parser;


pub fn make_ast(source: String) -> Result<Program, GenericError> {
    let mut parser = Parser::new(source.into_bytes(), PathBuf::new());
    match parser.parse() {
        Ok(ok) => Ok(ok),
        Err(p) => generic_error!("Cannot parser the file: {}", p.message),
    }
}