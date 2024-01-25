use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    compiler::ir::IfExpr, exeptions::GenericError, generic_error, instructions::{Bytecode, Instr, Opcode}, value::{List, Value}
};

use super::{ir::{BuildinOp, Expr, FuncExpr, ListLiteral, Operation, Program, VarExpr, WhileExpr}, lexer::{Lexer, Token, TokenKind}};

type ResTok = Result<Token, GenericError>;

pub struct Parser {
    lexer: Lexer,
    consts: Vec<Value>,
    instrs: Vec<Instr>,
    pos: usize,
    peeked: Option<Token>,
    consts_def: HashMap<String, Token>,
    func_def: HashMap<String, usize>,
    var_def: HashMap<String, usize>,
    var_count: usize,
}

impl Parser {
    pub fn new(input: Vec<u8>) -> Self {
        let lexer = Lexer::new(input);

        Self {
            lexer,
            consts: Vec::new(),
            instrs: Vec::new(),
            pos: 0,
            peeked: None,
            consts_def: HashMap::new(),
            func_def: HashMap::new(),
            var_def: HashMap::new(),
            var_count: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Bytecode, GenericError> {
        loop {
            let token = self.next();

            if token.kind == TokenKind::Null {
                return Ok(Bytecode::new(self.instrs.clone(), self.consts.clone()));
            }
            self.parse_all(token, 0)?;
        }
    }

    pub fn parse_to_ir(&mut self) -> Result<Program, GenericError> {
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
            TokenKind::Pop  => Expr::Op(Box::new(Operation::Pop)),
            TokenKind::Dup  => Expr::Op(Box::new(Operation::Dup)),
            TokenKind::Dup2 => Expr::Op(Box::new(Operation::Dup2)),
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

            TokenKind::Print => Expr::Buildin(Box::new(BuildinOp::Print)),
            TokenKind::Println => Expr::Buildin(Box::new(BuildinOp::Println)),
            TokenKind::Debug => Expr::Buildin(Box::new(BuildinOp::Debug)),
            TokenKind::Len => Expr::Buildin(Box::new(BuildinOp::Len)),
            TokenKind::IdxGet => Expr::Buildin(Box::new(BuildinOp::IdxGet)),
            TokenKind::IdxSet => Expr::Buildin(Box::new(BuildinOp::IdxSet)),

            TokenKind::Str => Expr::StrExpr(Box::new(token.value)),
            TokenKind::Int => Expr::IntExpr(Box::new(token.value)),
            TokenKind::Identifier => Expr::IdentExpr(Box::new(token.value)),

            TokenKind::If => self.if_expr()?,
            TokenKind::Whlie => self.while_expr()?,
            TokenKind::Var => self.var_expr()?,
            TokenKind::Assing => self.assigin_expr()?,
            TokenKind::ParenOpen => self.list_expr()?,
            TokenKind::Func => self.func()?,

            _ => generic_error!("{} is not implemeted", token),
        };
        Ok(expr)
    }

    fn func(&mut self) -> Result<Expr, GenericError> {
        let name = self.expect(TokenKind::Identifier)?.value;
        let mut func_block = Vec::new();
        self.expect(TokenKind::CurlyOpen)?;
        loop {
            let tok = self.require()?;
            match tok.kind {
                TokenKind::CurlyClose => break,
                _ => func_block.push(self.expression(tok)?),
            }
        }
        Ok(Expr::Func(Box::new(
            FuncExpr {
                name,
                func_block
            }
        )))
    }

    fn assigin_expr(&mut self) -> Result<Expr, GenericError> {
        let name = self.expect(TokenKind::Identifier)?.value;
        Ok(Expr::Assigin(Box::new(
            name
        )))
    }

    fn list_expr(&mut self) -> Result<Expr, GenericError> {
        let mut list = vec![];
        loop {
            let tok = self.require()?;
            match tok.kind {
                TokenKind::ParenClose => break,
                TokenKind::Int => {
                    list.push(Rc::new(Value::Int64(tok.value.parse().unwrap())));
                }
                _ => generic_error!(
                    "{:?}({}) is not suported in List literals",
                    tok.kind,
                    tok.value
                ),
            }
        }
        Ok(Expr::ListExpr(Box::new(
            ListLiteral {
                value: List { elem: list }
            }
        )))
    }

    fn var_expr(&mut self) -> Result<Expr, GenericError> {
        let name = self.expect(TokenKind::Identifier)?.value;
        let mut value: Vec<Expr> = Vec::new();
        loop {
            let tok = self.require()?;
            match tok.kind {
                TokenKind::SemiColon => break,
                _ => value.push(self.expression(tok)?),
            }
        }
        Ok(Expr::Var(Box::new(
            VarExpr {
                name,
                value,
            }
        )))
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
                TokenKind::Func | TokenKind::Directive => generic_error!("You cannot declareate {} here!", tok.value),
                _ => {
                    if has_else {
                        else_branch.push(self.expression(tok)?)
                    } else {
                        if_branch.push(self.expression(tok)?)
                    }
                },
            }
        }
        if has_else {
            Ok(Expr::If(Box::new(
                IfExpr {
                    cond,
                    if_branch,
                    else_branch: Some(else_branch)
                }
            )))
        } else {
            Ok(Expr::If(Box::new(
                IfExpr {
                    cond,
                    if_branch,
                    else_branch: None
                }
            )))
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
        Ok(Expr::Whlie(Box::new(
            WhileExpr { cond, while_block }
        )))
    }

