use core::fmt;

use chs_util::Loc;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenKind {
    Unknow,
    EOF,
    Invalid,
    Whitespace,
    Comment,
    Ident,
    KeyWord,
    Interger,
    //Float,
    Char,
    String,
    Plus,
    Minus,
    Star,
    Slash,
    Assigin,
    ParenOpen,
    ParenClose,
    CurlyOpen,
    CurlyClose,
    BracketOpen,
    BracketClose,
    Arrow,
    Colon,
    DoubleColon,
    SemiColon,
    Comma,
    Dot,
    Ampersand,
    Percent,
    Eq,
    NEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    LAnd,
    LOr,
    LNot,
    Bor,
    ShL,
    ShR,
    True,
    False,
    Buildin,
    Pop,
    Dup,
    Over,
    Swap,
    Rot,
    Nop,
    Debug,
    Exit,
    Print,
    IdxSet,
    IdxGet,
    Len,
    Concat,
    Head,
    Tail,
    Call,
    DollarSing,
    Nil,
    Error,
    Tilde,
}

impl TokenKind {
    pub fn is_simple(&self) -> bool {
        match self {
            TokenKind::EOF
            | TokenKind::Whitespace
            | TokenKind::Comment
            | TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Star
            | TokenKind::Slash
            | TokenKind::ParenOpen
            | TokenKind::ParenClose
            | TokenKind::CurlyOpen
            | TokenKind::CurlyClose
            | TokenKind::BracketOpen
            | TokenKind::Arrow
            | TokenKind::Colon
            | TokenKind::SemiColon
            | TokenKind::Comma
            | TokenKind::Dot
            | TokenKind::Assigin
            | TokenKind::Ampersand
            | TokenKind::Percent
            | TokenKind::Eq
            | TokenKind::NEq
            | TokenKind::Lt
            | TokenKind::LtEq
            | TokenKind::Gt
            | TokenKind::GtEq
            | TokenKind::LAnd
            | TokenKind::LOr
            | TokenKind::LNot
            | TokenKind::Bor
            | TokenKind::ShL
            | TokenKind::ShR
            | TokenKind::True
            | TokenKind::False
            | TokenKind::Pop
            | TokenKind::Dup
            | TokenKind::Over
            | TokenKind::Swap
            | TokenKind::Rot
            | TokenKind::Nop
            | TokenKind::Debug
            | TokenKind::Exit
            | TokenKind::Print
            | TokenKind::IdxSet
            | TokenKind::IdxGet
            | TokenKind::Len
            | TokenKind::Concat
            | TokenKind::Head
            | TokenKind::Tail
            | TokenKind::Call
            | TokenKind::DollarSing
            | TokenKind::Nil
            | TokenKind::Error
            | TokenKind::Tilde
            | TokenKind::DoubleColon
            | TokenKind::BracketClose => true,
            _ => false,
        }
    }
}

impl Default for TokenKind {
    fn default() -> Self {
        Self::Unknow
    }
}

#[derive(Debug, Default, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
    pub loc: Loc,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.kind.is_simple() {
            write!(f, "{:?} at {}", self.kind, self.loc)
        } else {
            write!(f, "{:?}({}) at {}", self.kind, self.value, self.loc)
        }
    }
}

impl Token {
    pub fn new(value: String, kind: TokenKind, loc: Loc) -> Self {
        Self { kind, value, loc }
    }
    pub fn is_eof(&self) -> bool {
        self.kind == TokenKind::EOF
    }
    pub fn is_whitespace(&self) -> bool {
        self.kind == TokenKind::Whitespace
    }
    pub fn is_commet(&self) -> bool {
        self.kind == TokenKind::Comment
    }
    pub fn is_unknow(&self) -> bool {
        self.kind == TokenKind::Unknow
    }
    pub fn is_binary_operator(&self) -> bool {
        matches!(
            self.kind,
            TokenKind::Plus
                | TokenKind::Star
                | TokenKind::Minus
                | TokenKind::Slash
                | TokenKind::Percent
                | TokenKind::Eq
                | TokenKind::NEq
                | TokenKind::Lt
                | TokenKind::LtEq
                | TokenKind::Gt
                | TokenKind::GtEq
                | TokenKind::LAnd
                | TokenKind::LOr //| TokenKind::LNot
                | TokenKind::Bor
                | TokenKind::ShL
                | TokenKind::ShR
                | TokenKind::Ampersand
        )
    }
    pub fn val_eq(&self, value: &str) -> bool {
        self.value == value
    }
}

#[derive(Debug)]
pub struct Lexer {
    input: Vec<u8>,
    max_pos: usize,
    curr_pos: usize,
    curr_loc: Loc,
}

impl Lexer {
    pub fn new(input: Vec<u8>) -> Self {
        let max_pos = input.len();
        Self {
            input,
            curr_pos: 0,
            max_pos,
            curr_loc: Loc::new(1, 1),
        }
    }

