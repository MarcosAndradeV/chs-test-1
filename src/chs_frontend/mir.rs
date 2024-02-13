use std::{rc::Rc, str::FromStr};

use crate::{exeptions::GenericError, generic_error};

use super::lexer::{Lexer, Token, TokenKind};

#[derive(Debug)]
pub enum Intrinsic {
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
pub enum Const {
    Str(Rc<[char]>),
    Int64(i64),
    Bool(bool),
    Fn(usize, usize),
    Nil,
}

#[derive(Debug)]
pub enum Opkind {
    Intrinsic(Intrinsic),
    Const(Const),
    MakeList,
    IfStart,
    IfEnd,
    WhileStart,
    WhileBlock,
    WhileEnd,
    MakeFn,
    Ret,
    Call,
    None,
}

#[derive(Debug)]
pub struct Operation {
    kind: Opkind,
    operand: Option<usize>,
    token: Token,
}

impl Operation {
    pub fn empty() -> Self {
        Operation {
            kind: Opkind::None,
            operand: None,
            token: Token::empty(),
        }
    }
}

impl FromStr for Intrinsic {
    type Err = GenericError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "+" => Self::Add,
            "-" => Self::Minus,
            "*" => Self::Mul,
            "/" => Self::Div,
            "=" => Self::Eq,
            "&" => Self::Bitand,
            "|" => Self::Bitor,
            "<" => Self::Lt,
            ">" => Self::Gt,
            "<=" => Self::Lte,
            ">=" => Self::Gte,
            "!=" => Self::Neq,
            ">>" => Self::Shr,
            "<<" => Self::Shl,
            "||" => Self::Lor,
            "&&" => Self::Land,
            "pop" => Self::Pop,
            "dup" => Self::Dup,
            "mod" => Self::Mod,
            "len" => Self::Len,
            "over" => Self::Over,
            "swap" => Self::Swap,
            "exit" => Self::Exit,
            "tail" => Self::Tail,
            "head" => Self::Head,
            "call" => Self::Call,
            "print" => Self::Print,
            "debug" => Self::Debug,
            "idxget" => Self::IdxGet,
            "idxset" => Self::IdxSet,
            "concat" => Self::Concat,
            _ => generic_error!("Cannot parse {} to Intrinsic", s),
        })
    }
}

pub type Operations = Vec<Operation>;

#[derive(Debug)]
pub enum CHSType {
    Int,
    Str,
    List(Rc<Self>),
    Fn(Rc<[Self]>, Rc<[Self]>),
    Nil,
    Void,
    Enum(String, Rc<[Self]>),
}

impl FromStr for CHSType {
    type Err = GenericError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "int" => Ok(Self::Int),
            "str" => Ok(Self::Str),
            "void" => Ok(Self::Void),
            _ => generic_error!("Cannot parse {} to CHSType", s),
        }
    }
}
#[derive(Debug)]
pub struct CHSFn {
    name: String,
    addr: usize,
    loc: (usize, usize),
    ins: Rc<[CHSType]>,
    outs: Rc<[CHSType]>,
    used: bool,
}

#[derive(Debug)]
pub struct Mir {
    ops: Vec<Operation>,
    fns: Vec<CHSFn>,
}

pub struct MirParser {
    lexer: Lexer,
    peeked: Option<Token>,
    ops: Operations,
    fn_def: Vec<CHSFn>,
}

impl MirParser {
    pub fn new(input: Vec<u8>) -> Self {
        let lexer = Lexer::new(input);
        Self {
            lexer,
            peeked: None,
            ops: Vec::new(),
            fn_def: Vec::new(),
        }
    }

    pub fn parse_to_mir(mut self) -> Result<Mir, GenericError> {
        loop {
            let token = self.next();

            if token.kind == TokenKind::Null {
                return Ok(Mir {
                    ops: self.ops,
                    fns: self.fn_def,
                });
            }
            self.expression(token)?;
        }
    }

