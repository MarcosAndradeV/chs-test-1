#![allow(unused)]
use std::{collections::HashMap, rc::Rc, cell::RefCell};

use crate::{exeptions::GenericError, generic_error, value::{Value, List}, instructions::{Instr, Opcode, Bytecode}};

use super::lexer::{Lexer, Token, TokenKind};

type ResTok = Result<Token, GenericError>;

pub struct Parser {
    lexer: Lexer,
    consts: Vec<Value>, instrs: Vec<Instr>,
    pos: usize,
    peeked: Option<Token>,
    entry_point: usize,
    consts_def: HashMap<String, Token>,
    macro_def: HashMap<String, Vec<Token>>,
    var_def: HashMap<String, usize>,
    var_count: usize,
    proc_def: HashMap<String, usize>,
    locals: HashMap<String, usize>,
    locals_count: usize,
}

impl Parser {
    pub fn new(input: Vec<u8>) -> Self {
        let lexer = Lexer::new(input);

        Self {
            lexer,
            consts: Vec::new(), instrs:Vec::new(),
            pos: 0,
            peeked: None,
            entry_point: 0,
            consts_def: HashMap::new(),
            macro_def: HashMap::new(),
            var_def: HashMap::new(),
            var_count: 0,
            proc_def: HashMap::new(),
            locals: HashMap::new(),
            locals_count: 1,
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

    fn name_def(&mut self) -> ResTok {
        let token = self.expect(TokenKind::Identifier)?;
        if self.consts_def.get(&token.value).is_some() {
            return generic_error!("{} is already defined as const", token.value);
        }
        if self.macro_def.get(&token.value).is_some() {
            return generic_error!("{} is already defined as macro", token.value);
        }
        if self.var_def.get(&token.value).is_some() {
            return generic_error!("{} is already defined as variable", token.value);
        }
        if self.proc_def.get(&token.value).is_some() {
            return generic_error!("{} is already defined as procedure", token.value);
        }
        Ok(token)
    }

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
            generic_error!("require {:?}", tok);
        }
        Ok(tok)
    }

    fn while_block(&mut self, d: usize) -> Result<(), GenericError> {
        self.instrs.push(Instr::new(Opcode::PushLabel, None));
        let whileaddrs = self.instrs.len();
        let mut ifoffset = 0usize;
        loop {
            // condition
            let tok = self.require()?;
            match tok.kind {
                TokenKind::CurlyOpen => {
                    ifoffset = self.instrs.len();
                    break;
                }
                _ => self.parse_one(tok)?,
            }
        }
        loop {
            let tok = self.require()?;
            match tok.kind {
                TokenKind::CurlyClose => {
                    self.instrs.push(Instr::new(Opcode::Jmp, Some(whileaddrs+d)));
                    self.instrs.insert(
                        ifoffset,
                        Instr::new(Opcode::JmpIf, Some(self.instrs.len() + 1+d)),
                    );
                    self.instrs.push(Instr::new(Opcode::DropLabel, None));
                    break;
                }
                TokenKind::Func | TokenKind::Directive => 
                {return generic_error!("You cannot declareate {} here!", tok.value);}
                _ => self.parse_all(tok, d+1)?
            }
        }
        Ok(())
    }
    
    
    fn if_block(&mut self, d: usize) -> Result<(), GenericError> {
        self.instrs.push(Instr::new(Opcode::PushLabel, None));
        self.expect(TokenKind::CurlyOpen);
        let offset = self.instrs.len();
        let mut offset2 = 0;
        let mut has_else = false;
        loop {
            let tok = self.require()?;
            match tok.kind {
                TokenKind::CurlyClose => break,
                TokenKind::Else => {
                    has_else = true;
                    offset2 = self.instrs.len() + 1;
                    self.instrs.insert(
                        offset,
                        Instr::new(Opcode::JmpIf, Some(self.instrs.len() + 2+d)),
                    );
                }
                TokenKind::Func | TokenKind::Directive => 
                {return generic_error!("You cannot declareate {} here!", tok.value);}
                _ => self.parse_all(tok, d+1)?
            }
        }
        if !has_else {
            self.instrs.insert(
                offset,
                Instr::new(Opcode::JmpIf, Some(self.instrs.len() + 1 + d)),
            );
        }
        if has_else {
            self.instrs.insert(
                offset2,
                Instr::new(Opcode::Jmp, Some(self.instrs.len()+1+d)),
            );
        }
        self.instrs.push(Instr::new(Opcode::DropLabel, None));
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
            TokenKind::ParenOpen => self.list()?,
            TokenKind::BracketOpen => self.index_get()?,
            _ => self.parse_one(token)?
        }
        
