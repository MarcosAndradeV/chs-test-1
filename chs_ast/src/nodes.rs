use core::fmt;
use std::slice::Iter;

use chs_lexer::Token;
use chs_util::{chs_error, CHSError, Loc};

use crate::types::CHSType;

#[derive(Debug, Default)]
pub struct Module {
    pub name: String,
    pub expressions: Vec<TopLevelExpression>,
}

impl Module {
    pub fn push(&mut self, expr: TopLevelExpression) {
        self.expressions.push(expr);
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Module: {}", self.name)?;
        for expr in &self.expressions {
            writeln!(f, " {expr} ")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum TopLevelExpression {
    FnDecl(Box<FnDecl>),
}

impl fmt::Display for TopLevelExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TopLevelExpression::FnDecl(v) => write!(f, "{v}"),
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    VarDecl(Box<VarDecl>),
    Literal(Literal),
    Var(Var),
    Call(Box<Call>),
    Ref(Box<Self>),
    Deref(Box<Self>),
    ExprList(Vec<Expression>),
    Binop(Box<Binop>),
    Group(Box<Expression>)
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::VarDecl(v) => write!(f, "{v}"),
            Expression::Literal(v) => write!(f, "{v}"),
            Expression::Var(v) => write!(f, "{v}"),
            Expression::Call(v) => write!(f, "{v}"),
            Expression::Ref(v) => write!(f, "(&{v})"),
            Expression::Deref(v) => write!(f, "(*{v})"),
            Expression::ExprList(v) => write!(f, "{v:?}"),
            Expression::Binop(v) => write!(f, "{v}"),
            Expression::Group(v) => write!(f, "({v})"),
        }
    }
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
            Expression::Ref(_expression) => todo!(),
            Expression::Deref(_expression) => todo!(),
            _ => todo!(),
        }
    }

    pub fn precedence(&self) -> Precedence {
        match self {
            Expression::Binop(binop) => binop.op.precedence(),
            Expression::Ref(_) | Expression::Deref(_) => Precedence::Prefix,
            Expression::Call(_)  => Precedence::Call,
            _ => Precedence::Lowest,
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

    /// Returns `true` if the expression is [`ExprList`].
    ///
    /// [`ExprList`]: Expression::ExprList
    #[must_use]
    pub fn is_expr_list(&self) -> bool {
        matches!(self, Self::ExprList(..))
    }
}

#[derive(Debug)]
pub struct Call {
    pub loc: Loc,
    pub caller: Expression,
    pub args: Expression,
}

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}(", self.caller)?;
        for (i, item) in self.args.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", item)?;
        }
        write!(f, ")")
    }
}

#[derive(Debug)]
pub struct Var {
    pub loc: Loc,
    pub name: String,
}

impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug)]
pub struct VarDecl {
    pub loc: Loc,
    pub name: String,
    pub value: Expression,
    pub ttype: Option<CHSType>,
}

impl fmt::Display for VarDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.ttype.is_some() {
            write!(
                f,
                "{} : {} = {}",
                self.name,
                self.ttype.as_ref().unwrap(),
                self.value
            )
        } else {
            write!(f, "{} := {}", self.name, self.value)
        }
    }
}

#[derive(Debug)]
pub struct FnDecl {
    pub loc: Loc,
    pub name: String,
    pub args: Vec<(String, CHSType)>,
    pub ret_type: CHSType,
    pub body: Expression,
}

impl fmt::Display for FnDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fn {}(", self.name)?;
        for (i, item) in self.args.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", item.0, item.1)?;
        }
        writeln!(f, ") -> {}", self.ret_type)?;
        for expr in self.body.iter() {
            writeln!(f, "   {expr} ")?;
        }
        write!(f, " end")
    }
}

#[derive(Debug)]
pub struct Binop {
    pub loc: Loc,
    pub op: Operator,
    pub left: Expression,
    pub right: Expression,
}

impl fmt::Display for Binop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.left, self.op, self.right)
    }
}

#[derive(Debug)]
pub enum Operator {
    Add,
    Div,
    Minus,
    Mult,
    Eq,
    NEq,
    Gt,
    Lt,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Minus => write!(f, "-"),
            Operator::Mult => write!(f, "*"),
            Operator::Div => write!(f, "/"),
            Operator::Eq => write!(f, "=="),
            Operator::NEq => write!(f, "!="),
            Operator::Gt => write!(f, ">"),
            Operator::Lt => write!(f, "<"),
        }
    }
}

impl Operator {
    pub fn from_token(token: &Token) -> Result<Self, CHSError> {
        use chs_lexer::TokenKind::*;
        match token.kind {
            Plus => Ok(Self::Add),
            Minus => Ok(Self::Minus),
            Asterisk => Ok(Self::Mult),
            Slash => Ok(Self::Div),
            Eq => Ok(Self::Eq),
            NotEq => Ok(Self::NEq),
            _ => chs_error!("{} Unsuported operator", token.loc),
        }
    }

    pub fn is_logical(&self) -> bool {
        match self {
            Operator::Eq | Operator::Gt | Operator::Lt | Operator::NEq => true,
            Operator::Add | Operator::Div | Operator::Minus | Operator::Mult => false,
        }
    }

    pub fn precedence(&self) -> Precedence {
        match self {
            Operator::Add | Operator::Minus => Precedence::Sum,
            Operator::Mult | Operator::Div => Precedence::Product,
            Operator::Lt | Operator::Gt => Precedence::LessGreater,
            Operator::Eq | Operator::NEq => Precedence::Equals,
            // _ => Precedence::Lowest,
        }
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest = 1,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

#[derive(Debug)]
pub enum Literal {
    IntegerLiteral { loc: Loc, value: i64 },
    BooleanLiteral { loc: Loc, value: bool },
    StringLiteral { loc: Loc, value: String },
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::IntegerLiteral { loc: _, value } => write!(f, "{value}"),
            Literal::BooleanLiteral { loc: _, value } => write!(f, "{value}"),
            Literal::StringLiteral { loc: _, value: _ } => write!(f, "ESCAPE THE STRINGS"),
        }
    }
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
