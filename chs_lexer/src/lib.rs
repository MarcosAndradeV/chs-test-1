use core::fmt;
use std::path::PathBuf;
use chs_util::Loc;


const fn is_word_separator(c: u8) -> bool{
    matches!(c,
        b':'|
        b';'|
        b'.'|
        b','|
        b'['|
        b']'|
        b'{'|
        b'}'|
        b'('|
        b')'
    ) || c.is_ascii_whitespace()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenKind {
    Invalid,
    EOF,

    Interger,
    Keyword,
    Word,

    Assign,
    Comma,
    Semicolon,
    Colon,
    Dot,

    ParenOpen,
    ParenClose,
    CurlyOpen,
    CurlyClose,
    SquareOpen,
    SquareClose,
}

impl Default for TokenKind {
    fn default() -> Self {
        Self::Invalid
    }
}

impl TokenKind {
    fn from_word_or_keyword(value: &String) -> Self {
        match value.as_str() {
            "fn" | "if" | "else" | "while" |"true" | "false" | "nil" | "return" | "do" => Self::Keyword,
            _ => Self::Word,
        }
    }
    pub fn is_eof(&self) -> bool {
        *self == TokenKind::EOF
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Invalid => write!(f, "Invalid"),
            TokenKind::EOF => write!(f, "EOF"),
            TokenKind::Word => write!(f, "Word"),
            TokenKind::Interger => write!(f, "Interger"),
            TokenKind::Keyword => write!(f, "Keyword"),
            TokenKind::Assign => write!(f, "="),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::ParenOpen => write!(f, "("),
            TokenKind::ParenClose => write!(f, ")"),
            TokenKind::CurlyOpen => write!(f, "{{"),
            TokenKind::CurlyClose => write!(f, "}}"),
            TokenKind::SquareOpen => write!(f, "["),
            TokenKind::SquareClose => write!(f, "]"),
            TokenKind::Dot => write!(f, "."),
        }
    }
}

#[derive(Debug, Default)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
    pub loc: Loc,
}

impl Token {
    pub fn val_eq(&self, value: &str) -> bool {
        self.value == value
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?}({})", self.loc, self.kind, self.value)
    }
}

#[derive(Default)]
pub struct Lexer {
    input: Vec<u8>,
    pos: usize,
    read_pos: usize,
    ch: u8,
    loc: Loc,
}

