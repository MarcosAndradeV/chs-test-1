use chs_util::{chs_error, CHSError};

use crate::nodes::Module;

use super::{
    lexer::{Lexer, Token, TokenKind},
    nodes::{
        Expr, FnExpr, IfExpr, LambdaExpr, ListExpr, Operation, PeekExpr, Program, SExpr, WhileExpr,
    },
};

type ResTok = Result<Token, CHSError>;

struct Parser {
    pub lexer: Lexer,
    pub pos: usize,
    pub peeked: Option<Token>,
}

pub fn parse(filename: String, input: Vec<u8>) -> Result<Module, CHSError> {
    let lexer = Lexer::new(input);
    let mut parser = Parser {
        lexer,
        pos: 0,
        peeked: None,
    };
    let mut exprs: Vec<Expr> = Vec::new();
    loop {
        let token = parser.next();

        if token.kind == TokenKind::EOF {
            let program = Program { exprs };
            return Ok(Module {
                filesource: filename,
                program,
            });
        }
        exprs.push(parser.top_level_expression(token)?);
    }
}

impl Parser {
    pub fn top_level_expression(&mut self, token: Token) -> Result<Expr, CHSError> {
        let expr = match token.kind {
            TokenKind::KeyWord if token.val_eq("fn") => self.fn_expr()?,
            _ => chs_error!("Parser Error: {} is not implemeted in top level", token),
        };
        Ok(expr)
    }

    fn expression(&mut self, token: Token) -> Result<Expr, CHSError> {
        let expr = match token.kind {
            TokenKind::Pop => Expr::Op(Box::new(Operation::Pop)),
            TokenKind::Dup => Expr::Op(Box::new(Operation::Dup)),
            TokenKind::Over => Expr::Op(Box::new(Operation::Over)),
            TokenKind::Swap => Expr::Op(Box::new(Operation::Swap)),
            TokenKind::Rot => Expr::Op(Box::new(Operation::Rot)),
            TokenKind::Nop => Expr::Op(Box::new(Operation::Nop)),
            TokenKind::Tilde => self.addr_of_expr()?,

            TokenKind::Plus => Expr::Op(Box::new(Operation::Add)),
            TokenKind::Minus => Expr::Op(Box::new(Operation::Minus)),
            TokenKind::Star => Expr::Op(Box::new(Operation::Mul)),
            TokenKind::Slash => Expr::Op(Box::new(Operation::Div)),
            TokenKind::Percent => Expr::Op(Box::new(Operation::Mod)),

            TokenKind::Eq => Expr::Op(Box::new(Operation::Eq)),
            TokenKind::NEq => Expr::Op(Box::new(Operation::Neq)),
            TokenKind::GtEq => Expr::Op(Box::new(Operation::Gte)),
            TokenKind::Gt => Expr::Op(Box::new(Operation::Gt)),
            TokenKind::LtEq => Expr::Op(Box::new(Operation::Lte)),
            TokenKind::Lt => Expr::Op(Box::new(Operation::Lt)),

            TokenKind::ShL => Expr::Op(Box::new(Operation::Shl)),
            TokenKind::ShR => Expr::Op(Box::new(Operation::Shr)),
            TokenKind::Ampersand => Expr::Op(Box::new(Operation::Bitand)),
            TokenKind::Bor => Expr::Op(Box::new(Operation::Bitor)),
            TokenKind::LAnd => Expr::Op(Box::new(Operation::Land)),
            TokenKind::LOr => Expr::Op(Box::new(Operation::Lor)),
            TokenKind::LNot => Expr::Op(Box::new(Operation::Lnot)),

            TokenKind::Debug => Expr::Op(Box::new(Operation::Debug)),
            TokenKind::Exit => Expr::Op(Box::new(Operation::Exit)),
            TokenKind::Print => Expr::Op(Box::new(Operation::Print)),
            TokenKind::Puts => Expr::Op(Box::new(Operation::Puts)),
            TokenKind::IdxSet => Expr::Op(Box::new(Operation::IdxSet)),
            TokenKind::IdxGet => Expr::Op(Box::new(Operation::IdxGet)),
            TokenKind::Len => Expr::Op(Box::new(Operation::Len)),
            TokenKind::Concat => Expr::Op(Box::new(Operation::Concat)),
            TokenKind::Head => Expr::Op(Box::new(Operation::Head)),
            TokenKind::Tail => Expr::Op(Box::new(Operation::Tail)),
            TokenKind::Call => Expr::Op(Box::new(Operation::Call)),
            TokenKind::DollarSing => Expr::Op(Box::new(Operation::StackSize)),

            TokenKind::String => Expr::StrExpr(Box::new(token.value)),
            TokenKind::Interger => Expr::IntExpr(Box::new(token.value)),
            TokenKind::Char => Expr::CharExpr(Box::new(token.value)),
            TokenKind::True | TokenKind::False => Expr::BoolExpr(Box::new(token.value)),
            TokenKind::Nil => Expr::NilExpr,
            TokenKind::Ident => Expr::IdentExpr(Box::new(token.value)),
            TokenKind::Error => Expr::ErrorExpr(Box::new(format!(
                "{} {:?}",
                self.expect(TokenKind::String)?.value,
                token.loc
            ))),

            TokenKind::KeyWord if token.val_eq("if") => self.if_expr()?,
            //TokenKind::KeyWord if token.val_eq("let") => self.let_expr()?,
            TokenKind::KeyWord if token.val_eq("while") => self.while_expr()?,
            TokenKind::Assigin => self.assigin_expr()?,
            TokenKind::BracketOpen => self.list_expr()?,
            TokenKind::KeyWord if token.val_eq("peek") => self.peek_expr()?,
            TokenKind::KeyWord if token.val_eq("fn") => self.lambda_expr()?,
            TokenKind::ParenOpen => self.s_expr()?,

            _ => chs_error!("Parser Error: {} is not implemeted", token),
        };
        Ok(expr)
    }

