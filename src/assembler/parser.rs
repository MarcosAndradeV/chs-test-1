use std::path::PathBuf;

use super::{
    lexer::{Lexer, Token, TokenKind},
    node::{Bool,
        IntLiteral, Nil, Program, Stmt,
    },
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

type ResExpr = Result<Stmt, ParseError>;

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

    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let mut stmt = Vec::new();

        loop {
            let token = self.next();

            if token.kind == TokenKind::Null {
                let file = self.file.clone();

                return Ok(Program { stmt, file });
            }
            stmt.push(Stmt::A);
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

}
