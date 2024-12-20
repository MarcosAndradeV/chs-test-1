use chs_lexer::Token;
use chs_util::{chs_error, CHSError, Loc};

#[derive(Default)]
pub struct Module(pub Vec<Expression>);

impl Module {
    pub fn push(&mut self, expr: Expression) {
        self.0.push(expr);
    }
}

pub enum Expression {
    Literal(Literal),
}

impl Expression {
    pub fn from_literal_token(token: Token) -> Result<Self, CHSError> {
        use chs_lexer::TokenKind::*;
        match token.kind {
            Interger => Ok(Self::Literal(Literal::IntegerLiteral {
                loc: token.loc,
                value: token.value.parse().expect("No interger token. Probably a lexer error."),
            })),
            _ => chs_error!("{} Unsuported literal", token.loc),
        }
    }
}

pub enum Literal {
    IntegerLiteral { loc: Loc, value: i64 },
}

#[cfg(test)]
mod tests {
    use chs_lexer::Lexer;

    use super::*;

    #[test]
    fn ast_literal_token() {
        let mut lex = Lexer::new(file!().into(), "10 :".into());
        assert!(Expression::from_literal_token(lex.next_token()).is_ok(), "Token 1 should be a literal");
        assert!(Expression::from_literal_token(lex.next_token()).is_err(), "Token 1 should not be a literal");
    }
}
