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
pub enum Operators {
    Add,
    Minus,
    Mul,
    Div
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Int(IntLiteral),
    Op(Operators),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Expressions {
    pub values: Vec<Expression>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Proc {
    pub name: Identifier,
    pub body: Expressions
}

#[derive(Debug, PartialEq, Eq)]
pub enum TopLevel {
    Proc(Box<Proc>)
}

#[derive(Debug, PartialEq, Eq)]
pub struct Program {
    pub stmt: Vec<TopLevel>,
    pub file: PathBuf,
}
