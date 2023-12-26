use std::cmp::{Eq, PartialEq};
use std::path::PathBuf;

use super::lexer::Token;

#[derive(Debug, PartialEq, Eq)]
pub struct IntLiteral {
    pub value: String,
}

impl From<Token> for IntLiteral {
    fn from(value: Token) -> Self {
        Self { value: value.value }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct FloatLiteral {
    pub value: String,
}

impl From<Token> for FloatLiteral {
    fn from(value: Token) -> Self {
        Self { value: value.value }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Identifier {
    pub name: String,
}

impl From<Token> for Identifier {
    fn from(value: Token) -> Self {
        Self { name: value.value }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Bool(bool);

impl From<bool> for Bool {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Nil;

#[derive(Debug, PartialEq, Eq)]
pub enum Stmt {
    A
}

#[derive(Debug, PartialEq, Eq)]
pub struct Program {
    pub stmt: Vec<Stmt>,
    pub file: PathBuf,
}
