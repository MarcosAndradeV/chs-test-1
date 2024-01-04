use crate::exepitons::GenericError;
use crate::generic_error;

use super::ast::*;
use super::lexer::*;


pub struct Parser {
    lexer: Lexer,
    peeked: Option<Token>
}

impl Parser {
    pub fn new(source: String) -> Self {
        Self { lexer: Lexer::new(source.into_bytes()), peeked: None }
    }

    fn peek(&mut self) -> &Token {
        if self.peeked.is_none() {
            self.peeked = Some(self.next());
        }

        self.peeked.as_ref().unwrap()
    }

    fn next(&mut self) -> Token {
        loop {
            let token =
                self.peeked.take().unwrap_or_else(|| self.lexer.next_token());

            match token.kind {
                TokenKind::Comment | TokenKind::Whitespace => {}
                _ => return token,
            }
        }
    }

    fn require(&mut self) -> Result<Token, GenericError> {
        let tok = self.next();
        match tok.kind {
            TokenKind::Invalid | TokenKind::Null => generic_error!(""),
            _ => return Ok(tok)
        }
    }

    fn expect(&mut self, kind: TokenKind) -> Result<Token, GenericError> {
        let tok = self.require()?;
        if tok.kind != kind {
            return generic_error!("");
        }
        Ok(tok)
    }

    pub fn parse_program(&mut self) -> Result<Program, GenericError> {
        let mut top_level_stmts = Vec::new();
        loop {
            let tok = self.next();
            if tok.kind == TokenKind::Null {
                return Ok(Program { top_level_stmts });
            }
            top_level_stmts.push(self.parse_top_level(tok)?);
        }
    }

    fn parse_top_level(&mut self, tok: Token) -> Result<TopLevelStmt, GenericError> {
        let stmt = match tok.kind {
            TokenKind::Fn => self.parse_fn()?,
            _ => return generic_error!("{} is not permitted on the Top Level", tok)
        };
        Ok(stmt)
    }

    fn parse_fn(&mut self) -> Result<TopLevelStmt, GenericError> {
        let name = self.expect(TokenKind::Identifier)?.value;


        return Ok(TopLevelStmt::Fn(Box::new(
            FnStmt {
                name,
                arguments: vec![],
                body: vec![],
                return_type: None
            }
        )));
        //generic_error!("Not implemeted yet")
    }

}