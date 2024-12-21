use chs_lexer::{Lexer, Token, TokenKind};
use chs_util::{chs_error, CHSError};
use nodes::{Expression, Module, Var, VarDecl};
use types::{generalize, infer};

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
            chs_error!("{} ERROR: Unexpected token {}({})", token.loc, token.kind, token.value)
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
                chs_error!("{} ERROR: Invalid token ({})", token.loc, token.value);
            }

            self.parse_top_expression(token)?;
        }
        Ok(self.module)
    }

    fn parse_top_expression(&mut self, token: Token) -> Result<(), CHSError> {
        use chs_lexer::TokenKind::*;
        match token.kind {
            Word if self.peek().kind == Colon => {
                self.next();
                self.expect_kind(Assign)?;
                // let ttoken = self.next();
                // let _ttype = match ttoken.kind {
                //     Word if ttoken.val_eq("int") => CHSType::Const(ttoken.value),
                //     Word if ttoken.val_eq("bool") => CHSType::Const(ttoken.value),
                //     Word  => CHSType::new_gen_var(1),
                //     _ => todo!()
                // };
                // TODO: MAKE THE TYPE INFER AFTER PARSING EVERYTHING
                let value = self.parse_expression()?;
                let ty = infer(&mut self.module, &value, 1)?;
                let generalized_ty = generalize(ty, 0);
                let name = token.value;
                self.module.env.insert(name.clone(), generalized_ty.clone());
                self.module.push_var_decl(
                    VarDecl {
                        loc: token.loc,
                        name,
                        ttype: generalized_ty,
                        value,
                    }
                );
                Ok(())
            },
            _ => {
                chs_error!(
                    "{} ERROR: Invalid Expression on top level {}({})", token.loc, token.kind, token.value
                )
            }
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, CHSError> {
        use chs_lexer::TokenKind::*;
        let token = self.next();
        match token.kind {
            Interger => Ok(Expression::from_literal_token(token)?),
            Keyword if token.val_eq("true") || token.val_eq("false") => Ok(Expression::from_literal_token(token)?),
            Word => {
                Ok(Expression::Var(Var {
                    loc: token.loc,
                    name: token.value
                }))
            },
            _ => todo!()
        }
    }
}
