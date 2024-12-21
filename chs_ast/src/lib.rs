use chs_lexer::{Lexer, Token, TokenKind};
use chs_util::{chs_error, CHSError};
use nodes::{Expression, Module, Var, VarDecl};
use types::{generalize, infer, unify, CHSType};

pub mod nodes;
pub mod types;

// [Token] -> Module

#[derive(Default)]
pub struct Parser {
    lexer: Lexer,
    peeked: Option<Token>,
    module: Module,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self {
            lexer,
            ..Default::default()
        }
    }

    fn next(&mut self) -> Token {
        let token = self
            .peeked
            .take()
            .unwrap_or_else(|| self.lexer.next_token());
        return token;
    }

    fn expect_kind(&mut self, kind: TokenKind) -> Result<Token, CHSError> {
        let token = self.next();
        if token.kind != kind {
            chs_error!(
                "{} Unexpected token {}({}), Expect: {}",
                token.loc,
                token.kind,
                token.value,
                kind
            )
        }
        Ok(token)
    }

    fn peek(&mut self) -> &Token {
        if self.peeked.is_none() {
            self.peeked = Some(self.next());
        }
        self.peeked.as_ref().unwrap()
    }

    pub fn parse(mut self) -> Result<Module, CHSError> {
        use chs_lexer::TokenKind::*;
        loop {
            let token = self.next();
            if token.kind.is_eof() {
                break;
            }
            if token.kind == Invalid {
                chs_error!("{} Invalid token ({})", token.loc, token.value);
            }

            self.parse_top_expression(token)?;
        }
        Ok(self.module)
    }

    // TODO: MAKE THE TYPE INFER AFTER PARSING EVERYTHING
    fn parse_top_expression(&mut self, token: Token) -> Result<(), CHSError> {
        use chs_lexer::TokenKind::*;
        match token.kind {
            Word if self.peek().kind == Colon => {
                self.next();
                let (value, ttype) = if let Some(mut ttype) = self.parse_type()? {
                    self.expect_kind(Assign)?;
                    let value = self.parse_expression()?;
                    let ty = infer(&mut self.module, &value, 1)?;
                    let mut tgen = generalize(ty, 0);
                    unify(&mut tgen, &mut ttype)?;
                    (value, tgen)
                } else {
                    let value = self.parse_expression()?;
                    let ty = infer(&mut self.module, &value, 1)?;
                    (value, generalize(ty, 0))
                };

                let name = token.value;
                self.module.env.insert(name.clone(), ttype.clone());
                self.module.push(Expression::VarDecl(Box::new(VarDecl {
                    loc: token.loc,
                    name,
                    ttype,
                    value,
                })));
                Ok(())
            }
            _ => {
                chs_error!(
                    "{} Invalid Expression on top level {}({})",
                    token.loc,
                    token.kind,
                    token.value
                )
            }
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, CHSError> {
        use chs_lexer::TokenKind::*;
        let token = self.next();
        match token.kind {
            Interger => Ok(Expression::from_literal_token(token)?),
            Keyword if token.val_eq("true") || token.val_eq("false") => {
                Ok(Expression::from_literal_token(token)?)
            }
            Word => Ok(Expression::Var(Var {
                loc: token.loc,
                name: token.value,
            })),
            _ => todo!(),
        }
    }

    fn parse_type(&mut self) -> Result<Option<CHSType>, CHSError> {
        use chs_lexer::TokenKind::*;
        let ttoken = self.next();
        let ttype = match ttoken.kind {
            Word if ttoken.val_eq("int") => Some(CHSType::Const(ttoken.value)),
            Word if ttoken.val_eq("bool") => Some(CHSType::Const(ttoken.value)),
            Word => {
                self.module.id.reset_id();
                Some(CHSType::new_var(&mut self.module.id, 0))
            }
            Assign => None,
            _ => chs_error!("Type not implemnted"),
        };
        Ok(ttype)
    }
}
