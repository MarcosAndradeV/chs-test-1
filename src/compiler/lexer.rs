#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum TokenKind {
    Comment,
    Identifier,
    Int,
    Float,
    Addrs,
    Invalid,
    Null,
    Whitespace,
    Proc,
    Add,
    Minus,
    Mul,
    Div,
    Print,
    CurlyOpen,
    CurlyClose,
    Pop,
    Dup,
    Eq,
    If,
    Hlt,
    Else,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
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
            b'#' => self.comment(),
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.identifier_or_keyword(self.position),
            b' ' | b'\t' | b'\r' | b'\n' => self.whitespace(),
            b'-'| b'+' | b'*' | b'/' | b'=' => self.operator(),
            b'{' => self.make_token(TokenKind::CurlyOpen),
            b'}' => self.make_token(TokenKind::CurlyClose),
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
                _ => TokenKind::Identifier
            }
            4 => match value.as_str() {
                "proc" => TokenKind::Proc,
                "else" => TokenKind::Else,
                _ => TokenKind::Identifier
            }
            5 => match value.as_str() {
                "print" => TokenKind::Print,
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
}