    pub fn reset(&mut self) {
        self.curr_pos = 0;
        self.curr_loc = Loc::new(1, 1);
    }

    pub fn get_next_token(&mut self) -> Token {
        let start: usize = self.curr_pos;
        let start_loc = self.curr_loc;
        match self.curr_char() {
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.identfier(start),
            b'0'..=b'9' => self.number(start),
            c if c.is_ascii_whitespace() => self.whitespace(start),
            b'\"' => self.string(start),
            b'#' => self.char_(start),
            b'/' => {
                if self.peek_char(1) == b'/' {
                    self.curr_pos += 2;
                    return self.comment(start);
                }
                self.make_token_advance(start, TokenKind::Slash)
            }
            b'-' => {
                if self.peek_char(1) == b'>' {
                    self.curr_pos += 2;
                    return self.make_token_advance(start, TokenKind::Arrow);
                }
                self.make_token_advance(start, TokenKind::Minus)
            }
            b'+' => {
                if self.peek_char(1) == b'+' {
                    self.curr_pos += 2;
                    return self.make_token_advance(start, TokenKind::Concat);
                }
                self.make_token_advance(start, TokenKind::Plus)
            }
            b'*' => self.make_token_advance(start, TokenKind::Star),
            b'(' => self.make_token_advance(start, TokenKind::ParenOpen),
            b')' => self.make_token_advance(start, TokenKind::ParenClose),
            b'{' => self.make_token_advance(start, TokenKind::CurlyOpen),
            b'}' => self.make_token_advance(start, TokenKind::CurlyClose),
            b'[' => self.make_token_advance(start, TokenKind::BracketOpen),
            b']' => self.make_token_advance(start, TokenKind::BracketClose),
            b':' => {
                if self.peek_char(1) == b'=' {
                    self.curr_pos += 2;
                    return self.make_token_advance(start, TokenKind::Assigin);
                }
                if self.peek_char(1) == b':' {
                    self.curr_pos += 2;
                    return self.make_token_advance(start, TokenKind::DoubleColon);
                }
                self.make_token_advance(start, TokenKind::Swap)
            }
            b';' => self.make_token_advance(start, TokenKind::SemiColon),
            b',' => self.make_token_advance(start, TokenKind::Comma),
            b'.' => self.make_token_advance(start, TokenKind::Dup),
            b'%' => self.make_token_advance(start, TokenKind::Percent),
            b'|' => self.make_token_advance(start, TokenKind::Bor),
            b'$' => self.make_token_advance(start, TokenKind::DollarSing),
            b'~' => self.make_token_advance(start, TokenKind::Tilde),
            b'=' => self.make_token_advance(start, TokenKind::Eq),
            b'!' => {
                if self.peek_char(1) == b'=' {
                    self.curr_pos += 2;
                    return self.make_token(start, TokenKind::NEq, start_loc);
                }
                self.make_token_advance(start, TokenKind::LNot)
            }
            b'>' => {
                if self.peek_char(1) == b'=' {
                    self.curr_pos += 2;
                    return self.make_token(start, TokenKind::GtEq, start_loc);
                }
                if self.peek_char(1) == b'>' {
                    self.curr_pos += 2;
                    return self.make_token(start, TokenKind::ShR, start_loc);
                }
                self.make_token_advance(start, TokenKind::Gt)
            }
            b'<' => {
                if self.peek_char(1) == b'=' {
                    self.curr_pos += 2;
                    return self.make_token(start, TokenKind::LtEq, start_loc);
                }
                if self.peek_char(1) == b'<' {
                    self.curr_pos += 2;
                    return self.make_token(start, TokenKind::ShL, start_loc);
                }
                self.make_token_advance(start, TokenKind::Lt)
            }
            b'&' => self.make_token_advance(start, TokenKind::Ampersand),
            _ => {
                if self.has_next() {
                    self.make_token_advance(start, TokenKind::Invalid)
                } else {
                    Token::new(String::from("\0"), TokenKind::EOF, start_loc)
                }
            }
        }
    }

    fn make_token_advance(&mut self, start: usize, kind: TokenKind) -> Token {
        let start_loc = self.curr_loc;
        self.advance_pos();
        self.make_token(start, kind, start_loc)
    }

    fn whitespace(&mut self, start: usize) -> Token {
        let start_loc = self.curr_loc;
        loop {
            self.advance_pos();
            if !self.curr_char().is_ascii_whitespace() {
                break;
            }
        }
        self.make_token(start, TokenKind::Whitespace, start_loc)
    }

    fn comment(&mut self, start: usize) -> Token {
        let start_loc = self.curr_loc;
        loop {
            self.advance_pos();
            if matches!(self.curr_char(), b'\n' | b'\0') {
                break;
            }
        }
        self.make_token(start, TokenKind::Comment, start_loc)
    }