    fn expression(&mut self, token: Token) -> Result<(), GenericError> {
        match token.kind {
            TokenKind::Pop
            | TokenKind::Dup
            | TokenKind::Over
            | TokenKind::Swap
            | TokenKind::Add
            | TokenKind::Minus
            | TokenKind::Mul
            | TokenKind::Div
            | TokenKind::Mod
            | TokenKind::Eq
            | TokenKind::Neq
            | TokenKind::Gt
            | TokenKind::Gte
            | TokenKind::Lte
            | TokenKind::Lt
            | TokenKind::Land
            | TokenKind::Lor
            | TokenKind::Shl
            | TokenKind::Shr
            | TokenKind::Bitand
            | TokenKind::Bitor
            | TokenKind::Exit
            | TokenKind::Print
            | TokenKind::IdxGet
            | TokenKind::IdxSet
            | TokenKind::Len
            | TokenKind::Concat
            | TokenKind::Tail
            | TokenKind::Head
            | TokenKind::Call => self.add_op_intrinsic(token)?,

            TokenKind::Int
            | TokenKind::Str
            | TokenKind::True
            | TokenKind::False
            | TokenKind::Nil => self.add_op_const(token)?,

            TokenKind::If => self.if_block(token)?,
            TokenKind::Fn => self.fn_block(token)?,
            TokenKind::Whlie => self.while_block(token)?,
            TokenKind::BracketOpen => self.list_expr(token)?,

            _ => generic_error!(
                "File:{}: Token {} is not implemeted",
                token.get_loc(),
                token.get_kind()
            ),
        }
        Ok(())
    }

    fn fn_block(&mut self, token: Token) -> Result<(), GenericError> {
        let offset = self.ops.len();
        self.ops.push(Operation::empty());

        let name = self.expect(TokenKind::Identifier)?.value;

        self.expect(TokenKind::DoubleColon)?;
        let mut ins = vec![];
        loop {
            let token = self.require()?;
            match token.kind {
                TokenKind::Arrow => break,
                TokenKind::Identifier => ins.push(CHSType::from_str(&token.value)?),
                TokenKind::BracketOpen => {
                    ins.push(CHSType::List(Rc::new(CHSType::from_str(
                        &self.expect(TokenKind::Identifier)?.value,
                    )?)));
                    self.expect(TokenKind::BracketClose)?;
                }
                TokenKind::CurlyOpen => {
                    self.peeked = Some(token);
                    break;
                }
                _ => generic_error!("File:{}: ???", token.get_loc()),
            }
        }
        let mut outs = vec![];
        loop {
            let token = self.require()?;
            match token.kind {
                TokenKind::CurlyOpen => break,
                TokenKind::Identifier => outs.push(CHSType::from_str(&token.value)?),
                TokenKind::BracketOpen => {
                    outs.push(CHSType::List(Rc::new(CHSType::from_str(
                        &self.expect(TokenKind::Identifier)?.value,
                    )?)));
                    self.expect(TokenKind::BracketClose)?;
                }
                _ => generic_error!("File:{}: ???", token.get_loc()),
            }
        }

        self.fn_def.push(CHSFn {
            name,
            addr: self.ops.len(),
            loc: token.loc,
            ins: ins.into(),
            outs: outs.into(),
            used: false,
        });

        loop {
            let token = self.require()?;
            match token.kind {
                TokenKind::CurlyClose => {
                    self.ops.push(Operation {
                        kind: Opkind::Ret,
                        operand: None,
                        token,
                    });
                    break;
                }
                _ => self.expression(token)?
            }
        }

        let skip_len = self.ops.len().saturating_sub(offset);
        let elem = unsafe { self.ops.get_unchecked_mut(offset) };
        *elem = Operation {
            kind: Opkind::MakeFn,
            operand: Some(skip_len),
            token,
        };
        Ok(())
    }

    fn list_expr(&mut self, token: Token) -> Result<(), GenericError> {
        let list_init = self.ops.len();
        loop {
            let token = self.require()?;
            match token.kind {
                TokenKind::BracketClose => break,
                TokenKind::BracketOpen => self.list_expr(token)?,
                _ if self.add_op_intrinsic(token.clone()).is_ok() => {}
                _ if self.add_op_const(token.clone()).is_ok() => {}
                _ => generic_error!(
                    "File:{}: Token {} not allowed in list literals",
                    token.get_loc(),
                    token.get_kind()
                ),
            }
        }
        let op = Operation {
            kind: Opkind::MakeList,
            operand: Some(self.ops.len().saturating_sub(list_init)),
            token,
        };
        self.ops.push(op);
        Ok(())
    }

