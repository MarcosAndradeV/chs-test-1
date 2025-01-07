use core::fmt;

use chs_lexer::Token;
use chs_util::{chs_error, CHSError, Loc};

use chs_types::CHSType;

#[derive(Debug, Default)]
pub struct Module {
    pub name: String,
    pub funcs: Vec<Function>,
    pub type_decls: Vec<(String, CHSType)>,
    pub const_decl: Vec<ConstDecl>,
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Module: {}", self.name)?;
        for expr in &self.const_decl {
            writeln!(f, " {expr} ")?;
        }
        for (name, t) in &self.type_decls {
            writeln!(f, " {name} : {t} ")?;
        }
        for expr in &self.funcs {
            writeln!(f, " {expr} ")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum ConstExpression {
    Symbol(String),
    IntegerLiteral(i64),
    BooleanLiteral(bool),
    StringLiteral(String),
    Void,
}

impl fmt::Display for ConstExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConstExpression::Symbol(v) => write!(f, "{v}"),
            ConstExpression::IntegerLiteral(v) => write!(f, "{v}"),
            ConstExpression::BooleanLiteral(v) => write!(f, "{v}"),
            ConstExpression::StringLiteral(_) => write!(f, "ESCAPE THE STRINGS"),
            ConstExpression::Void => write!(f, "()"),
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    ConstExpression(ConstExpression),
    // ExpressionList(Vec<Self>),
    Binop(Box<Binop>),
    Unop(Box<Unop>),
    Call(Box<Call>),
    VarDecl(Box<VarDecl>),
    Group(Box<Self>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Expression::ExpressionList(v) => {
            //     for (i, item) in v.iter().enumerate() {
            //         if i > 0 {
            //             write!(f, ", ")?;
            //         }
            //         write!(f, "{}\n", item)?;
            //     }
            //     writeln!(f, "")
            // }
            Expression::ConstExpression(v) => write!(f, "{v}"),
            Expression::Call(v) => write!(f, "{v}"),
            Expression::Binop(v) => write!(f, "{v}"),
            Expression::Unop(v) => write!(f, "{v}"),
            Expression::VarDecl(v) => write!(f, "{v}"),
            Expression::Group(v) => write!(f, "({v}"),
        }
    }
}
impl Expression {
    pub fn from_literal_token(token: Token) -> Result<Self, CHSError> {
        use chs_lexer::TokenKind::*;
        match token.kind {
            Interger => {
                let value: i64 = token
                    .value
                    .parse::<i64>()
                    .expect("No interger token. Probably a lexer error.");
                Ok(Self::ConstExpression(ConstExpression::IntegerLiteral(
                    value,
                )))
            }
            Keyword if token.val_eq("true") => {
                Ok(Self::ConstExpression(ConstExpression::BooleanLiteral(true)))
            }
            Keyword if token.val_eq("false") => Ok(Self::ConstExpression(
                ConstExpression::BooleanLiteral(false),
            )),
            Ident => Ok(Self::ConstExpression(ConstExpression::Symbol(token.value))),
            String => Ok(Self::ConstExpression(ConstExpression::StringLiteral(
                token.value,
            ))),
            _ => chs_error!("{} Unsuported literal", token.loc),
        }
    }
}

#[derive(Debug)]
pub struct Function {
    pub loc: Loc,
    pub name: String,
    pub args: Vec<(String, CHSType)>,
    pub ret_type: CHSType,
    pub body: Vec<Expression>,
}

impl fmt::Display for Function {
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
pub struct ConstDecl {
    pub loc: Loc,
    pub name: String,
    pub value: ConstExpression,
    pub ttype: CHSType,
}

impl fmt::Display for ConstDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {} = {}", self.name, self.ttype, self.value)
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
pub struct Call {
    pub loc: Loc,
    pub caller: Expression,
    pub args: Vec<Expression>,
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
pub struct Unop {
    pub loc: Loc,
    pub op: Operator,
    pub left: Expression,
}

impl fmt::Display for Unop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}{})", self.op, self.left)
    }
}

#[derive(Debug)]
pub enum Operator {
    // Binary
    Plus,
    Minus,
    Div,
    Mult,
    Eq,
    NEq,
    Gt,
    Lt,
    // Unary
    Negate,
    LNot,
    Refer,
    Deref,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::Plus => write!(f, "+"),
            Operator::Minus => write!(f, "-"),
            Operator::Mult => write!(f, "*"),
            Operator::Div => write!(f, "/"),
            Operator::Eq => write!(f, "=="),
            Operator::NEq => write!(f, "!="),
            Operator::Gt => write!(f, ">"),
            Operator::Lt => write!(f, "<"),
            Operator::Negate => write!(f, "-"),
            Operator::LNot => write!(f, "!"),
            Operator::Refer => write!(f, "&"),
            Operator::Deref => write!(f, "*"),
        }
    }
}

impl Operator {
    pub fn from_token(token: &Token, unary: bool) -> Result<Self, CHSError> {
        use chs_lexer::TokenKind::*;
        match token.kind {
            Minus if unary => Ok(Self::Negate),
            Bang if unary => Ok(Self::LNot),
            Asterisk if unary => Ok(Self::Deref),
            Ampersand if unary => Ok(Self::Refer),
            Plus => Ok(Self::Plus),
            Minus => Ok(Self::Minus),
            Asterisk => Ok(Self::Mult),
            Slash => Ok(Self::Div),
            Eq => Ok(Self::Eq),
            NotEq => Ok(Self::NEq),
            _ => chs_error!("{} Unsuported operator", token.loc),
        }
    }

    pub fn precedence(&self) -> Precedence {
        match self {
            Operator::Plus | Operator::Minus => Precedence::Sum,
            Operator::Mult | Operator::Div => Precedence::Product,
            Operator::Lt | Operator::Gt => Precedence::LessGreater,
            Operator::Eq | Operator::NEq => Precedence::Equals,
            Operator::Negate | Operator::LNot => Precedence::Prefix,
            Operator::Refer | Operator::Deref => Precedence::RefDeref,
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
    RefDeref,
    Call,
}