    fn expect(&mut self, kind: TokenKind) -> ResTok {
        let token = self.next();

        if token.kind == kind {
            return Ok(token);
        }

        generic_error!("Expect {:?} at {}", kind, self.pos)
    }

    fn expect_or(&mut self, kind: TokenKind, kind_2: TokenKind) -> ResTok {
        let token = self.next();

        if token.kind == kind || token.kind == kind_2 {
            return Ok(token);
        }

        generic_error!("Expect {:?} or {:?} at {}", kind, kind_2, self.pos)
    }

    fn name_is_defined(&mut self) -> ResTok {
        let token: Token = self.expect(TokenKind::Identifier)?;
        if self.consts_def.get(&token.value).is_some() {
            generic_error!("{} is already defined as const", token.value);
        }
        if self.var_def.get(&token.value).is_some() {
            generic_error!("{} is already defined as variable", token.value);
        }
        if self.func_def.get(&token.value).is_some() {
            generic_error!("{} is already defined as function", token.value);
        }
        Ok(token)
    }

    fn name_is_valid_variable(&mut self) -> ResTok {
        let token: Token = self.expect(TokenKind::Identifier)?;
        if self.consts_def.get(&token.value).is_some() {
            generic_error!("{} is already defined as const", token.value);
        }
        if self.func_def.get(&token.value).is_some() {
            generic_error!("{} is already defined as function", token.value);
        }
        Ok(token)
    }