        Ok(())
    }

    fn parse_one(&mut self, token: Token) -> Result<(), GenericError> {
        let instr = match token.kind {
            TokenKind::Int => {
                self.consts.push(Value::Int64(token.value.parse().unwrap()));
                Ok(Instr::new(Opcode::Const, Some(self.consts.len()-1)))
            }
            TokenKind::Str => {
                self.consts.push(Value::Str(token.value));
                Ok(Instr::new(Opcode::Const, Some(self.consts.len()-1)))
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
            TokenKind::Println => Ok(Instr::new(Opcode::Println, None)),
            TokenKind::Len => Ok(Instr::new(Opcode::Len, None)),
            TokenKind::Debug => Ok(Instr::new(Opcode::Debug, None)),
            TokenKind::Print => Ok(Instr::new(Opcode::Print, None)),
            TokenKind::Hlt => Ok(Instr::new(Opcode::Halt, None)),
            // ################################################################## //

            TokenKind::Identifier => {
                match self.consts_def.get(&token.value) {
                    Some(v) => {
                        if !matches!(v.kind, TokenKind::Int | TokenKind::Str) {
                            return generic_error!("{:?} is not valid", token.value);
                        }
                        self.parse_one(v.clone())?;
                        return Ok(());
                    },
                    None => {}
                }
                match self.macro_def.get(&token.value) {
                    Some(v) => {
                        for t in v.clone() {
                            self.parse_one(t)?;
                        }
                        return Ok(());
                    }
                    None => {}
                }
                match self.locals.get(&token.value) {
                    Some(v) => {
                        self.instrs.push(Instr::new(Opcode::PushPtr, Some(*v)));
                        self.instrs.push(Instr::new(Opcode::Load, None));
                        return Ok(());
                    }
                    None => {
                        match self.var_def.get(&token.value) {
                            Some(v) => {
                                self.instrs.push(Instr::new(Opcode::PushPtr, Some(*v)));
                                self.instrs.push(Instr::new(Opcode::Load, None));
                                return Ok(());
                            }
                            None => {}
                        }
                    }
                }

                match self.proc_def.get(&token.value) {
                    Some(v) => {
                        self.instrs.push(Instr::new(Opcode::PushLabel, None));
                        self.instrs.push(Instr::new(Opcode::Jmp, Some(*v)));
                        return Ok(());
                    }
                    None => return generic_error!("{} is not defined", token.value)
                }

                return Ok(());
            }
            _ => generic_error!("{:?} is not implemented yet", token.value)
        };
        self.instrs.push(instr?);
        Ok(())
    }

    fn directive(&mut self) -> Result<(), GenericError> {
        let tok = self.next();
        match tok.kind {
            TokenKind::Def => {
                let name = self.name_def()?;

                let val = self.expect_or(TokenKind::Int, TokenKind::Str)?;
                self.consts_def.insert(name.value, val);
            }
            TokenKind::Macro => {
                let name = self.name_def()?;
                let mut toks = vec![];
                self.expect(TokenKind::CurlyOpen)?;
                loop {
                    let tok = self.require()?;
                    match tok.kind {
                        TokenKind::CurlyClose => {
                            break;
                        }
                        _ => toks.push(tok),
                    }
                }
                self.macro_def.insert(name.value, toks);
            }
            TokenKind::Entry => {
                self.entry_point = self.instrs.len();
            }
            e => return generic_error!("{:?} is not directive", e)
        }
        Ok(())
    }

    fn var_stmt(&mut self) -> Result<(), GenericError> {
        let name = self.require()?;
        let var_count = match self.var_def.get(&name.value) {
            Some(v) => {*v}
            None => {
                self.var_def.insert(name.value, self.var_count);
                self.var_count += 1;
                self.var_count - 1
            }
        } ;

        self.instrs.push(Instr::new(Opcode::PushPtr, Some(var_count)));
        loop {
            let tok = self.require()?;
            match tok.kind {
                TokenKind::SemiColon => break,
                TokenKind::ParenOpen => self.list()?,
                _ => self.parse_one(tok)?
            }
        }
        self.instrs.push(Instr::new(Opcode::Store, None));
        Ok(())
    }

    fn local_var_stmt(&mut self) -> Result<(), GenericError> {
        let name = self.require()?;
        let local_var_stmt = match self.locals.get(&name.value) {
            Some(v) => {*v}
            None => {
                self.locals.insert(name.value, self.locals_count);
                self.locals_count += 1;
                self.locals_count - 1
            }
        } ;

        self.instrs.push(Instr::new(Opcode::PushPtr, Some(local_var_stmt)));
        loop {
            let tok = self.require()?;
            match tok.kind {
                TokenKind::SemiColon => break,
                TokenKind::ParenOpen => self.list()?,
                _ => self.parse_one(tok)?
            }
        }
        self.instrs.push(Instr::new(Opcode::Store, None));
        Ok(())
    }

    fn set_stmt(&mut self) -> Result<(), GenericError> {
        let name = self.require()?;
        let mut is_idx = false;
        match self.locals.get(&name.value) {
            Some(v) => {self.instrs.push(Instr::new(Opcode::PushPtr, Some(*v)))}
            None => {
                match self.var_def.get(&name.value) {
                    Some(v) => self.instrs.push(Instr::new(Opcode::PushPtr, Some(*v))),
                    None => return generic_error!("{} is not defined yet", name.value),
                }
            }
        }
        if self.peek().kind == TokenKind::BracketOpen {
            self.next();
            loop {
                let idx_tok = self.require()?;
                match idx_tok.kind {
                    TokenKind::BracketClose => break,
                    _ => self.parse_one(idx_tok)?
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
                _ => self.parse_one(tok)?
            }
        }
        if is_idx {
            self.instrs.push(Instr::new(Opcode::IdxSet, None));
            self.instrs.push(Instr::new(Opcode::Store, None));
            return Ok(());
        }
        self.instrs.push(Instr::new(Opcode::Store, None));
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
                _ => return generic_error!("{:?}({}) is not suported in List literals", tok.kind, tok.value),
            }
        }
        self.consts.push(Value::List(RefCell::new(List {elem: list})));
        self.instrs.push(Instr::new(Opcode::Const, Some(self.consts.len()-1)));
        return Ok(());

    }

    fn index_get(&mut self) -> Result<(), GenericError> {
        loop {
            let tok = self.require()?;
            match tok.kind {
                TokenKind::BracketClose => break,
                TokenKind::BracketOpen => self.index_get()?,
                _ => self.parse_one(tok)?
            }
        }
        self.instrs.push(Instr::new(Opcode::IdxGet, None));
        return Ok(());
    }

    fn func_block(&mut self, d: usize) -> Result<(), GenericError> {
        let name = self.name_def()?;
        let mut ret = false;
        let pos = self.instrs.len();
        self.expect(TokenKind::ParenOpen)?;
        loop {
            let tok = self.require()?;
            match tok.kind {
                TokenKind::ParenClose => break,
                TokenKind::Identifier => {
                    self.locals.insert(tok.value, self.locals_count);
                    self.locals_count += 1;
                    self.instrs.push(Instr::new(Opcode::PushPtr, Some(self.locals_count - 1)));
                    self.instrs.push(Instr::new(Opcode::Over, None));
                    self.instrs.push(Instr::new(Opcode::Store, None));
                    self.instrs.push(Instr::new(Opcode::Pop, None));
                }
                _ => generic_error!("{} is not accepted", tok)
            }
        }


        self.proc_def.insert(name.value, pos+1);
        self.expect(TokenKind::CurlyOpen)?;
        loop {
            let tok = self.require()?;
            match tok.kind {
                TokenKind::CurlyClose => {
                    if !ret {
                        self.instrs.push(Instr::new(Opcode::GetLabel, None));
                        self.instrs.push(Instr::new(Opcode::Jmpr, None));
                    }
                    self.instrs.insert(pos, Instr::new(
                        Opcode::Jmp, Some(self.instrs.len()+1+d)));
                    self.locals.clear();
                    self.locals_count = self.var_count+1;
                    break
                },
                TokenKind::Var => { self.local_var_stmt()? }
                TokenKind::Identifier => {
                    match self.locals.get(&tok.value) {
                        Some(v) => {
                            self.instrs.push(Instr::new(Opcode::PushPtr, Some(*v)));
                            self.instrs.push(Instr::new(Opcode::Load, None));
                            continue;
                        }
                        None => self.parse_one(tok)?
                    }
                }
                TokenKind::Ret => {
                    self.instrs.push(Instr::new(Opcode::GetLabel, None));
                    self.instrs.push(Instr::new(Opcode::Jmpr, None));
                    ret = true;
                },
                _ => self.parse_all(tok, d+1)?
            }
        }
        Ok(())
    }

}