use std::path::PathBuf;

use crate::{
    instructions::{Instr, Opcode},
    value::CHSValue,
};

use super::{
    lexer::{Lexer, Token, TokenKind},
    node::{Expression, Expressions, Identifier, IntLiteral, Operators, Proc, Program, TopLevel},
};

macro_rules! error {
    ($message: expr, $($field: expr),*) => {
        return Err(ParseError {
            message: format!($message, $($field),*),
        })
    };

    ($message: expr) => {
        return Err(ParseError {
            message: $message.to_string(),
        })
    }
}

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
}

pub struct Parser {
    file: PathBuf,
    lexer: Lexer,
    peeked: Option<Token>,
    pos: usize,
}

type ResTok = Result<Token, ParseError>;

impl Parser {
    pub fn new(input: Vec<u8>, file: PathBuf) -> Self {
        let lexer = Lexer::new(input);

        Self {
            file,
            lexer,
            peeked: None,
            pos: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Instr>, ParseError> {
        let mut stmt = Vec::new();

        loop {
            let token = self.next();

            if token.kind == TokenKind::Null {
                let file = self.file.clone();
                stmt.push(Instr::new(Opcode::Halt, CHSValue::none()));
                return Ok(stmt);
            }
            stmt.push(self.expression(token)?);
        }
    }

    fn expect(&mut self, kind: TokenKind) -> ResTok {
        let token = self.next();

        if token.kind == kind {
            return Ok(token);
        }

        error!("Expect {:?} at {}", kind, self.pos)
    }

    fn expect_or(&mut self, kind: TokenKind, kind_2: TokenKind) -> ResTok {
        let token = self.next();

        if token.kind == kind || token.kind == kind_2 {
            return Ok(token);
        }

        error!("Expect {:?} or {:?} at {}", kind, kind_2, self.pos)
    }

    fn not_expect(&mut self, kind: TokenKind) -> ResTok {
        let token = self.next();

        if token.kind != kind {
            return Ok(token);
        }

        error!("Not Expect {:?} at {}", kind, self.pos)
    }

    fn next(&mut self) -> Token {
        loop {
            self.pos += 1;
            let token = self
                .peeked
                .take()
                .unwrap_or_else(|| self.lexer.next_token());

            match token.kind {
                TokenKind::Comment | TokenKind::Whitespace => {}
                _ => return token,
            }
        }
    }
    fn require(&mut self) -> ResTok {
        let tok = self.next();
        if matches!(tok.kind, TokenKind::Invalid | TokenKind::Null) {
            return error!("require");
        }
        Ok(tok)
    }

    // fn top_level(&mut self, token: Token) -> Result<TopLevel, ParseError> {
    //     let expr = match token.kind {
    //         TokenKind::Proc => self.proc()?,
    //         _ => return error!("No top level"),
    //     };
    //     Ok(expr)
    // }

    // fn proc(&mut self) -> Result<TopLevel, ParseError> {
    //     let name = Identifier::from(self.expect(TokenKind::Identifier)?);
    //     let _ = self.expect(TokenKind::CurlyOpen)?;

    //     let body = self.expressions()?;

    //     Ok(TopLevel::Proc(Box::new(Proc { name, body })))
    // }

    // fn expressions(&mut self) -> Result<Expressions, ParseError> {
    //     let mut values = Vec::new();

    //     loop {
    //         let token = self.require()?;

    //         if token.kind == TokenKind::CurlyClose {
    //             return Ok(Expressions { values });
    //         }

    //         values.push(self.expression(token)?);
    //     }
    // }

    fn expression(&self, token: Token) -> Result<Instr, ParseError> {
        let expr = match token.kind {
            TokenKind::Int => {
                let val = match token.value.parse::<i64>() {
                    Ok(v) => v,
                    Err(_) => error!("..."),
                };
                Instr::new(Opcode::Pushi, CHSValue::from(val))
            }
            TokenKind::Add => Instr::new(Opcode::Add, CHSValue::None),
            TokenKind::Minus => Instr::new(Opcode::Minus, CHSValue::None),
            TokenKind::Mul => Instr::new(Opcode::Mul, CHSValue::None),
            TokenKind::Div => Instr::new(Opcode::Div, CHSValue::None),
            _ => return error!(""),
        };
        Ok(expr)
    }
}
