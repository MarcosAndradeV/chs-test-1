

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Identfier {
    name: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IntLiteral {
    pub value: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StrLiteral {
    pub value: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct If {
    condition: Box<Expression>,
    truthy_block: Vec<Stmt>,
    falsy_block: Option<Vec<Stmt>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct While {
    condition: Box<Expression>,
    while_block: Vec<Stmt>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Var {
    name: Identfier,
    value: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Set {
    name: Identfier,
    value: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum OperatorKind {
    Add,
    BitAnd,
    BitOr,
    BitXor,
    Div,
    Eq,
    Ge,
    Gt,
    Le,
    Lt,
    Mod,
    Mul,
    Ne,
    Pow,
    Shl,
    Shr,
    Sub,
    UnsignedShr,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Operator {
    pub kind: OperatorKind,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Binary {
    pub left: Expression,
    pub right: Expression,
    pub operator: Operator,
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression {
    IntLit(Box<IntLiteral>),
    StrLit(Box<StrLiteral>),
    Ident(Box<Identfier>)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Stmt {
    IfStmt(Box<If>),
    WhileStmt(Box<While>),
    ExprStmt(Box<Expression>),
    VarStmt(Box<Var>),
    SetStmt(Box<Set>), 
    PrintStmt(Box<Expression>),
    ReturnStmt(Box<Expression>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FnStmt {
    pub name: String,
    pub arguments: Vec<(Identfier, String)>,
    pub body: Vec<Stmt>,
    pub return_type: Option<String>
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TopLevelStmt {
    Fn(Box<FnStmt>)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Program {
    pub top_level_stmts: Vec<TopLevelStmt>
}


#[cfg(test)]
mod test {
    use super::*;
   #[test]
   fn test(){
    let ast = Program {
        top_level_stmts: vec![
            TopLevelStmt::Fn(Box::new(
                FnStmt { 
                    name: "main".to_string(),
                    arguments: vec![],
                    body: vec![
                        Stmt::PrintStmt(Box::new(
                            Expression::StrLit(Box::new(
                                StrLiteral { value: "Hello, world!".to_string() }
                            ))
                        ))
                    ],
                    return_type: None
                }
            ))
        ]
    };

    println!("{:?}", ast);


   }
}