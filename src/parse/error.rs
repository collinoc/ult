use super::super::lex::token::Token;
use std::error::Error;
use std::fmt::{ Display, Result, Formatter };

#[allow(dead_code)]
#[derive(Debug)]
pub enum ParseError {
    BadExpression(Token),
    BadStatement(Token),
    BadAssignment,
    UnexpectedToken(Token),
    SyntaxError,
    UnexpectedEOF,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ParseError::BadExpression(tok) => write!(f, "Bad expression! {:?}", tok),
            ParseError::BadStatement(tok) => write!(f, "Bad statement! {:?}", tok),
            ParseError::BadAssignment => write!(f, "Bad assignment!"),
            ParseError::UnexpectedToken(tok) => write!(f, "Unexpected token! {:?}", tok),
            ParseError::SyntaxError => write!(f, "Syntax error!"),
            ParseError::UnexpectedEOF => write!(f, "Unexpected EOF!"),
        }
    }
}

impl Error for ParseError {}