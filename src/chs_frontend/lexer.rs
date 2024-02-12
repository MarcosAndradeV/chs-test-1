use core::fmt;
use std::{
    fs::File,
    io::{self, Read},
    path::PathBuf,
};

pub fn read_file_to_bytes(filepath: PathBuf) -> io::Result<Vec<u8>> {
    let mut file = File::open(filepath)?;
    let mut data = vec![];
    file.read_to_end(&mut data)?;
    Ok(data)
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum TokenKind {
    Comment,
    Whitespace,
    Var,
    Peek,
    Assing,
    Len,
    IdxGet,
    IdxSet,
    Concat,
    Head,
    Tail,
    Invalid,
    Null,
    Int,
    Float,
    Identifier,
    Str,
    True,
    False,
    Nil,
    Import,

    Print,
    Debug,
    Exit,
    Call,

    If,
    Else,
    Whlie,
    Fn,

    Add,
    Minus,
    Mul,
    Div,
    Mod,
    Shl,
    Shr,
    Bitor,
    Bitand,
    Lor,
    Land,

    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,

    CurlyOpen,
    CurlyClose,

    ParenOpen,
    ParenClose,

    BracketOpen,
    BracketClose,

    SemiColon,
    Colon,
    DoubleColon,
    Arrow,
    Tilde,

    Pop,
    Dup,
    Over,
    Swap,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
    pub loc: (usize, usize)
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{} {:?} -> {}", self.loc.0, self.loc.1, self.kind, self.value)
    }
}

impl Token {
    fn new(kind: TokenKind, value: String, loc: (usize, usize)) -> Self {
        Self { kind, value, loc }
    }

    fn invalid(value: String, loc: (usize, usize)) -> Self {
        Self::new(TokenKind::Invalid, value, loc)
    }

    fn null(loc: (usize, usize)) -> Self {
        Self::new(TokenKind::Null, String::new(), loc)
    }

    pub fn empty() -> Self {
        Self::new(TokenKind::Null, String::new(), (0, 0))
    }

    pub fn get_loc(&self) -> String {
        format!("{}:{}", self.loc.0, self.loc.1)
    }

    pub fn get_kind(&self) -> String {
        format!("{:?}", self.kind)
    }

    pub fn get_value(&self) -> String {
        format!("{}", self.value)
    }
}

pub struct Lexer {
    input: Vec<u8>,

    max_position: usize,

    position: usize,
    col: usize,
    row: usize,
}

