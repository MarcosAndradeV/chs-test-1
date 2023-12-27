use std::path::PathBuf;

use self::{node::Program, parser::{ParseError, Parser}};



pub mod lexer;
pub mod node;
pub mod parser;


pub fn make_ast(source: String) -> Result<Program, ParseError> {
    let mut parser = Parser::new(source.into_bytes(), PathBuf::new());
    parser.parse()
}