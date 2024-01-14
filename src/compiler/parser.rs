#![allow(unused)]
use std::collections::HashMap;

use crate::{exeptions::GenericError, generic_error, config::{Word, Value}, instructions::{Instr, Opcode}};

use super::lexer::{Lexer, Token, TokenKind};

type ResTok = Result<Token, GenericError>;

pub struct Parser {
    lexer: Lexer,
    consts: Vec<Value>, instrs: Vec<Instr>,
    pos: usize,
    peeked: Option<Token>,
    consts_def: HashMap<String, Token>,
    macro_def: HashMap<String, Vec<Token>>,
    var_def: HashMap<String, usize>,
    var_count: usize,
}

impl Parser {
    pub fn new(input: Vec<u8>) -> Self {
        let lexer = Lexer::new(input);

        Self {
            lexer,
            consts: Vec::new(), instrs:Vec::new(),
            pos: 0,
            peeked: None,
            consts_def: HashMap::new(),
            macro_def: HashMap::new(),
            var_def: HashMap::new(),
            var_count: 0
        }
    }
    pub fn parse(&mut self) -> Result<(Vec<Instr>, Vec<Value>), GenericError> {
        loop {
            let token = self.next();

            if token.kind == TokenKind::Null {
                return Ok((self.instrs.clone(), self.consts.clone()));
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
        if self.consts_def.get(&token.value).is_some() || self.macro_def.get(&token.value).is_some() {
            return generic_error!("{} is already defined", token.value);
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
                    self.instrs.push(Instr::new(Opcode::Jmp, Some(whileaddrs)));
                    self.instrs.insert(
                        ifoffset,
                        Instr::new(Opcode::JmpIf, Some(self.instrs.len() + 1+d)),
                    );
                    self.instrs.push(Instr::new(Opcode::DropLabel, None));
                    break;
                }
                _ => self.parse_all(tok, d+1)?
            }
        }
        Ok(())
    }


    fn if_block(&mut self, d: usize) -> Result<(), GenericError> {
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
                _ => self.parse_all(tok, d+1)?
            }
        }
        if !has_else {
            self.instrs.insert(
                offset,
                Instr::new(Opcode::JmpIf, Some(self.instrs.len() + 1+d)),
            );
        }
        if has_else {
            self.instrs.insert(
                offset2,
                Instr::new(Opcode::Jmp, Some(self.instrs.len()+1+d)),
            );
        }
        Ok(())
    }

    fn parse_all(&mut self, token: Token, d: usize) -> Result<(), GenericError> {
       match token.kind {
            TokenKind::If => self.if_block(d)?,
            TokenKind::Whlie => self.while_block(d)?,
            TokenKind::Directive => self.directive()?,
            TokenKind::Var => self.var_stmt()?,
            TokenKind::Set => self.set_stmt()?,
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
            TokenKind::Print => Ok(Instr::new(Opcode::Print, None)),
            TokenKind::Debug => Ok(Instr::new(Opcode::Debug, None)),
            TokenKind::Load => Ok(Instr::new(Opcode::Load, None)),
            TokenKind::Store => Ok(Instr::new(Opcode::Store, None)),
            TokenKind::Write => Ok(Instr::new(Opcode::Write, None)),
            TokenKind::Pstr => Ok(Instr::new(Opcode::Pstr, None)),
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
                match self.var_def.get(&token.value) {
                    Some(v) => {
                        self.instrs.push(Instr::new(Opcode::PushPtr, Some(*v)));
                        self.instrs.push(Instr::new(Opcode::Load, None));
                        return Ok(());
                    }
                    None => return generic_error!("{} is not defined", token.value)
                }

                return Ok(());
            }
            _ => generic_error!("{:?} is not implemented yet", token.kind)
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
            e => return generic_error!("{:?} is not directive", e)
        }
        Ok(())
    }

    fn var_stmt(&mut self) -> Result<(), GenericError> {
        let name = self.name_def()?;
        self.var_def.insert(name.value, self.var_count);
        self.var_count += 1;
        match self.require()?.kind {
            TokenKind::I64 => {
                self.instrs.push(Instr::new(Opcode::PushPtr, Some(self.var_count-1)));
                loop {
                    let tok = self.require()?;
                    match tok.kind {
                        TokenKind::SemiColon => {
                            break;
                        }
                        _ => self.parse_one(tok)?
                    }
                }
                self.instrs.push(Instr::new(Opcode::Store, None));
                return Ok(());
            },
            TokenKind::U64 => {
                self.instrs.push(Instr::new(Opcode::PushPtr, Some(self.var_count-1)));
                loop {
                    let tok = self.require()?;
                    match tok.kind {
                        TokenKind::SemiColon => {
                            break;
                        }
                        TokenKind::Int => {
                            self.consts.push(Value::Uint64(tok.value.parse().unwrap()));
                            self.instrs.push(Instr::new(Opcode::Const, Some(self.consts.len()-1)))
                        }
                        _ => self.parse_one(tok)?
                    }
                }
                self.instrs.push(Instr::new(Opcode::Store, None));
                return Ok(());
            }
            TokenKind::StrT => {
                self.instrs.push(Instr::new(Opcode::PushPtr, Some(self.var_count-1)));
                loop {
                    let tok = self.require()?;
                    match tok.kind {
                        TokenKind::SemiColon => {
                            break;
                        }
                        TokenKind::Str => {
                            self.consts.push(Value::Str(tok.value));
                            self.instrs.push(Instr::new(Opcode::Const, Some(self.consts.len()-1)));
                        }
                        _ => return generic_error!("???")
                    }
                }
                self.instrs.push(Instr::new(Opcode::Store, None));
                return Ok(());
            }
            e=> return generic_error!("{:?} Type not suported", e)
        }
    }

    fn set_stmt(&mut self) -> Result<(), GenericError> {
        let name = self.require()?;
        match self.var_def.get(&name.value) {
            Some(v) => self.instrs.push(Instr::new(Opcode::PushPtr, Some(*v))),
            None => return generic_error!("{} is not defined yet", name.value),
        }
        loop {
            let tok = self.require()?;
            match tok.kind {
                TokenKind::SemiColon => {
                    break;
                }
                _ => self.parse_one(tok)?
            }
        }
        self.instrs.push(Instr::new(Opcode::Store, None));


        Ok(())
    }

}