

use std::slice::Iter;

use chs_lexer::Token;
use chs_util::{chs_error, CHSError, Loc};

use crate::types::CHSType;

#[derive(Debug, Default)]
pub struct Module {
    pub top_level: Vec<Expression>,
}

impl Module {

    pub fn push(&mut self, expr: Expression) {
        self.top_level.push(expr);
    }

}

pub type VarId = usize;

#[derive(Debug)]
pub enum Expression {
    VarDecl(Box<VarDecl>),
    FnDecl(Box<FnDecl>),
    Literal(Literal),
    Var(Var),
    Call(Box<Call>),
    Ref(Box<Self>),
    Deref(Box<Self>),
    ExprList(Vec<Expression>)
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
            String => Ok(Self::Literal(Literal::StringLiteral {
                loc: token.loc,
                value: token.value,
            })),
            _ => chs_error!("{} Unsuported literal", token.loc),
        }
    }

    pub fn loc(&self) -> &Loc {
        match self {
            Expression::VarDecl(v) => &v.loc,
            Expression::Literal(literal) => literal.loc(),
            Expression::Var(var) => &var.loc,
            Expression::Call(call) => &call.loc,
            Expression::FnDecl(fn_decl) => &fn_decl.loc,
            Expression::Ref(_expression) => todo!(),
            Expression::Deref(_expression) => todo!(),
            _ => todo!(),
        }
    }

    pub fn len(&self) -> usize {
        if let Expression::ExprList(v) = self {
            v.len()
        } else {
            1
        }
    }

    pub fn iter(&self) -> Iter<'_, Expression> {
        if let Expression::ExprList(ls) = self {
            ls.iter()
        } else {
            unreachable!("aaaaa")
        }
    }
}

#[derive(Debug)]
pub struct Call {
    pub loc: Loc,
    pub caller: Expression,
    pub args: Expression,
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
    pub ttype: Option<CHSType>,
}

#[derive(Debug)]
pub struct FnDecl {
    pub loc: Loc,
    pub name: String,
    pub args: Vec<(String, CHSType)>,
    pub ret_type: CHSType,
    pub body: Expression,
}

#[derive(Debug)]
pub enum Literal {
    IntegerLiteral { loc: Loc, value: i64 },
    BooleanLiteral { loc: Loc, value: bool },
    StringLiteral { loc: Loc, value: String },
}

impl Literal {
    pub fn loc(&self) -> &Loc {
        match self {
            Literal::IntegerLiteral { loc, value: _ } => loc,
            Literal::BooleanLiteral { loc, value: _ } => loc,
            Literal::StringLiteral { loc, value: _ } => loc,
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
