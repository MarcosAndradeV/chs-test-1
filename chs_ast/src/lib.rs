use chs_lexer::{Lexer, Token, TokenKind};
use chs_util::{chs_error, CHSError};
use nodes::{Call, Expression, FnDecl, Module, Var, VarDecl};
use types::{generalize, infer, unify, CHSType, CHSBOOL, CHSCHAR, CHSINT, CHSSTRING, CHSVOID};

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
                "{} Unexpected token '{}' of '{}', Expect: {}",
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
        let a = CHSType::Const("int".to_string());
        self.module.env.insert(
            "add".to_string(),
            CHSType::Arrow(vec![a.clone(), a.clone()], a.into()),
        );

        loop {
            let token = self.next();
            if token.kind.is_eof() {
                break;
            }
            if token.kind == Invalid {
                chs_error!("{} Invalid token '{}'", token.loc, token.value);
            }

            self.parse_top_expression(token)?;
        }
        Ok(self.module)
    }

    // TODO: MAKE THE TYPE INFER AFTER PARSING EVERYTHING
    fn parse_top_expression(&mut self, token: Token) -> Result<(), CHSError> {
        use chs_lexer::TokenKind::*;
        self.module.id.reset_id();
        match token.kind {
            Word if self.peek().kind == Colon => {
                self.next();
                let (value, ttype) = if let Some(ttype) = self.parse_type()? {
                    self.expect_kind(Assign)?;
                    let value = self.parse_expression()?;
                    let ty = infer(&mut self.module, &value, 1)?;
                    (value, unify(ttype, generalize(ty, 1))?)
                } else {
                    let value = self.parse_expression()?;
                    let ty = infer(&mut self.module, &value, 1)?;
                    (value, generalize(ty, 1))
                };
                let name = token.value;
                let expr = Expression::VarDecl(Box::new(VarDecl {
                    loc: token.loc,
                    name,
                    ttype,
                    value,
                }));
                infer(&mut self.module, &expr, 1)?;
                self.module.push(expr);
                Ok(())
            }
            Keyword if token.val_eq("fn") => {
                let token = self.expect_kind(Word)?;
                let name = token.value;
                self.expect_kind(ParenOpen)?;
                let (args, ret_type) = self.parse_fn_type_list()?;
                self.expect_kind(Assign)?;
                self.module.env.insert(name.clone(), ret_type.clone());
                let body = self.parse_expression()?;
                let expr = Expression::FnDecl(Box::new(FnDecl {
                    loc: token.loc,
                    name,
                    args,
                    ret_type,
                    body,
                }));
                infer(&mut self.module, &expr, 1)?;
                self.module.push(expr);
                Ok(())
            }
            _ => {
                self.peeked = Some(token);
                let expr = self.parse_expression()?;
                self.module.push(expr);
                Ok(())
            }
            /*
                _ => {
                    chs_error!(
                        "{} Invalid Expression on top level {}('{}')",
                        token.loc,
                        token.kind,
                        token.value
                    )
                }
            */
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, CHSError> {
        use chs_lexer::TokenKind::*;
        let token = self.next();
        let mut left = match token.kind {
            Interger => Expression::from_literal_token(token)?,
            Keyword if token.val_eq("true") || token.val_eq("false") => {
                Expression::from_literal_token(token)?
            }
            Keyword if token.val_eq("print") => {
                let expr = self.parse_expression()?;
                Expression::Print(expr.into())
            }
            Ampersand => {
                let expr = self.parse_expression()?;
                Expression::Ref(expr.into())
            }
            Asterisk => {
                let expr = self.parse_expression()?;
                Expression::Deref(expr.into())
            }
            Word => Expression::Var(Var {
                loc: token.loc,
                name: token.value,
            }),
            String => Expression::from_literal_token(token)?,
            _ => chs_error!(
                "{} Unexpected token {}('{}')",
                token.loc,
                token.kind,
                token.value
            ),
        };
        loop {
            let ptoken = self.peek();
            match ptoken.kind {
                ParenOpen => {
                    let ptoken = self.next();
                    let args = self.parse_arg_list()?;
                    let call = Call {
                        loc: ptoken.loc,
                        caller: left,
                        args,
                    };
                    left = Expression::Call(Box::new(call));
                }
                _ => return Ok(left),
            }
        }
    }

    fn parse_arg_list(&mut self) -> Result<Vec<Expression>, CHSError> {
        use chs_lexer::TokenKind::*;
        let mut args = vec![];
        loop {
            let ptoken = self.peek();
            match ptoken.kind {
                ParenClose => {
                    self.next();
                    return Ok(args);
                }
                Comma => {
                    self.next();
                    continue;
                }
                _ => {
                    let value = self.parse_expression()?;
                    args.push(value);
                }
            }
        }
    }

    fn parse_fn_type_list(&mut self) -> Result<(Vec<(String, CHSType)>, CHSType), CHSError> {
        use chs_lexer::TokenKind::*;
        let mut list = vec![];
        let mut ret_type = CHSType::Const("()".to_string());
        loop {
            let ptoken = self.peek();
            match ptoken.kind {
                ParenClose => {
                    self.next();
                    let ptoken = self.peek();
                    if ptoken.kind == Arrow {
                        self.next();
                        if let Some(value) = self.parse_type()? {
                            ret_type = value;
                        } else {
                            return Ok((list, ret_type));
                        }
                    }
                    return Ok((list, ret_type));
                }
                Comma => {
                    self.next();
                    continue;
                }
                Word => {
                    let token = self.next();
                    self.expect_kind(Colon)?;
                    if let Some(value) = self.parse_type()? {
                        list.push((token.value, value));
                    } else {
                        return Ok((list, ret_type));
                    }
                }
                _ => chs_error!(""),
            }
        }
    }

    fn parse_type(&mut self) -> Result<Option<CHSType>, CHSError> {
        use chs_lexer::TokenKind::*;
        let ttoken = self.next();
        let ttype = match ttoken.kind {
            Word if ttoken.val_eq("int")  => Some(CHSINT.clone()),
            Word if ttoken.val_eq("bool") => Some(CHSBOOL.clone()),
            Word if ttoken.val_eq("char") => Some(CHSCHAR.clone()),
            Word if ttoken.val_eq("void") => Some(CHSVOID.clone()),
            Word if ttoken.val_eq("string") => Some(CHSSTRING.clone()),
            Asterisk => {
                if let Some(ttp) = self.parse_type()? {
                    Some(CHSType::App(
                        CHSType::Const("pointer".to_string()).into(),
                        vec![ttp],
                    ))
                } else {
                    chs_error!("Expect type")
                }
            }
            Assign => None,
            _ => chs_error!("Type not implemnted {}", ttoken),
        };
        Ok(ttype)
    }
}
