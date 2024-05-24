use std::fmt;

#[derive(Debug)]
pub enum Operation {
    Pop,
    Dup,
    Swap,
    Over,

    Add,
    Minus,
    Mul,
    Div,
    Mod,

    Eq,
    Neq,
    Gt,
    Gte,
    Lte,
    Lt,

    Land,
    Lor,

    Shl,
    Shr,
    Bitand,
    Bitor,

    Debug,
    Exit,
    Print,
    IdxSet,
    IdxGet,
    Len,
    Concat,
    Tail,
    Head,
    Call,
}

#[derive(Debug)]
pub struct IfExpr {
    pub if_branch: Expressions,
    pub else_branch: Option<Expressions>,
}

#[derive(Debug)]
pub struct WhileExpr {
    pub cond: Expressions,
    pub while_block: Expressions,
}

#[derive(Debug)]
pub struct VarExpr {
    pub name: String,
    pub value: Expressions,
}

#[derive(Debug)]
pub struct PeekExpr {
    pub names: Vec<String>,
    pub body: Expressions,
}

#[derive(Debug)]
pub struct FnExpr {
    pub name: String,
    pub body: Expressions,
}

#[derive(Debug)]
pub struct ListExpr {
    pub itens: Vec<Expr>,
}

#[derive(Debug)]
pub enum Expr {
    Op(Box<Operation>),

    IntExpr(Box<String>),
    StrExpr(Box<String>),
    BoolExpr(Box<String>),
    ListExpr(Box<ListExpr>),
    NilExpr,

    IdentExpr(Box<String>),
    Assigin(Box<String>),
    
    If(Box<IfExpr>),
    Whlie(Box<WhileExpr>),
    Var(Box<VarExpr>),
    Peek(Box<PeekExpr>),
    Fn(Box<FnExpr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Op(_) => write!(f, "Op"),
            Expr::IntExpr(_) => write!(f, "IntExpr"),
            Expr::StrExpr(_) => write!(f, "StrExpr"),
            Expr::BoolExpr(_) => write!(f, "BoolExpr"),
            Expr::NilExpr => write!(f, "NilExpr"),
            Expr::If(_) => write!(f, "If"),
            Expr::Whlie(_) => write!(f, "Whlie"),
            Expr::Var(_) => write!(f, "Var"),
            Expr::Peek(_) => write!(f, "Peek"),
            Expr::ListExpr(_) => write!(f, "ListExpr"),
            Expr::IdentExpr(_) => write!(f, "Identifier"),
            Expr::Assigin(_) => write!(f, "Assigin"),
            Expr::Fn(_) => write!(f, "Fn"),
        }
    }
}

type Expressions = Vec<Expr>;
pub struct Program {
    pub exprs: Expressions,
}

impl IntoIterator for Program {
    type Item = Expr;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.exprs.into_iter()
    }
}