impl Lexer {
    pub fn get_filename(&self) -> PathBuf {
        self.loc.filepath.clone()
    }
    pub fn new(filepath: PathBuf, input: Vec<u8>) -> Self {
        let mut lex = Self {
            input,
            loc: Loc::new(
                filepath,
                1,
                1,
            ),
            ..Default::default()
        };
        lex.read_char();
        return lex;
    }
    fn read_char(&mut self) {
        if self.read_pos >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_pos];
        }
        self.pos = self.read_pos;
        self.read_pos += 1;
        self.loc.next(self.ch);
    }
    #[allow(dead_code)]
    fn peek_char(&mut self) -> u8 {
        if self.pos >= self.input.len() {
            0
        } else {
            self.input[self.read_pos]
        }
    }
    pub fn next_token(&mut self) -> Token {
        use TokenKind::*;
        self.skip_whitespace();
        match self.ch {
            b':' => self.make_token(Colon, ":"),
            b'.' => self.make_token(Dot, "."),
            b'=' => self.make_token(Assign, "="),
            b',' => self.make_token(Comma, ","),
            b';' => self.make_token(Semicolon, ";"),
            b'(' => self.make_token(ParenOpen, "("),
            b')' => self.make_token(ParenClose, ")"),
            b'{' => self.make_token(CurlyOpen, "{"),
            b'}' => self.make_token(CurlyClose, "}"),
            b'[' => self.make_token(SquareOpen, "["),
            b']' => self.make_token(SquareClose, "]"),
            b'0'..=b'9' => self.number(),
            0 => self.make_token(EOF, "\0"),
            _ => self.word(),
        }
    }

    fn make_token(&mut self, kind: TokenKind, value: &str) -> Token {
        let loc = self.loc.clone();
        self.read_char();
        Token {
            kind,
            value: value.into(),
            loc,
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            if !self.ch.is_ascii_whitespace() {
                break;
            }
            self.read_char();
        }
    }

    fn number(&mut self) -> Token {
        let start_pos = self.pos;
        let loc = self.loc.clone();

        loop {
            if !matches!(self.ch, b'0'..=b'9') {
                break;
            }
            self.read_char();
        }
        let value: String = String::from_utf8_lossy(&self.input[start_pos..self.pos]).into();

        Token {
            kind: TokenKind::Interger,
            value,
            loc,
        }
    }

    fn word(&mut self) -> Token {
        let start_pos = self.pos;
        let loc = self.loc.clone();

        loop {
            if is_word_separator(self.ch) {
                break;
            }
            self.read_char();
        }
        let value: String = String::from_utf8_lossy(&self.input[start_pos..self.pos]).into();
        if value.is_empty() {
            return Token::default();
        }
        Token {
            kind: TokenKind::from_word_or_keyword(&value),
            value,
            loc,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct LexTester {
        line: usize,
        col: usize,
    }
    impl LexTester {
        fn new() -> Self {
            LexTester { line: 1, col: 1 }
        }
        fn gen_token(&self, kind: TokenKind, value: &str) -> Token {
            Token {
                loc: Loc::new(file!().into(), self.line, self.col),
                kind,
                value: value.to_string(),
            }
        }
    }

    #[test]
    fn test_next_token_1() {
        use TokenKind::*;
        let input = "=+(){}".to_string();
        let tlex = LexTester::new();
        let tests = &[
            tlex.gen_token(Assign, "="),
            tlex.gen_token(Word, "+"),
            tlex.gen_token(ParenOpen, "("),
            tlex.gen_token(ParenClose, ")"),
            tlex.gen_token(CurlyOpen, "{"),
            tlex.gen_token(CurlyClose, "}"),
            tlex.gen_token(EOF, "\0"),
        ];
        let mut lex = Lexer::new(file!().into(), input.into_bytes());
        for tt in tests {
            assert_eq!(lex.next_token().kind, tt.kind)
        }
    }

    #[test]
    fn test_next_token_2() {
        use TokenKind::*;
        let input = r#"
            x := 5;
            y := 10;
            x + y;
            "#
        .to_string();
        let tlex = LexTester::new();
        let tests = &[
            tlex.gen_token(Word, "x"),
            tlex.gen_token(Colon, ":"),
            tlex.gen_token(Assign, "="),
            tlex.gen_token(Interger, "5"),
            tlex.gen_token(Semicolon, ";"),
            tlex.gen_token(Word, "y"),
            tlex.gen_token(Colon, ":"),
            tlex.gen_token(Assign, "="),
            tlex.gen_token(Interger, "5"),
            tlex.gen_token(Semicolon, ";"),
            tlex.gen_token(Word, "x"),
            tlex.gen_token(Word, "+"),
            tlex.gen_token(Word, "y"),
            tlex.gen_token(Semicolon, ";"),
        ];
        let mut lex = Lexer::new(file!().into(), input.into_bytes());
        for tt in tests {
            assert_eq!(lex.next_token().kind, tt.kind)
        }
    }

    #[test]
    fn test_next_token_3() {
        use TokenKind::*;
        let input = r#"
            ! - / * 5;
            "#
        .to_string();
        let tlex = LexTester::new();
        let tests = &[
            tlex.gen_token(Word, "!"),
            tlex.gen_token(Word, "-"),
            tlex.gen_token(Word, "/"),
            tlex.gen_token(Word, "*"),
            tlex.gen_token(Interger, "5"),
            tlex.gen_token(Semicolon, ";"),
            tlex.gen_token(EOF, "\0"),
        ];
        let mut lex = Lexer::new(file!().into(), input.into_bytes());
        for tt in tests {
            assert_eq!(lex.next_token().kind, tt.kind)
        }
    }

    #[test]
    fn test_next_token_4() {
        use TokenKind::*;
        let input = r#"
            false != true;
            "#
        .to_string();
        let tlex = LexTester::new();
        let tests = &[
            tlex.gen_token(Keyword, "false"),
            tlex.gen_token(Word, "!="),
            tlex.gen_token(Keyword, "true"),
            tlex.gen_token(Semicolon, ";"),
            tlex.gen_token(EOF, "\0"),
        ];
        let mut lex = Lexer::new(file!().into(), input.into_bytes());
        for tt in tests {
            assert_eq!(lex.next_token().kind, tt.kind)
        }
    }
}
