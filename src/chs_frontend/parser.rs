use crate::{exeptions::GenericError, generic_error};

use super::{
    ast::{Expr, FnExpr, IfExpr, LambdaExpr, ListExpr, Operation, PeekExpr, Program, SExpr, VarExpr, WhileExpr},
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
            TokenKind::Rot => Expr::Op(Box::new(Operation::Rot)),
            TokenKind::Nop => Expr::Op(Box::new(Operation::Nop)),

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
            TokenKind::Lnot => Expr::Op(Box::new(Operation::Lnot)),

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
            TokenKind::Identifier => {
                if self.peek().kind == TokenKind::Assing {
                    self.var_expr(token.value)?
                } else {
                    Expr::IdentExpr(Box::new(token.value))
                }
            },

            TokenKind::If => self.if_expr()?,
            TokenKind::Whlie => self.while_expr()?,
            //TokenKind::Var => self.var_expr()?,
            TokenKind::Assing => self.assigin_expr()?,
            TokenKind::BracketOpen => self.list_expr()?,
            TokenKind::Peek => self.peek_expr()?,
            TokenKind::Fn => self.fn_expr()?,
            TokenKind::ParenOpen => self.s_expr()?,
            TokenKind::Tilde => self.lambda_expr()?,

            _ => generic_error!("Parser Error: {} is not implemeted", token),
        };
        Ok(expr)
    }

    fn lambda_expr(&mut self) -> Result<Expr, GenericError>{
        let ftoken = self.require()?;
        let body = self.expression(ftoken)?;
        Ok(Expr::LambdaExpr(Box::new(LambdaExpr{body})))
    }

    fn s_expr(&mut self) -> Result<Expr, GenericError>{
        let ftoken = self.require()?;
        let func = self.expression(ftoken)?;
        let mut args = vec![];
        loop {
            let tok = self.require()?;
            match tok.kind {
                TokenKind::ParenClose => break,
                _ => args.push(self.expression(tok)?),
            }
        }
        Ok(Expr::SExpr(Box::new(SExpr{func, args})))
    }

    fn fn_expr(&mut self) -> Result<Expr, GenericError> {
        let name = self.expect(TokenKind::Identifier)?.value;
        let mut body = vec![];
        self.expect(TokenKind::CurlyOpen)?;
        loop {
            let tok = self.require()?;
            match tok.kind {
                TokenKind::CurlyClose => break,
                TokenKind::Fn => {
                    generic_error!("Parser Error: Cannot create {} inside peek block", tok.get_kind())
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
                        generic_error!("Parser Error: Peek expect at least 1 identifier.")
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
                    generic_error!("Parser Error: Cannot create {} inside peek block", tok)
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
                    "Parser Error: {:?}({}) is not suported in List literals",
                    token.kind,
                    token.value
                ),
                _ => list.push(self.expression(token)?),
            }
        }
        Ok(Expr::ListExpr(Box::new(ListExpr { itens: list })))
    }

    fn var_expr(&mut self, name: String) -> Result<Expr, GenericError> {
        self.expect(TokenKind::Assing)?;
        let mut value: Vec<Expr> = Vec::new();
        loop {
            let tok = self.require()?;
            match tok.kind {
                TokenKind::SemiColon => {
                    break Ok(Expr::Var(Box::new(VarExpr { name, value, dtype: None})));
                },
                TokenKind::DoubleColon => {
                    break Ok(Expr::Var(Box::new(VarExpr { name, value, dtype: Some(self.expect(TokenKind::Identifier)?.value)})));
                },
                _ => value.push(self.expression(tok)?),
            }
        }
    }

    fn if_expr(&mut self) -> Result<Expr, GenericError> {
        self.expect(TokenKind::CurlyOpen)?;
        let mut if_branch: Vec<Expr> = Vec::new();
        let mut else_branch: Vec<Expr> = Vec::new();
        loop {
            let tok = self.require()?;
            match tok.kind {
                TokenKind::CurlyClose => break,
                _ => if_branch.push(self.expression(tok)?)
            }
        }
        if TokenKind::Else == self.peek().kind {
            self.next();
            self.expect(TokenKind::CurlyOpen)?;
            loop {
                let tok = self.require()?;
                match tok.kind {
                    TokenKind::CurlyClose => break,
                    _ => else_branch.push(self.expression(tok)?)
                }
            }
            Ok(Expr::If(Box::new(IfExpr {
                if_branch,
                else_branch: Some(else_branch),
            })))
        } else {
            Ok(Expr::If(Box::new(IfExpr {
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

        generic_error!("Parser Error: Expect {:?} at {}", kind, self.pos)
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
            generic_error!("Parser Error: require {:?}[{}] {}", tok.kind, tok.value, self.pos);
        }
        Ok(tok)
    }
}
