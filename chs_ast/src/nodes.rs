use std::collections::HashMap;

use chs_lexer::Token;
use chs_util::{chs_error, CHSError, Loc};

use crate::types::{CHSType, CHSTypeId};

#[derive(Debug, Default)]
pub struct Module {
    pub top_level: Vec<Expression>,
    pub env: HashMap<String, CHSType>,
    pub id: CHSTypeId
}

impl Module {
    pub fn with_env(env: HashMap<String, CHSType>) -> Self {
        Self { env, ..Default::default() }
    }

    pub fn push(&mut self, expr: Expression) {
        self.top_level
            .push(expr);
    }

    pub fn set_env(&mut self, env: HashMap<String, CHSType>) {
        self.env = env;
    }
}

pub type VarId = usize;

#[derive(Debug)]
pub enum Expression {
    VarDecl(Box<VarDecl>),
    Literal(Literal),
    Var(Var),
}

impl Expression {
    pub fn from_literal_token(token: Token) -> Result<Self, CHSError> {
        use chs_lexer::TokenKind::*;
        match token.kind {
            Interger => Ok(Self::Literal(Literal::IntegerLiteral {
                loc: token.loc,
                value: token
                    .value
                    .parse()
                    .expect("No interger token. Probably a lexer error."),
            })),
            Keyword if token.val_eq("true") || token.val_eq("false") => {
                Ok(Self::Literal(Literal::BooleanLiteral {
                    loc: token.loc,
                    value: token
                        .value
                        .parse()
                        .expect("No interger token. Probably a lexer error."),
                }))
            }
            _ => chs_error!("{} Unsuported literal", token.loc),
        }
    }

    pub fn loc(&self) -> &Loc {
        match self {
            Expression::VarDecl(v) => &v.loc,
            Expression::Literal(literal) => literal.loc(),
            Expression::Var(var) => &var.loc,
        }
    }
}

#[derive(Debug)]
pub struct Var {
    pub loc: Loc,
    pub name: String,
}

#[derive(Debug)]
pub struct VarDecl {
    pub loc: Loc,
    pub name: String,
    pub value: Expression,
    pub ttype: CHSType,
}

#[derive(Debug)]
pub enum Literal {
    IntegerLiteral { loc: Loc, value: i64 },
    BooleanLiteral { loc: Loc, value: bool },
}

impl Literal {
    pub fn loc(&self) -> &Loc {
        match self {
            Literal::IntegerLiteral { loc, value: _ } => loc,
            Literal::BooleanLiteral { loc, value: _ } => loc,
        }
    }
}

#[cfg(test)]
mod tests {
    use chs_lexer::Lexer;

    use super::*;

    #[test]
    fn ast_literal_token() {
        let mut lex = Lexer::new(file!().into(), "10 :".into());
        assert!(
            Expression::from_literal_token(lex.next_token()).is_ok(),
            "Token 1 should be a literal"
        );
        assert!(
            Expression::from_literal_token(lex.next_token()).is_err(),
            "Token 1 should not be a literal"
        );
    }
}
