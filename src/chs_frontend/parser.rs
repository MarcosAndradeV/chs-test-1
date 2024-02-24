use crate::{exeptions::GenericError, generic_error};

use super::{
    ast::{Expr, FnExpr, IfExpr, ListExpr, Operation, PeekExpr, Program, VarExpr, WhileExpr},
    lexer::{Lexer, Token, TokenKind},
};

type ResTok = Result<Token, GenericError>;

pub struct Parser {
    lexer: Lexer,
    pos: usize,
    peeked: Option<Token>,
}

impl Parser {
    pub fn new(input: Vec<u8>) -> Self {
        let lexer = Lexer::new(input);
        Self {
            lexer,
            pos: 0,
            peeked: None,
        }
    }

    pub fn parse_to_ast(&mut self) -> Result<Program, GenericError> {
        let mut exprs: Vec<Expr> = Vec::new();
        loop {
            let token = self.next();

            if token.kind == TokenKind::Null {
                let program = Program { exprs };
                return Ok(program);
            }
            exprs.push(self.expression(token)?);
        }
    }

    fn expression(&mut self, token: Token) -> Result<Expr, GenericError> {
        let expr = match token.kind {
            TokenKind::Pop => Expr::Op(Box::new(Operation::Pop)),
            TokenKind::Dup => Expr::Op(Box::new(Operation::Dup)),
            TokenKind::Over => Expr::Op(Box::new(Operation::Over)),
            TokenKind::Swap => Expr::Op(Box::new(Operation::Swap)),

            TokenKind::Add => Expr::Op(Box::new(Operation::Add)),
            TokenKind::Minus => Expr::Op(Box::new(Operation::Minus)),
            TokenKind::Mul => Expr::Op(Box::new(Operation::Mul)),
            TokenKind::Div => Expr::Op(Box::new(Operation::Div)),
            TokenKind::Mod => Expr::Op(Box::new(Operation::Mod)),

            TokenKind::Eq => Expr::Op(Box::new(Operation::Eq)),
            TokenKind::Neq => Expr::Op(Box::new(Operation::Neq)),
            TokenKind::Gte => Expr::Op(Box::new(Operation::Gte)),
            TokenKind::Gt => Expr::Op(Box::new(Operation::Gt)),
            TokenKind::Lte => Expr::Op(Box::new(Operation::Lte)),
            TokenKind::Lt => Expr::Op(Box::new(Operation::Lt)),

            TokenKind::Shl => Expr::Op(Box::new(Operation::Shl)),
            TokenKind::Shr => Expr::Op(Box::new(Operation::Shr)),
            TokenKind::Bitand => Expr::Op(Box::new(Operation::Bitand)),
            TokenKind::Bitor => Expr::Op(Box::new(Operation::Bitor)),
            TokenKind::Land => Expr::Op(Box::new(Operation::Land)),
            TokenKind::Lor => Expr::Op(Box::new(Operation::Lor)),

            TokenKind::Debug => Expr::Op(Box::new(Operation::Debug)),
            TokenKind::Exit => Expr::Op(Box::new(Operation::Exit)),
            TokenKind::Print => Expr::Op(Box::new(Operation::Print)),
            TokenKind::IdxSet => Expr::Op(Box::new(Operation::IdxSet)),
            TokenKind::IdxGet => Expr::Op(Box::new(Operation::IdxGet)),
            TokenKind::Len => Expr::Op(Box::new(Operation::Len)),
            TokenKind::Concat => Expr::Op(Box::new(Operation::Concat)),
            TokenKind::Head => Expr::Op(Box::new(Operation::Head)),
            TokenKind::Tail => Expr::Op(Box::new(Operation::Tail)),
            TokenKind::Call => Expr::Op(Box::new(Operation::Call)),

            TokenKind::Str => Expr::StrExpr(Box::new(token.value)),
            TokenKind::Int => Expr::IntExpr(Box::new(token.value)),
            TokenKind::True | TokenKind::False => Expr::BoolExpr(Box::new(token.value)),
            TokenKind::Nil => Expr::NilExpr,
            TokenKind::Identifier => Expr::IdentExpr(Box::new(token.value)),

            TokenKind::If => self.if_expr()?,
            TokenKind::Whlie => self.while_expr()?,
            TokenKind::Var => self.var_expr()?,
            TokenKind::Assing => self.assigin_expr()?,
            TokenKind::BracketOpen => self.list_expr()?,
            TokenKind::Peek => self.peek_expr()?,
            TokenKind::Fn => self.fn_expr()?,
            TokenKind::Tilde => self.nocall_expr()?,

            _ => generic_error!("{} is not implemeted", token),
        };
        Ok(expr)
    }

    fn nocall_expr(&mut self) -> Result<Expr, GenericError> {
        let f = self.expect(TokenKind::Identifier)?.value;
        Ok(Expr::NoCall(Box::new(f)))
    }

    fn fn_expr(&mut self) -> Result<Expr, GenericError> {
        let name = self.expect(TokenKind::Identifier)?.value;
        let mut body = vec![];
        self.expect(TokenKind::CurlyOpen)?;
        loop {
            let tok = self.require()?;
            match tok.kind {
                TokenKind::CurlyClose => break,
                TokenKind::Var | TokenKind::Fn => {
                    generic_error!("Cannot create {} inside peek block", tok)
                }
                _ => body.push(self.expression(tok)?),
            }
        }
        Ok(Expr::Fn(Box::new(FnExpr { name, body })))
    }