    fn string(&mut self, start: usize) -> Token {
        let start_loc = self.curr_loc;
        let mut buf = String::new();
        loop {
            self.advance_pos();
            match self.curr_char() {
                b'\"' => break self.advance_pos(),
                b'\0' => return self.make_token(start, TokenKind::Invalid, start_loc),
                b'\\' => {
                    match self.peek_char(1) {
                        b'n' => buf.push('\n'),
                        b'\\' => buf.push('\\'),
                        _ => return self.make_token_advance(start, TokenKind::Invalid),
                    }
                    self.advance_pos();
                }
                a => buf.push(a as char),
            }
        }
        Token::new(buf, TokenKind::String, start_loc)
    }

    fn char_(&mut self, start: usize) -> Token {
        let start_loc = self.curr_loc;
        let mut buf = String::new();
        self.advance_pos();
        match self.curr_char() {
            b'\\' => {
                match self.peek_char(1) {
                    b'n' => buf.push('\n'),
                    b'\\' => buf.push('\\'),
                    b' ' => buf.push(' '),
                    _ => return self.make_token_advance(start, TokenKind::Invalid),
                }
                self.advance_pos();
            }
            a => buf.push(a as char)
        }
        self.advance_pos();
        Token::new(buf, TokenKind::Char, start_loc)
    }

    fn number(&mut self, start: usize) -> Token {
        let start_loc = self.curr_loc;
        loop {
            self.advance_pos();
            if !matches!(self.curr_char(), b'0'..=b'9' | b'.') {
                break;
            }
        }
        self.make_token(start, TokenKind::Interger, start_loc)
    }

    fn advance_pos(&mut self) {
        if self.has_next() {
            self.curr_pos += 1;
            self.curr_loc = self.curr_loc.next(self.curr_char());
        }
    }

    fn identfier(&mut self, start: usize) -> Token {
        let start_loc = self.curr_loc;
        loop {
            self.advance_pos();
            if !matches!(self.curr_char(), b'a'..=b'z' | b'A'..=b'Z' | b'_' | b'0'..=b'9') {
                break;
            }
        }
        let value = String::from_utf8_lossy(&self.input[start..self.curr_pos]).to_string();
        match value.as_str() {
            "fn" | "while" | "if" | "else" | "peek" | "let" => {
                Token::new(value, TokenKind::KeyWord, start_loc)
            }
            "print" => Token::new(value, TokenKind::Print, start_loc),
            "and" => Token::new(value, TokenKind::LAnd, start_loc),
            "or" => Token::new(value, TokenKind::LOr, start_loc),
            "true" => Token::new(value, TokenKind::True, start_loc),
            "false" => Token::new(value, TokenKind::False, start_loc),
            "nil" => Token::new(value, TokenKind::Nil, start_loc),
            "head" => Token::new(value, TokenKind::Head, start_loc),
            "tail" => Token::new(value, TokenKind::Tail, start_loc),
            "over" => Token::new(value, TokenKind::Over, start_loc),
            "rot" => Token::new(value, TokenKind::Rot, start_loc),
            "call" => Token::new(value, TokenKind::Call, start_loc),
            "pop" | "drop" => Token::new(value, TokenKind::Pop, start_loc),
            "error" => Token::new(value, TokenKind::Error, start_loc),
            "debug" => Token::new(value, TokenKind::Debug, start_loc),
            "mod" => Token::new(value, TokenKind::Percent, start_loc),
            "idxset" => Token::new(value, TokenKind::IdxSet, start_loc),
            "idxget" => Token::new(value, TokenKind::IdxGet, start_loc),
            "len" => Token::new(value, TokenKind::Len, start_loc),
            "dup" => Token::new(value, TokenKind::Dup, start_loc),
            _ => Token::new(value, TokenKind::Ident, start_loc),
        }
    }

    fn make_token(&self, start: usize, kind: TokenKind, start_loc: Loc) -> Token {
        let value = String::from_utf8_lossy(&self.input[start..self.curr_pos]).to_string();
        Token::new(value, kind, start_loc)
    }

    fn peek_char(&self, offset: usize) -> u8 {
        if self.curr_pos + offset < self.max_pos {
            self.input[self.curr_pos + offset]
        } else {
            0
        }
    }

    fn curr_char(&self) -> u8 {
        if self.has_next() {
            self.input[self.curr_pos]
        } else {
            0
        }
    }

    fn has_next(&self) -> bool {
        self.curr_pos < self.max_pos
    }

    pub fn curr_loc(&self) -> Loc {
        self.curr_loc
    }
}

/*
for elem in &self.input {
    println!("{} at {}", *elem as char, self.curr_pos);
    self.curr_pos+=1;
}
println!("{}", self.max_pos == self.curr_pos);
*/