    fn addr_of_expr(&mut self) -> Result<Expr, CHSError> {
        let name = self.expect(TokenKind::Ident)?.value;
        Ok(Expr::AddrOf(name.into()))
    }

    fn lambda_expr(&mut self) -> Result<Expr, CHSError> {
        self.expect(TokenKind::CurlyOpen)?;
        let mut body = vec![];
        loop {
            let tok = self.require()?;
            match tok.kind {
                TokenKind::CurlyClose => break,
                _ => body.push(self.expression(tok)?),
            }
        }
        Ok(Expr::LambdaExpr(Box::new(LambdaExpr { body })))
    }

    fn s_expr(&mut self) -> Result<Expr, CHSError> {
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
        Ok(Expr::SExpr(Box::new(SExpr { func, args })))
    }

    fn fn_expr(&mut self) -> Result<Expr, CHSError> {
        let name = self.expect(TokenKind::Ident)?.value;
        self.expect(TokenKind::CurlyOpen)?;
        let mut body = vec![];
        loop {
            let tok = self.require()?;
            match tok.kind {
                TokenKind::CurlyClose => break,
                _ => body.push(self.expression(tok)?),
            }
        }
        Ok(Expr::Fn(Box::new(FnExpr { name, body })))
    }

    fn peek_expr(&mut self) -> Result<Expr, CHSError> {
        let mut names = vec![];
        loop {
            let tok = self.require()?;
            match tok.kind {
                TokenKind::CurlyOpen => {
                    if names.len() == 0 {
                        chs_error!("Parser Error: Peek expect at least 1 identifier.")
                    }
                    break;
                }
                TokenKind::Ident => names.push(tok.value),
                _ => chs_error!(""),
            }
        }
        let mut body = vec![];
        loop {
            let tok = self.require()?;
            match tok.kind {
                TokenKind::CurlyClose => break,
                TokenKind::KeyWord if tok.val_eq("fn") => {
                    chs_error!("Parser Error: Cannot create {} inside peek block", tok)
                }
                _ => body.push(self.expression(tok)?),
            }
        }

        Ok(Expr::Peek(Box::new(PeekExpr { names, body })))
    }

    fn assigin_expr(&mut self) -> Result<Expr, CHSError> {
        let name = self.expect(TokenKind::Ident)?.value;
        Ok(Expr::Assigin(Box::new(name)))
    }

    fn list_expr(&mut self) -> Result<Expr, CHSError> {
        let mut itens = vec![];
        loop {
            let token = self.require()?;
            match token.kind {
                TokenKind::BracketClose => break,
                TokenKind::KeyWord => chs_error!(
                    "Parser Error: {:?}({}) is not suported in List literals",
                    token.kind,
                    token.value
                ),
                _ => itens.push(self.expression(token)?),
            }
        }
        Ok(Expr::ListExpr(Box::new(ListExpr { itens })))
    }

    fn if_expr(&mut self) -> Result<Expr, CHSError> {
        self.expect(TokenKind::CurlyOpen)?;
        let mut if_branch: Vec<Expr> = Vec::new();
        let mut else_branch: Vec<Expr> = Vec::new();
        loop {
            let tok = self.require()?;
            match tok.kind {
                TokenKind::CurlyClose => break,
                _ => if_branch.push(self.expression(tok)?),
            }
        }
        if self.peek().val_eq("else") {
            self.next();
            self.expect(TokenKind::CurlyOpen)?;
            loop {
                let tok = self.require()?;
                match tok.kind {
                    TokenKind::CurlyClose => break,
                    _ => else_branch.push(self.expression(tok)?),
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

    fn while_expr(&mut self) -> Result<Expr, CHSError> {
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

        chs_error!(
            "Expect token {:?} at {}, but got {}",
            kind,
            token.loc,
            self.peek()
        )
    }

    fn next(&mut self) -> Token {
        loop {
            self.pos += 1;
            let token = self
                .peeked
                .take()
                .unwrap_or_else(|| self.lexer.get_next_token());

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
        if matches!(tok.kind, TokenKind::Invalid | TokenKind::EOF) {
            chs_error!("Parser Error: require Valid Token got {}", tok);
        }
        Ok(tok)
    }
}
