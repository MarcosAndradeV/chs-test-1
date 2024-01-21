use std::{path::PathBuf, fs::File, io::{self, Read}};
use core::fmt;

pub fn lex_file(filepath: PathBuf) -> io::Result<Vec<u8>> {
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
    Set,
    Len,

    Directive,
    Def,
    
    Invalid,
    Null,
    
    Int,
    Float,
    Identifier,
    Str,
    Char,
    
    Hlt,
    Print,
    Println,
    Debug,
    
    If,
    Else,
    Whlie,

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
    
    Pop,
    Dup,
    Dup2,
    Over,
    Swap,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}({})", self.kind, self.value)
    }
}

impl Token {
    fn new(kind: TokenKind, value: String) -> Self {
        Self { kind, value }
    }

    fn invalid(value: String) -> Self {
        Self::new(TokenKind::Invalid, value)
    }

    fn null() -> Self {
        Self::new(TokenKind::Null, String::new())
    }


}

pub struct Lexer {
    input: Vec<u8>,

    max_position: usize,

    position: usize,
}

impl Lexer {
    pub fn new(input: Vec<u8>) -> Self {
        let max = input.len();
        Self {
            input,
            max_position: max,
            position: 0,
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
    }

    fn has_next(&self) -> bool {
        self.position < self.max_position
    }

    fn next_regular_token(&mut self) -> Token {
        match self.current_byte() {
            b'0'..=b'9' => self.number(false),
            b'\"' => self.string(),
            b'#' => self.comment(),
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.identifier_or_keyword(self.position),
            b' ' | b'\t' | b'\r' | b'\n' => self.whitespace(),
            b'-'| b'+' | b'*' | b'/' | b'=' | b'>' | b'<' | b'|' | b'&' | b'!' => self.operator(),
            b'{' => self.make_token(TokenKind::CurlyOpen),
            b'}' => self.make_token(TokenKind::CurlyClose),
            b'(' => self.make_token(TokenKind::ParenOpen),
            b')' => self.make_token(TokenKind::ParenClose),
            b'[' => self.make_token(TokenKind::BracketOpen),
            b']' => self.make_token(TokenKind::BracketClose),
            b'%' => self.make_token(TokenKind::Directive),
            b';' => self.make_token(TokenKind::SemiColon),
            b':' => self.make_token(TokenKind::Colon),
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
            b'-' => {

                match self.next_byte() {
                    b'0'..=b'9' => self.number(true),
                    _ => self.make_token(TokenKind::Minus)
                }
                
            },
            b'+' => self.make_token(TokenKind::Add),
            b'*' => self.make_token(TokenKind::Mul),
            b'/' => self.make_token(TokenKind::Div),
            b'=' => self.make_token(TokenKind::Eq),
            b'!' => {
                match self.next_byte() {
                    b'=' => {self.position+=2; self.token(TokenKind::Neq, self.position-2)},
                    _ => Token::invalid("! ?".to_string())
                }
            }
            b'>' => {
                match self.next_byte() {
                    b'=' => {self.position+=2; self.token(TokenKind::Gte, self.position-2)},
                    b'>' => {self.position+=2; self.token(TokenKind::Shr, self.position-2)},
                    _ => self.make_token(TokenKind::Gt)
                }
            }
            b'<' => {
                match self.next_byte() {
                    b'=' => {self.position+=2; self.token(TokenKind::Lte, self.position-2)},
                    b'<' => {self.position+=2; self.token(TokenKind::Shl, self.position-2)},
                    _ => self.make_token(TokenKind::Lt)
                }
            }
            b'|' => {
                match self.next_byte() {
                    b'|' => {self.position+=2; self.token(TokenKind::Lor, self.position-2)},
                    _ => self.make_token(TokenKind::Bitor)
                }
            },
            b'&' => {
                match self.next_byte() {
                    b'&' => {self.position+=2; self.token(TokenKind::Land, self.position-2)},
                    _ => self.make_token(TokenKind::Bitand)
                }
            },
            _ => self.make_token(TokenKind::Invalid)
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
                b' ' | b'\t' | b'\r' | b'\n' => self.advance_char(),
                _ => break,
            }
        }

        let value = self.slice_string(start, self.position);

        Token::new(TokenKind::Whitespace, value)
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
                _ => TokenKind::Identifier
            }
            3 => match value.as_str() {
                "pop" => TokenKind::Pop,
                "dup" => TokenKind::Dup,
                "hlt" => TokenKind::Hlt,
                "mod" => TokenKind::Mod,
                "def" => TokenKind::Def,
                "var" => TokenKind::Var,
                "set" => TokenKind::Set,
                "len" => TokenKind::Len,
                _ => TokenKind::Identifier
            }
            4 => match value.as_str() {
                "else" => TokenKind::Else,
                "dup2" => TokenKind::Dup2,
                "over" => TokenKind::Over,
                "swap" => TokenKind::Swap,
                _ => TokenKind::Identifier
            }
            5 => match value.as_str() {
                "print" => TokenKind::Print,
                "while" => TokenKind::Whlie,
                "debug" => TokenKind::Debug,
                _ => TokenKind::Identifier
            }
            7 => match value.as_str() {
                "println" => TokenKind::Println,  
                _ => TokenKind::Identifier
            }
            _ => TokenKind::Identifier,
        };

        Token::new(kind, value)
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
        Token::new(kind, value)
    }

    fn slice_string(&mut self, start: usize, stop: usize) -> String {
        String::from_utf8_lossy(&self.input[start..stop]).into_owned()
    }

    fn invalid(&mut self, start: usize, stop: usize) -> Token {
        let value = self.slice_string(start, stop);

        self.position = self.max_position;

        Token::invalid(value)
    }

    fn null(&self) -> Token {
        Token::null()
    }

    fn string(&mut self) -> Token {
        let mut buffer = String::new();
        self.advance_char();
        loop {
            match self.current_byte() {
                0 | b'\"' => {self.advance_char(); break;}
                b'\\' => {
                    match self.next_byte() {
                        b'n' => {buffer.push('\n'); self.advance_char(); self.advance_char();}
                        _ => {buffer.push(self.current_byte() as char); self.advance_char()}
                    }
                }
                _ => {buffer.push(self.current_byte() as char); self.advance_char()}
            }
        }
        Token::new(TokenKind::Str, buffer)
    }
}


#[cfg(test)]
mod test {
    use super::{Lexer, TokenKind};

   #[test]
   fn test(){
    let mut lex = Lexer::new("%def".to_string().into_bytes());
    loop {
        let tok = lex.next_token();
        println!("{:?}", tok);

        if tok.kind == TokenKind::Null {break;}
    }
   }
}