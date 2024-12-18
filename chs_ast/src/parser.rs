use chs_util::{chs_error, CHSError};

#[allow(unused_imports)]
use crate::{lexer::{Lexer, Token, TokenKind}, nodes::{self, Expression, Group, Module, WordExpr, Word, TopLevelExpr}};


pub struct Parser {
    lexer: Lexer,
    peeked: Option<Token>
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self { lexer, peeked: None }
    }

    fn next_token(&mut self) -> Token {
        let token = self.peeked.take().unwrap_or_else(|| self.lexer.next_token());
        return token;
    }

    fn peek_token(&mut self) -> &Token {
        if self.peeked.is_none() {
            self.peeked = Some(self.next_token());
        }
        self.peeked.as_ref().unwrap()
    }

    fn expect_kind(&mut self, kind: TokenKind) -> Result<Token, CHSError> {
        let token = self.next_token();
        if token.kind != kind {
            chs_error!("ERROR: {} Expect \"{}\" found {}", token.loc, kind, token.kind);
        }
        Ok(token)
    }

    pub fn parse(&mut self) -> Result<Module, CHSError> {
        let mut exprs = Vec::new();
        loop {
            let token = self.next_token();
            if token.kind == TokenKind::EOF {
                break;
            }
            let expr = self.parse_top_level_expr(token)?;
            exprs.push(expr);
        }

        return Ok(Module {filesource: self.lexer.get_filename(), program: exprs});
    }

    fn parse_top_level_expr(&mut self, token: Token) -> Result<Expression, CHSError> {
        match token.kind {
            TokenKind::Word => Ok(Expression::from_word_token(token)?),
            TokenKind::Interger => Ok(Expression::from_integer_token(token)),
            TokenKind::Colon if self.peek_token().kind == TokenKind::Assign => {
                self.next_token();
                let token = self.expect_kind(TokenKind::Word)?;
                Ok(Expression::from_assign_token_no_type(token))
            }
            _ => chs_error!("{} Invalid token", token),
        }
    }
}