    fn peek_expr(&mut self) -> Result<Expr, GenericError> {
        let mut names = vec![];
        loop {
            let tok = self.require()?;
            match tok.kind {
                TokenKind::CurlyOpen => {
                    if names.len() == 0 {
                        generic_error!("Peek expect at least 1 identifier.")
                    }
                    break;
                }
                TokenKind::Identifier => names.push(tok.value),
                _ => generic_error!(""),
            }
        }
        let mut body = vec![];
        loop {
            let tok = self.require()?;
            match tok.kind {
                TokenKind::CurlyClose => break,
                TokenKind::Var | TokenKind::Fn => {
                    generic_error!("Cannot create {} inside peek block", tok)
                }
                _ => body.push(self.expression(tok)?),
            }
        }

        Ok(Expr::Peek(Box::new(PeekExpr { names, body })))
    }

    fn assigin_expr(&mut self) -> Result<Expr, GenericError> {
        let name = self.expect(TokenKind::Identifier)?.value;
        Ok(Expr::Assigin(Box::new(name)))
    }

    fn list_expr(&mut self) -> Result<Expr, GenericError> {
        let mut list = vec![];
        loop {
            let token = self.require()?;
            match token.kind {
                TokenKind::BracketClose => break,
                TokenKind::If
                | TokenKind::Whlie
                | TokenKind::Fn
                | TokenKind::Var => generic_error!(
                    "{:?}({}) is not suported in List literals",
                    token.kind,
                    token.value
                ),
                _ => list.push(self.expression(token)?),
            }
        }
        Ok(Expr::ListExpr(Box::new(ListExpr { itens: list })))
    }

    fn var_expr(&mut self) -> Result<Expr, GenericError> {
        let name = self.expect(TokenKind::Identifier)?.value;
        // type
        self.expect(TokenKind::Assing)?;
        let mut value: Vec<Expr> = Vec::new();
        loop {
            let tok = self.require()?;
            match tok.kind {
                TokenKind::SemiColon => break,
                _ => value.push(self.expression(tok)?),
            }
        }
        Ok(Expr::Var(Box::new(VarExpr { name, value })))
    }

    fn if_expr(&mut self) -> Result<Expr, GenericError> {
        let mut cond: Vec<Expr> = Vec::new();
        loop {
            // condition
            let tok = self.require()?;
            match tok.kind {
                TokenKind::CurlyOpen => {
                    break;
                }
                _ => cond.push(self.expression(tok)?),
            }
        }
        let mut if_branch: Vec<Expr> = Vec::new();
        let mut else_branch: Vec<Expr> = Vec::new();
        let mut has_else: bool = false;
        loop {
            let tok = self.require()?;
            match tok.kind {
                TokenKind::CurlyClose => break,
                TokenKind::Else => {
                    has_else = true;
                }
                _ => {
                    if has_else {
                        else_branch.push(self.expression(tok)?)
                    } else {
                        if_branch.push(self.expression(tok)?)
                    }
                }
            }
        }
        if has_else {
            Ok(Expr::If(Box::new(IfExpr {
                cond,
                if_branch,
                else_branch: Some(else_branch),
            })))
        } else {
            Ok(Expr::If(Box::new(IfExpr {
                cond,
                if_branch,
                else_branch: None,
            })))
        }
    }

    fn while_expr(&mut self) -> Result<Expr, GenericError> {
        let mut cond: Vec<Expr> = Vec::new();
        loop {
            // condition
            let tok = self.require()?;
            match tok.kind {
                TokenKind::CurlyOpen => {
                    break;
                }
                _ => cond.push(self.expression(tok)?),
            }
        }
        let mut while_block: Vec<Expr> = Vec::new();
        loop {
            let tok = self.require()?;
            match tok.kind {
                TokenKind::CurlyClose => break,
                _ => while_block.push(self.expression(tok)?),
            }
        }
        Ok(Expr::Whlie(Box::new(WhileExpr { cond, while_block })))
    }

    fn expect(&mut self, kind: TokenKind) -> ResTok {
        let token = self.next();

        if token.kind == kind {
            return Ok(token);
        }

        generic_error!("Expect {:?} at {}", kind, self.pos)
    }

    fn next(&mut self) -> Token {
        loop {
            self.pos += 1;
            let token = self
                .peeked
                .take()
                .unwrap_or_else(|| self.lexer.next_token());

            match token.kind {
                TokenKind::Comment | TokenKind::Whitespace => {}
                _ => return token,
            }
        }
    }

    #[allow(dead_code)]
    fn peek(&mut self) -> &Token {
        if self.peeked.is_none() {
            self.peeked = Some(self.next());
        }

        self.peeked.as_ref().unwrap()
    }

    fn require(&mut self) -> ResTok {
        let tok = self.next();
        if matches!(tok.kind, TokenKind::Invalid | TokenKind::Null) {
            generic_error!("require {:?}[{}] {}", tok.kind, tok.value, self.pos);
        }
        Ok(tok)
    }
}
