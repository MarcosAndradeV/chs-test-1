use std::{collections::HashMap, path::PathBuf};

use crate::{
    instructions::{Instr, Opcode},
    value::CHSValue,
};

use super::{
    lexer::{Lexer, Token, TokenKind},
    node::Program,
};

const BODY_OFFSET: usize = 4;

macro_rules! error {
    ($message: expr, $($field: expr),*) => {
        return Err(ParseError {
            message: format!($message, $($field),*),
        })
    };

    ($message: expr) => {
        return Err(ParseError {
            message: $message.to_string(),
        })
    }
}

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
}

pub struct Parser {
    file: PathBuf,
    lexer: Lexer,
    peeked: Option<Token>,
    pos: usize,
    label_table: HashMap<String, usize>,
    instr_count: usize,
}

type ResTok = Result<Token, ParseError>;

impl Parser {
    pub fn new(input: Vec<u8>, file: PathBuf) -> Self {
        let lexer = Lexer::new(input);

        Self {
            file,
            lexer,
            peeked: None,
            pos: 0,
            label_table: HashMap::new(),
            instr_count: 0
        }
    }

    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let mut instrs = Vec::new();

        loop {
            let token = self.next();

            if token.kind == TokenKind::Null {
                let file = self.file.clone();
                println!("{:?}", self.label_table);
                let main_ = match self.label_table.get("main") {
                    Some(v) => *v,
                    None => error!("main Not Found.")
                };
                instrs.insert(0, Instr::new(Opcode::Halt, CHSValue::none()));
                instrs.insert(0, Instr::new(Opcode::Call, CHSValue::none()));
                instrs.insert(0, Instr::new(Opcode::Pushi, CHSValue::P(main_+2)));
                return Ok(Program { stmt: instrs, file });
            }
            self.instr_count = instrs.len();
            instrs.append(&mut self.top_level(token)?);
            println!("{}:{}", self.instr_count, instrs.len());
        }
    }

    fn expect(&mut self, kind: TokenKind) -> ResTok {
        let token = self.next();

        if token.kind == kind {
            return Ok(token);
        }

        error!("Expect {:?} at {}", kind, self.pos)
    }

    fn expect_or(&mut self, kind: TokenKind, kind_2: TokenKind) -> ResTok {
        let token = self.next();

        if token.kind == kind || token.kind == kind_2 {
            return Ok(token);
        }

        error!("Expect {:?} or {:?} at {}", kind, kind_2, self.pos)
    }

    fn not_expect(&mut self, kind: TokenKind) -> ResTok {
        let token = self.next();

        if token.kind != kind {
            return Ok(token);
        }

        error!("Not Expect {:?} at {}", kind, self.pos)
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
            return error!("require {:?}", tok);
        }
        Ok(tok)
    }

    fn top_level(&mut self, token: Token) -> Result<Vec<Instr>, ParseError> {
        match token.kind {
            TokenKind::Func => self.func(),
            _ => error!("Not top level {:?}", token),
        }
    }

    fn func(&mut self) -> Result<Vec<Instr>, ParseError> {
        let name = self.expect(TokenKind::Identifier)?;
        self.label_table.insert(name.value.clone(), self.instr_count+1);
        let _ = self.expect(TokenKind::CurlyOpen)?;
        let mut body = vec![Instr::new(Opcode::PreProc, CHSValue::None)];
        loop {
            let tok = self.require()?;
            match tok.kind {
                TokenKind::CurlyClose => break,
                TokenKind::If => {
                    self.if_block(&mut body)?;
                    continue;
                }
                TokenKind::Whlie => {
                    self.while_block(&mut body)?;
                    continue;
                }
                
                _ => body.push(self.instr(tok)?),
            }
        }
        if !body.last().is_some_and(|x| x.opcode == Opcode::Ret) {
            return error!("...");
        }
        Ok(body)
    }

    fn if_block(&mut self, body: &mut Vec<Instr>) -> Result<(), ParseError> {
        self.expect(TokenKind::CurlyOpen);
        let offset = body.len();
        let mut offset2 = 0;
        let mut has_else = false;
        loop {
            let tok = self.require()?;
            match tok.kind {
                TokenKind::CurlyClose => break,
                TokenKind::Else => {
                    has_else = true;
                    offset2 = body.len() + 1;
                    body.insert(
                        offset,
                        Instr::new(Opcode::JmpIf, CHSValue::P(body.len() + BODY_OFFSET + 1)),
                    );
                }
                _ => body.push(self.instr(tok)?),
            }
        }
        if !has_else {
            body.insert(
                offset,
                Instr::new(Opcode::JmpIf, CHSValue::P(body.len() + BODY_OFFSET)),
            );
        }
        if has_else {
            body.insert(
                offset2,
                Instr::new(Opcode::Jmp, CHSValue::P(body.len() + BODY_OFFSET)),
            );
        }
        Ok(())
    }

    fn while_block(&mut self, body: &mut Vec<Instr>) -> Result<(), ParseError> {
        body.push(Instr::new(Opcode::While, CHSValue::None));
        let mut ifoffset = 0usize;
        loop {
            // condition
            let tok = self.require()?;
            match tok.kind {
                TokenKind::CurlyOpen => {
                    ifoffset = body.len();
                    break;
                }
                _ => body.push(self.instr(tok)?),
            }
        }
        loop {
            let tok = self.require()?;
            match tok.kind {
                TokenKind::CurlyClose => {
                    body.push(Instr::new(Opcode::JmpWhile, CHSValue::None));
                    body.insert(
                        ifoffset,
                        Instr::new(Opcode::JmpIf, CHSValue::P(body.len() + BODY_OFFSET)),
                    );
                    body.push(Instr::new(Opcode::Unbind, CHSValue::P(1)));
                    break;
                }
                TokenKind::If => self.if_block(body)?,
                _ => body.push(self.instr(tok)?),
            }
        }
        Ok(())
    }

    fn instr(&mut self, tok: Token) -> Result<Instr, ParseError> {
        let instr = match tok.kind {
            TokenKind::Int => Instr::new(
                Opcode::Pushi,
                CHSValue::I(tok.value.parse::<i64>().unwrap()),
            ),
            TokenKind::Add => Instr::new(Opcode::Add, CHSValue::none()),
            TokenKind::Minus => Instr::new(Opcode::Minus, CHSValue::none()),
            TokenKind::Mul => Instr::new(Opcode::Mul, CHSValue::none()),
            TokenKind::Div => Instr::new(Opcode::Div, CHSValue::none()),
            TokenKind::Inc => Instr::new(Opcode::Inc, CHSValue::none()),
            TokenKind::Mod => Instr::new(Opcode::Mod, CHSValue::none()),
            TokenKind::Lgor => Instr::new(Opcode::Lgor, CHSValue::none()),
            TokenKind::Print => Instr::new(Opcode::Print, CHSValue::none()),
            TokenKind::Debug => Instr::new(Opcode::Debug, CHSValue::none()),
            TokenKind::Pop => Instr::new(Opcode::Pop, CHSValue::none()),
            TokenKind::Hlt => Instr::new(Opcode::Halt, CHSValue::none()),
            TokenKind::Eq => Instr::new(Opcode::Eq, CHSValue::none()),
            TokenKind::Dup => Instr::new(Opcode::Dup, CHSValue::none()),
            TokenKind::Gt => Instr::new(Opcode::Gt, CHSValue::none()),
            TokenKind::Lt => Instr::new(Opcode::Lt, CHSValue::none()),
            TokenKind::Call => Instr::new(Opcode::Call, CHSValue::none()),
            TokenKind::Ret => Instr::new(Opcode::Ret, CHSValue::none()),
            TokenKind::Swap => Instr::new(Opcode::Swap, CHSValue::none()),
            TokenKind::Over => Instr::new(Opcode::Over, self.operand()?),
            TokenKind::Jmp => Instr::new(Opcode::Jmp, self.operand()?),
            TokenKind::Identifier => {
                let pos = match self.label_table.get(&tok.value) {
                    Some(v) => *v,
                    None => error!("{} not found.", tok.value)
                };
                Instr::new(Opcode::Pushi, CHSValue::P(pos+2))
            }
            _ => return error!("{:?} is not a Instr at {}", tok, self.pos),
        };
        Ok(instr)
    }

    fn operand(&mut self) -> Result<CHSValue, ParseError> {
        let tok = self.expect(TokenKind::Int)?;
        Ok(CHSValue::P(tok.value.parse::<usize>().unwrap()))
    }
}