    #[allow(dead_code)]
    fn not_expect(&mut self, kind: TokenKind) -> ResTok {
        let token = self.next();

        if token.kind != kind {
            return Ok(token);
        }

        generic_error!("Not Expect {:?} at {}", kind, self.pos)
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

    fn while_block(&mut self, d: usize) -> Result<(), GenericError> {
        self.instrs.push(Instr::new(Opcode::PushLabel, Some(3)));
        let whileaddrs = self.instrs.len();
        let ifoffset: usize;
        loop {
            // condition
            let tok = self.require()?;
            match tok.kind {
                TokenKind::CurlyOpen => {
                    ifoffset = self.instrs.len();
                    break;
                }
                TokenKind::Identifier => self.identfier(tok)?,
                TokenKind::BracketOpen => self.index_get()?,
                _ => self.parse_one(tok)?,
            }
        }
        loop {
            let tok = self.require()?;
            match tok.kind {
                TokenKind::CurlyClose => {
                    self.instrs
                        .push(Instr::new(Opcode::Jmp, Some(whileaddrs + d)));
                    self.instrs.insert(
                        ifoffset,
                        Instr::new(Opcode::JmpIf, Some(self.instrs.len() + 1 + d)),
                    );
                    self.instrs.push(Instr::new(Opcode::DropLabel, Some(3)));
                    break;
                }
                TokenKind::Directive => generic_error!("You cannot declareate {} here!", tok.value),
                _ => self.parse_all(tok, d + 1)?,
            }
        }
        Ok(())
    }

    fn if_block(&mut self, d: usize) -> Result<(), GenericError> {
        self.instrs.push(Instr::new(Opcode::PushLabel, Some(1)));
        let offset;
        let mut offset2 = 0;
        let mut has_else = false;
        loop {
            // condition
            let tok = self.require()?;
            match tok.kind {
                TokenKind::CurlyOpen => {
                    offset = self.instrs.len();
                    break;
                }
                TokenKind::Identifier => self.identfier(tok)?,
                TokenKind::BracketOpen => self.index_get()?,
                _ => self.parse_one(tok)?,
            }
        }
        loop {
            let tok = self.require()?;
            match tok.kind {
                TokenKind::CurlyClose => break,
                TokenKind::Else => {
                    self.instrs.push(Instr::new(Opcode::PushLabel, Some(2)));
                    has_else = true;
                    offset2 = self.instrs.len() + 1;
                    self.instrs.insert(
                        offset,
                        Instr::new(Opcode::JmpIf, Some(self.instrs.len() + 2 + d)),
                    );
                }
                TokenKind::Directive => generic_error!("You cannot declareate {} here!", tok.value),
                _ => self.parse_all(tok, d + 1)?,
            }
        }
        if !has_else {
            self.instrs.insert(
                offset,
                Instr::new(Opcode::JmpIf, Some(self.instrs.len() + 1 + d)),
            );
            self.instrs.push(Instr::new(Opcode::DropLabel, Some(1)));
            return Ok(());
        }
        if has_else {
            self.instrs.insert(
                offset2,
                Instr::new(Opcode::Jmp, Some(self.instrs.len() + 1 + d)),
            );
        }
        self.instrs.push(Instr::new(Opcode::DropLabel, Some(2)));
        Ok(())
    }

    fn parse_all(&mut self, token: Token, d: usize) -> Result<(), GenericError> {
        match token.kind {
            TokenKind::If => self.if_block(d)?,
            TokenKind::Whlie => self.while_block(d)?,
            TokenKind::Func => self.func_block(d)?,
            TokenKind::Directive => self.directive()?,
            TokenKind::Var => self.var_stmt()?,
            TokenKind::Set => self.set_stmt()?,
            TokenKind::Assing => self.inline_var_stmt()?,
            TokenKind::ParenOpen => self.list()?,
            TokenKind::BracketOpen => self.index_get()?,
            TokenKind::Identifier => self.identfier(token)?,
            _ => self.parse_one(token)?,
        }

        Ok(())
    }

    fn parse_one(&mut self, token: Token) -> Result<(), GenericError> {
        let instr = match token.kind {
            TokenKind::Int => {
                self.consts.push(Value::Int64(token.value.parse().unwrap()));
                Ok(Instr::new(Opcode::Const, Some(self.consts.len() - 1)))
            }
            TokenKind::Str => {
                self.consts.push(Value::Str(token.value));
                Ok(Instr::new(Opcode::Const, Some(self.consts.len() - 1)))
            }

            // TODO: Eliminate this :(
            TokenKind::Add => Ok(Instr::new(Opcode::Add, None)),
            TokenKind::Minus => Ok(Instr::new(Opcode::Minus, None)),
            TokenKind::Mul => Ok(Instr::new(Opcode::Mul, None)),
            TokenKind::Div => Ok(Instr::new(Opcode::Div, None)),
            TokenKind::Mod => Ok(Instr::new(Opcode::Mod, None)),

            TokenKind::Pop => Ok(Instr::new(Opcode::Pop, None)),
            TokenKind::Dup => Ok(Instr::new(Opcode::Dup, None)),
            TokenKind::Dup2 => Ok(Instr::new(Opcode::Dup2, None)),
            TokenKind::Swap => Ok(Instr::new(Opcode::Swap, None)),
            TokenKind::Over => Ok(Instr::new(Opcode::Over, None)),

            TokenKind::Eq => Ok(Instr::new(Opcode::Eq, None)),
            TokenKind::Neq => Ok(Instr::new(Opcode::Neq, None)),
            TokenKind::Gt => Ok(Instr::new(Opcode::Gt, None)),
            TokenKind::Gte => Ok(Instr::new(Opcode::Gte, None)),
            TokenKind::Lte => Ok(Instr::new(Opcode::Lte, None)),
            TokenKind::Lt => Ok(Instr::new(Opcode::Lt, None)),

            TokenKind::Shl => Ok(Instr::new(Opcode::Shl, None)),
            TokenKind::Shr => Ok(Instr::new(Opcode::Shr, None)),
            TokenKind::Bitand => Ok(Instr::new(Opcode::Bitand, None)),
            TokenKind::Bitor => Ok(Instr::new(Opcode::Bitor, None)),

            TokenKind::Lor => Ok(Instr::new(Opcode::Lor, None)),
            TokenKind::Land => Ok(Instr::new(Opcode::Land, None)),

            TokenKind::Hlt => Ok(Instr::new(Opcode::Halt, None)),

            TokenKind::Len => Ok(Instr::new(Opcode::Buildin, Some(2))),
            TokenKind::Println => Ok(Instr::new(Opcode::Buildin, Some(3))),
            TokenKind::Print => Ok(Instr::new(Opcode::Buildin, Some(4))),
            TokenKind::Debug => Ok(Instr::new(Opcode::Buildin, Some(5))),
            // ################################################################## //
            _ => generic_error!("{:?} is not implemented yet", token.value),
        };
        self.instrs.push(instr?);
        Ok(())
    }

    fn directive(&mut self) -> Result<(), GenericError> {
        let tok = self.next();
        match tok.kind {
            TokenKind::Def => {
                let name = self.name_is_defined()?;
                let val = self.expect_or(TokenKind::Int, TokenKind::Str)?;
                self.consts_def.insert(name.value, val);
            }
            TokenKind::Include => {
                generic_error!("{:?} Not implemented", tok.kind)
            }
            e => generic_error!("{:?} is not directive", e),
        }
        Ok(())
    }

    fn var_stmt(&mut self) -> Result<(), GenericError> {
        let name = self.name_is_valid_variable()?;
        let v_ptr = match self.var_def.get(&name.value) {
            Some(v) => *v,
            None => {
                self.var_def.insert(name.value, self.var_count);
                self.var_count += 1;
                self.var_count - 1
            }
        };

        self.instrs.push(Instr::new(Opcode::PushPtr, Some(v_ptr)));
        loop {
            let tok = self.require()?;
            match tok.kind {
                TokenKind::SemiColon => break,
                TokenKind::ParenOpen => self.list()?,
                TokenKind::Identifier => self.identfier(tok)?,
                _ => self.parse_one(tok)?,
            }
        }
        self.instrs
            .push(Instr::new(Opcode::GlobalStore, Some(v_ptr)));
        Ok(())
    }

    fn inline_var_stmt(&mut self) -> Result<(), GenericError> {
        let name = self.name_is_valid_variable()?;
        let v_ptr = match self.var_def.get(&name.value) {
            Some(v) => *v,
            None => {
                self.var_def.insert(name.value, self.var_count);
                self.var_count += 1;
                self.var_count - 1
            }
        };

        self.instrs.push(Instr::new(Opcode::PushPtr, Some(v_ptr)));
        self.instrs
            .push(Instr::new(Opcode::InlineStore, Some(v_ptr)));
        Ok(())
    }

    fn set_stmt(&mut self) -> Result<(), GenericError> {
        let name = self.name_is_valid_variable()?;
        let mut is_idx = false;
        let v_ptr = match self.var_def.get(&name.value) {
            Some(v) => *v,
            None => {
                self.var_def.insert(name.value, self.var_count);
                self.var_count += 1;
                self.var_count - 1
            }
        };
        self.instrs.push(Instr::new(Opcode::PushPtr, Some(v_ptr)));
        if self.peek().kind == TokenKind::BracketOpen {
            self.next();
            loop {
                let idx_tok = self.require()?;
                match idx_tok.kind {
                    TokenKind::BracketClose => break,
                    TokenKind::Identifier => self.identfier(idx_tok)?,
                    _ => self.parse_one(idx_tok)?,
                }
            }
            is_idx = true;
        }
        loop {
            let tok = self.require()?;
            match tok.kind {
                TokenKind::SemiColon => {
                    break;
                }
                TokenKind::BracketOpen => self.index_get()?,
                TokenKind::Identifier => self.identfier(tok)?,
                _ => self.parse_one(tok)?,
            }
        }
        if is_idx {
            self.instrs.push(Instr::new(Opcode::Buildin, Some(1)));
            self.instrs
                .push(Instr::new(Opcode::GlobalStore, Some(v_ptr + 1)));
            return Ok(());
        }
        self.instrs
            .push(Instr::new(Opcode::GlobalStore, Some(v_ptr)));
        Ok(())
    }

    fn list(&mut self) -> Result<(), GenericError> {
        let mut list = vec![];
        loop {
            let tok = self.require()?;
            match tok.kind {
                TokenKind::ParenClose => break,
                TokenKind::Int => {
                    list.push(Rc::new(Value::Int64(tok.value.parse().unwrap())));
                }
                TokenKind::SemiColon => {}
                _ => generic_error!(
                    "{:?}({}) is not suported in List literals",
                    tok.kind,
                    tok.value
                ),
            }
        }
        self.consts
            .push(Value::List(RefCell::new(List { elem: list })));
        self.instrs
            .push(Instr::new(Opcode::Const, Some(self.consts.len() - 1)));
        return Ok(());
    }

    fn index_get(&mut self) -> Result<(), GenericError> {
        loop {
            let tok = self.require()?;
            match tok.kind {
                TokenKind::BracketClose => break,
                TokenKind::BracketOpen => self.index_get()?,
                TokenKind::Identifier => self.identfier(tok)?,
                _ => self.parse_one(tok)?,
            }
        }
        self.instrs.push(Instr::new(Opcode::Buildin, Some(0)));
        return Ok(());
    }

    fn identfier(&mut self, token: Token) -> Result<(), GenericError> {
        match self.consts_def.get(&token.value) {
            Some(v) => {
                if !matches!(v.kind, TokenKind::Int | TokenKind::Str) {
                    generic_error!("{:?} is not valid", token.value);
                }
                self.parse_one(v.clone())?;
                return Ok(());
            }
            None => {}
        }
        match self.var_def.get(&token.value) {
            Some(v) => {
                self.instrs.push(Instr::new(Opcode::PushPtr, Some(*v)));
                self.instrs.push(Instr::new(Opcode::GlobalLoad, Some(*v)));
                return Ok(());
            }
            None => {}
        }
        match self.func_def.get(&token.value) {
            Some(v) => {
                self.instrs.push(Instr::new(Opcode::Call, Some(*v)));
                return Ok(());
            }
            None => {}
        }

        generic_error!("{} is not defined", token)
    }

    fn func_block(&mut self, d: usize) -> Result<(), GenericError> {
        let funcinit = self.instrs.len();
        self.instrs.push(Instr::new(Opcode::PushLabel, Some(4)));
        let func_body_init = self.instrs.len();
        let name = self.name_is_defined()?;
        self.func_def.insert(name.value, func_body_init);
        self.expect(TokenKind::CurlyOpen)?;
        loop {
            let tok = self.require()?;
            match tok.kind {
                TokenKind::CurlyClose => {
                    self.instrs.push(Instr::new(Opcode::DropLabel, Some(4)));
                    self.instrs.push(Instr::new(Opcode::Ret, None));
                    self.instrs.insert(
                        funcinit,
                        Instr::new(Opcode::Jmp, Some(self.instrs.len() + 1)),
                    );
                    break;
                }
                TokenKind::Func | TokenKind::Directive => generic_error!("You cannot declareate {} here!", tok.value),
                _ => self.parse_all(tok, d + 1)?,
            }
        }
        Ok(())
    }
}