    fn while_block(&mut self, token: Token) -> Result<(), GenericError> {
        let while_start = self.ops.len();
        self.ops.push(Operation::empty());

        loop {
            let token = self.require()?;
            match token.kind {
                TokenKind::CurlyOpen => {
                    self.peeked = Some(token);
                    break;
                }
                _ => self.expression(token)?,
            }
        }

        let while_block = self.ops.len();
        self.ops.push(Operation::empty());
        let wb_token = self.expect(TokenKind::CurlyOpen)?;

        loop {
            let token = self.require()?;
            match token.kind {
                TokenKind::CurlyClose => {
                    self.ops.push(Operation {
                        kind: Opkind::WhileEnd,
                        operand: Some(while_start),
                        token,
                    });
                    break;
                }
                _ => self.expression(token)?,
            }
        }

        let curr_len = self.ops.len();
        let elem = unsafe { self.ops.get_unchecked_mut(while_block) };
        *elem = Operation {
            kind: Opkind::WhileBlock,
            operand: Some(curr_len),
            token: wb_token,
        };

        let elem = unsafe { self.ops.get_unchecked_mut(while_start) };
        *elem = Operation {
            kind: Opkind::WhileStart,
            operand: Some(curr_len.saturating_sub(1)),
            token,
        };
        Ok(())
    }

    fn if_block(&mut self, token: Token) -> Result<(), GenericError> {
        let offset = self.ops.len();
        self.ops.push(Operation::empty());
        self.expect(TokenKind::CurlyOpen)?;

        loop {
            let token = self.require()?;
            match token.kind {
                TokenKind::CurlyClose => {
                    self.ops.push(Operation {
                        kind: Opkind::IfEnd,
                        operand: None,
                        token,
                    });
                    break;
                }
                TokenKind::Else => generic_error!("File:{}: Not implemeted", token.get_loc()),
                _ => self.expression(token)?,
            }
        }

        let curr_len = self.ops.len().saturating_sub(1);
        let elem = unsafe { self.ops.get_unchecked_mut(offset) };
        *elem = Operation {
            kind: Opkind::IfStart,
            operand: Some(curr_len),
            token,
        };
        Ok(())
    }

    fn add_op_const(&mut self, token: Token) -> Result<(), GenericError> {
        let c = match &token.kind {
            TokenKind::Int => {
                let v = token.value.parse::<i64>();
                if v.is_err() {
                    generic_error!(
                        "File:{}: Token {} cannot be parse {}",
                        token.get_loc(),
                        token.get_value(),
                        v.unwrap_err()
                    )
                }
                Const::Int64(v.unwrap())
            }
            TokenKind::True | TokenKind::False => {
                let v = token.value.parse::<bool>();
                if v.is_err() {
                    generic_error!(
                        "File:{}: Token {} cannot be parse {}",
                        token.get_loc(),
                        token.get_value(),
                        v.unwrap_err()
                    )
                }
                Const::Bool(v.unwrap())
            }
            TokenKind::Str => Const::Str(token.value.chars().collect()),
            TokenKind::Nil => Const::Nil,
            _ => generic_error!(""),
        };

        let op = Operation {
            kind: Opkind::Const(c),
            operand: None,
            token,
        };
        self.ops.push(op);
        Ok(())
    }

    fn add_op_intrinsic(&mut self, token: Token) -> Result<(), GenericError> {
        let op = Operation {
            kind: Opkind::Intrinsic(Intrinsic::from_str(&token.value)?),
            operand: None,
            token,
        };
        self.ops.push(op);
        Ok(())
    }

    fn next(&mut self) -> Token {
        loop {
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
    #[allow(unused)]
    fn peek(&mut self) -> &Token {
        if self.peeked.is_none() {
            self.peeked = Some(self.next());
        }

        self.peeked.as_ref().unwrap()
    }

    fn require(&mut self) -> Result<Token, GenericError> {
        let token = self.next();
        if matches!(token.kind, TokenKind::Invalid | TokenKind::Null) {
            generic_error!(
                "File:{}: Required token is {}",
                token.get_loc(),
                token.get_kind()
            );
        }
        Ok(token)
    }

    fn expect(&mut self, kind: TokenKind) -> Result<Token, GenericError> {
        let token = self.next();

        if token.kind == kind {
            return Ok(token);
        }

        generic_error!(
            "File:{}: Expected {:?} but get {} -> {}",
            token.get_loc(),
            kind,
            token.get_kind(),
            token.get_value()
        )
    }
}