impl Lexer {
    pub fn new(input: Vec<u8>) -> Self {
        let max = input.len();
        Self {
            input,
            max_position: max,
            position: 0,
            col: 1,
            row: 1,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.next_regular_token()
    }

    fn current_byte(&self) -> u8 {
        if self.has_next() {
            self.input[self.position]
        } else {
            0
        }
    }

    fn next_byte(&self) -> u8 {
        self.peek(1)
    }

    fn peek(&self, offset: usize) -> u8 {
        let index = self.position + offset;

        if index < self.max_position {
            self.input[index]
        } else {
            0
        }
    }

    fn advance_char(&mut self) {
        self.position += 1;
        self.col += 1;
    }

    fn has_next(&self) -> bool {
        self.position < self.max_position
    }

    fn next_regular_token(&mut self) -> Token {
        match self.current_byte() {
            b'0'..=b'9' => self.number(false),
            b'\"' => self.string(self.position),
            b'#' => self.comment(),
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.identifier_or_keyword(self.position),
            b' ' | b'\t' | b'\r' | b'\n' => self.whitespace(),
            b'-' | b'+' | b'*' | b'/' | b'=' | b'>' | b'<' | b'|' | b'&' | b'!' | b':' => {
                self.operator()
            }
            b'{' => self.make_token(TokenKind::CurlyOpen),
            b'}' => self.make_token(TokenKind::CurlyClose),
            b'(' => self.make_token(TokenKind::ParenOpen),
            b')' => self.make_token(TokenKind::ParenClose),
            b'[' => self.make_token(TokenKind::BracketOpen),
            b']' => self.make_token(TokenKind::BracketClose),
            b';' => self.make_token(TokenKind::SemiColon),
            b'~' => self.make_token(TokenKind::Tilde),
            _ => {
                if self.has_next() {
                    self.invalid(self.position, self.position + 1)
                } else {
                    self.null()
                }
            }
        }
    }

    fn operator(&mut self) -> Token {
        match self.current_byte() {
            b'-' => match self.next_byte() {
                b'0'..=b'9' => self.number(true),
                b'>' => {
                    self.position += 2;
                    self.token(TokenKind::Arrow, self.position - 2)
                }
                _ => self.make_token(TokenKind::Minus),
            },
            b'+' => self.make_token(TokenKind::Add),
            b'*' => self.make_token(TokenKind::Mul),
            b'/' => self.make_token(TokenKind::Div),
            b'=' => self.make_token(TokenKind::Eq),
            b'!' => match self.next_byte() {
                b'=' => {
                    self.position += 2;
                    self.token(TokenKind::Neq, self.position - 2)
                }
                _ => Token::invalid("!".to_string(), (self.row, self.col)),
            },
            b'>' => match self.next_byte() {
                b'=' => {
                    self.position += 2;
                    self.token(TokenKind::Gte, self.position - 2)
                }
                b'>' => {
                    self.position += 2;
                    self.token(TokenKind::Shr, self.position - 2)
                }
                _ => self.make_token(TokenKind::Gt),
            },
            b'<' => match self.next_byte() {
                b'=' => {
                    self.position += 2;
                    self.token(TokenKind::Lte, self.position - 2)
                }
                b'<' => {
                    self.position += 2;
                    self.token(TokenKind::Shl, self.position - 2)
                }
                _ => self.make_token(TokenKind::Lt),
            },
            b'|' => match self.next_byte() {
                b'|' => {
                    self.position += 2;
                    self.token(TokenKind::Lor, self.position - 2)
                }
                _ => self.make_token(TokenKind::Bitor),
            },
            b'&' => match self.next_byte() {
                b'&' => {
                    self.position += 2;
                    self.token(TokenKind::Land, self.position - 2)
                }
                _ => self.make_token(TokenKind::Bitand),
            },
            b':' => match self.next_byte() {
                b'=' => {
                    self.position += 2;
                    self.token(TokenKind::Assing, self.position - 2)
                }
                b':' => {
                    self.position += 2;
                    self.token(TokenKind::DoubleColon, self.position - 2)
                }
                _ => self.make_token(TokenKind::Colon),
            },
            _ => self.make_token(TokenKind::Invalid),
        }
    }

    fn make_token(&mut self, kind: TokenKind) -> Token {
        self.position += 1;
        self.token(kind, self.position - 1)
    }

    fn whitespace(&mut self) -> Token {
        let start = self.position;

        while self.has_next() {
            match self.current_byte() {
                b' ' | b'\t' | b'\r' => self.advance_char(),
                b'\n' => {
                    self.row += 1;
                    self.advance_char();
                }
                _ => break,
            }
        }

        let value = self.slice_string(start, self.position);

        Token::new(TokenKind::Whitespace, value, (self.row, self.col))
    }

    fn number(&mut self, skip_first: bool) -> Token {
        let start = self.position;

        if skip_first {
            self.position += 1;
        }

        let mut kind = TokenKind::Int;

        loop {
            match self.current_byte() {
                b'0'..=b'9' => {}
                b'.' if (b'0'..=b'9').contains(&self.next_byte()) => {
                    kind = TokenKind::Float;
                }
                _ => break,
            }

            self.position += 1;
        }

        self.token(kind, start)
    }

    fn comment(&mut self) -> Token {
        self.advance_char();
        if self.current_byte() == b' ' {
            self.advance_char();
        }
        let start = self.position;
        while self.has_next() && self.current_byte() != b'\n' {
            self.position += 1;
        }
        self.token(TokenKind::Comment, start)
    }

    fn identifier_or_keyword(&mut self, start: usize) -> Token {
        self.advance_identifier_bytes();

        let value = self.slice_string(start, self.position);

        let kind = match value.len() {
            2 => match value.as_str() {
                "if" => TokenKind::If,
                "fn" => TokenKind::Fn,
                _ => TokenKind::Identifier,
            },
            3 => match value.as_str() {
                "pop" => TokenKind::Pop,
                "dup" => TokenKind::Dup,
                "mod" => TokenKind::Mod,
                "var" => TokenKind::Var,
                "len" => TokenKind::Len,
                "nil" => TokenKind::Nil,
                _ => TokenKind::Identifier,
            },
            4 => match value.as_str() {
                "else" => TokenKind::Else,
                "over" => TokenKind::Over,
                "swap" => TokenKind::Swap,
                "peek" => TokenKind::Peek,
                "true" => TokenKind::True,
                "exit" => TokenKind::Exit,
                "tail" => TokenKind::Tail,
                "head" => TokenKind::Head,
                "call" => TokenKind::Call,
                _ => TokenKind::Identifier,
            },
            5 => match value.as_str() {
                "print" => TokenKind::Print,
                "while" => TokenKind::Whlie,
                "debug" => TokenKind::Debug,
                "false" => TokenKind::False,
                _ => TokenKind::Identifier,
            },
            6 => match value.as_str() {
                "idxget" => TokenKind::IdxGet,
                "idxset" => TokenKind::IdxSet,
                "concat" => TokenKind::Concat,
                "import" => TokenKind::Import,
                _ => TokenKind::Identifier,
            },
            _ => TokenKind::Identifier,
        };

        Token::new(kind, value, (self.row, self.col))
    }

    fn advance_identifier_bytes(&mut self) {
        loop {
            match self.current_byte() {
                b'0'..=b'9' | b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.position += 1,
                _ => break,
            }
        }
    }

    fn token(&mut self, kind: TokenKind, start: usize) -> Token {
        let value = self.slice_string(start, self.position);
        Token::new(kind, value, (self.row, self.col))
    }

    fn slice_string(&mut self, start: usize, stop: usize) -> String {
        String::from_utf8_lossy(&self.input[start..stop]).into_owned()
    }

    fn invalid(&mut self, start: usize, stop: usize) -> Token {
        let value = self.slice_string(start, stop);

        self.position = self.max_position;

        Token::invalid(value, (self.row, self.col))
    }

    fn null(&self) -> Token {
        Token::null((self.row, self.col))
    }

    fn string(&mut self, start: usize) -> Token {
        let mut buffer = String::new();
        self.advance_char();
        loop {
            match self.current_byte() {
                0 => return self.invalid(start, self.position),
                b'\"' => {
                    self.advance_char();
                    break;
                }
                b'\\' => match self.next_byte() {
                    b'n' => {
                        buffer.push('\n');
                        self.advance_char();
                        self.advance_char();
                    }
                    _ => {
                        buffer.push(self.current_byte() as char);
                        self.advance_char()
                    }
                },
                b'\n' => return self.invalid(start, self.position),
                _ => {
                    buffer.push(self.current_byte() as char);
                    self.advance_char()
                }
            }
        }
        Token::new(TokenKind::Str, buffer, (self.row, self.col))
    }
}

#[cfg(test)]
mod test {
    use super::{Lexer, TokenKind};

    #[test]
    fn test() {
        let mut lex = Lexer::new("1 1 +".to_string().into_bytes());
        loop {
            let tok = lex.next_token();
            println!("{:?}", tok);
            println!("{}", tok);

            if tok.kind == TokenKind::Null {
                break;
            }
        }
    }
}

