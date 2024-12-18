use chs_util::{chs_error, CHSError, Loc};

use crate::lexer::{Token, TokenKind};
use std::fmt;

#[derive(Debug)]
pub struct Module {
    pub filesource: String,
    pub program: Vec<Expression>,
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Module \"{}\" {{", self.filesource)?;
        write!(f, " ")?;
        for expr in &self.program {
            write!(f, "{expr} ")?;
        }
        write!(f, "}}")
    }
}

#[derive(Debug)]
pub enum TopLevelExpr {
    Expr(Expression),
}

type Expressions = Vec<Expression>;

#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    Group(Group),
    WhileExpr(WhileExpr),
    IfExpr(IfExpr),
    WordExpr(WordExpr),
    AssignExpr(AssignExpr)
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Literal(literal) => write!(f, "{literal}"),
            Expression::Group(group) => write!(f, "({})", group.value.as_ref()),
            Expression::WhileExpr(while_expr) => write!(f, "{while_expr}"),
            Expression::IfExpr(if_expr) => write!(f, "{if_expr}"),
            Expression::WordExpr(WordExpr{loc: _, op}) => write!(f, "{op}"),
            Expression::AssignExpr(assign_expr) => write!(f, "{assign_expr}"),
        }
    }
}

impl Expression {

    pub fn from_assign_token_no_type(value: Token) -> Self {
        Expression::AssignExpr(AssignExpr { loc: value.loc, name: value.value, type_: "".to_string() })
    }

    pub fn from_integer_token(value: Token) -> Self {
        let literal = value
            .value
            .parse::<i64>()
            .expect("Invalid token value for \"from_integer_token\". Probably a error in lexer");
        Expression::Literal(Literal::Number(NumberLiteral {
            loc: value.loc,
            value: literal,
        }))
    }

    pub fn from_boolean_token(value: Token) -> Self {
        let literal = value
            .value
            .parse::<bool>()
            .expect("TODO: Error in lexer probably");
        Expression::Literal(Literal::Boolean(BooleanLiteral {
            loc: value.loc,
            value: literal,
        }))
    }

    pub fn from_nil_token(value: Token) -> Self {
        Expression::Literal(Literal::Nil(NilLiteral { loc: value.loc }))
    }

    pub fn from_word_token(token: Token) -> Result<Self, CHSError> {
        if token.kind != TokenKind::Word { chs_error!("Invalid token kind for \"from_word_token\". Probably a error in parser") }
        #[rustfmt::skip]
        let e = match token.value.as_str() {
            "dup"  => WordExpr  { loc: token.loc, op:  Word::Dup  },
            "rot"  => WordExpr  { loc: token.loc, op:  Word::Rot  },
            "over" => WordExpr  { loc: token.loc, op:  Word::Over },
            "swap" => WordExpr  { loc: token.loc, op:  Word::Swap },
            "drop" => WordExpr  { loc: token.loc, op:  Word::Drop },
            "not"  => WordExpr  { loc: token.loc, op:  Word::LogicalNot },
            "-"    => WordExpr  { loc: token.loc, op:  Word::Minus },
            "+"    => WordExpr  { loc: token.loc, op:  Word::Plus },
            "*"    => WordExpr  { loc: token.loc, op:  Word::Star },
            "/"    => WordExpr  { loc: token.loc, op:  Word::Slash },
            "="    => WordExpr  { loc: token.loc, op:  Word::Eq },
            "!="   => WordExpr  { loc: token.loc, op:  Word::NEq },
            "<"    => WordExpr  { loc: token.loc, op:  Word::Lt },
            ">"    => WordExpr  { loc: token.loc, op:  Word::Gt },
            _      => WordExpr  { loc: token.loc, op:  Word::Ident(token.value) }
        };

        Ok(Expression::WordExpr(e))
    }
}

impl Default for Expression {
    fn default() -> Self {
        Expression::Literal(Literal::Nil(NilLiteral {
            loc: Loc::default(),
        }))
    }
}

#[derive(Debug)]
pub struct IfExpr {
    pub loc: Loc,
    pub if_branch: Expressions,
    pub else_branch: Option<Expressions>,
}

impl fmt::Display for IfExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "if {{")?;
        for expr in &self.if_branch {
            write!(f, " {expr} ")?;
        }
        write!(f, "}}")?;
        if let Some(else_) = &self.else_branch {
            write!(f, "else {{")?;
            for expr in else_ {
                write!(f, " {expr} ")?;
            }
            write!(f, "}}")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct WhileExpr {
    pub loc: Loc,
    pub cond: Box<Expression>,
    pub body: Expressions,
}

impl fmt::Display for WhileExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "while {} {{", self.cond)?;
        for expr in &self.body {
            write!(f, " {expr} ")?;
        }
        write!(f, "}}")
    }
}

#[derive(Debug)]
pub struct AssignExpr {
    pub loc: Loc,
    pub name: String,
    pub type_: String,
}

impl fmt::Display for AssignExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, ":= {}", self.name)
    }
}

#[derive(Debug)]
pub struct Group {
    pub loc: Loc,
    pub value: Box<Expression>,
}


#[derive(Debug)]
pub struct NumberLiteral {
    pub loc: Loc,
    pub value: i64,
}

#[derive(Debug)]
pub struct BooleanLiteral {
    pub loc: Loc,
    pub value: bool,
}

#[derive(Debug)]
pub enum Literal {
    Number(NumberLiteral),
    Boolean(BooleanLiteral),
    Nil(NilLiteral),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Number(number_literal) => write!(f, "{}", number_literal.value),
            Literal::Boolean(boolean_literal) => write!(f, "{}", boolean_literal.value),
            Literal::Nil(_) => write!(f, "nil",),
        }
    }
}

#[derive(Debug)]
pub struct NilLiteral {
    pub loc: Loc,
}



#[allow(dead_code)]
#[derive(Debug)]
pub struct WordExpr {
    loc: Loc,
    op: Word,
}

#[derive(Debug)]
pub enum Word {
    Drop,
    Over,
    Dup,
    Rot,
    Swap,
    LogicalNot,
    Eq,
    NEq,
    Lt,
    Gt,
    Plus,
    Minus,
    Slash,
    Star,
    Ident(String)
}

impl fmt::Display for Word {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Word::Drop       => write!(f, "drop"),
            Word::Over       => write!(f, "over"),
            Word::Dup        => write!(f, "dup"),
            Word::Rot        => write!(f, "rot"),
            Word::Swap       => write!(f, "swap"),
            Word::LogicalNot => write!(f, "!"),
            Word::Eq         => write!(f, "=="),
            Word::NEq        => write!(f, "!="),
            Word::Lt         => write!(f, "<"),
            Word::Gt         => write!(f, ">"),
            Word::Plus       => write!(f, "+"),
            Word::Minus      => write!(f, "-"),
            Word::Slash      => write!(f, "/"),
            Word::Star       => write!(f, "*"),
            Word::Ident(s)   => write!(f, "{s}"),
        }
    }
}
