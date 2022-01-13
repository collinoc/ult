use super::token::*;
use std::fmt::{ Display, Result, Formatter };
use std::error::Error;
use TokenError::*;

#[derive(Debug)]
pub enum TokenError {
    Unknown(char, (i32, i32)),
    Unexpected(char, (i32, i32)),
    Invalid(String, (i32, i32)),
    Expected(char, char, (i32, i32)),
    UnexpectedEOF,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

impl Display for TokenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Unknown(c, (l, p)) => write!(f, "Unknown token '{}' on line {} position {}", c, l, p),
            Unexpected(c, (l, p)) => write!(f, "Unexpected token '{}' on line {} position {}", c, l, p),
            Invalid(s, (l, p)) => write!(f, "Invalid token '{}' on line {} position {}", s, l, p),
            Expected(c, s, (l, p)) => write!(f, "Expected character '{}' but instead saw '{}' on line {} position {}", c, s, l, p),
            UnexpectedEOF => write!(f, "Unexpected EOF occurred"),
        }
    }
}

impl Error for TokenError {}