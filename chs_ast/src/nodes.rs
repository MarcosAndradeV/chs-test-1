use core::fmt;

use chs_lexer::Token;
use chs_util::{chs_error, CHSError, Loc};

use crate::types::CHSType;

#[derive(Debug, Default)]
pub struct Module {
    pub name: String,
    pub decls: Vec<Decl>,
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Module: {}", self.name)?;
        for expr in &self.decls {
            writeln!(f, " {expr} ")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Decl {
    Function(Function),
    TypeDecl(String, CHSType),
    ConstDecl(ConstDecl),
}

impl fmt::Display for Decl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Decl::Function(v) => write!(f, "{v}"),
            Decl::TypeDecl(n, v) => write!(f, "{n} : {v}"),
            Decl::ConstDecl(v) => write!(f, "{v}"),
        }
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
    Call(Box<Call>),
    VarDecl(Box<VarDecl>),
    Group(Box<Self>),
    Ref(Box<Self>),
    Deref(Box<Self>),
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
            Expression::VarDecl(v) => write!(f, "{v}"),
            Expression::Group(v) => write!(f, "({v})"),
            Expression::Ref(v) => write!(f, "&{v}"),
            Expression::Deref(v) => write!(f, "*{v}"),
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
