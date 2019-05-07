use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt;
use crate::lexer::{Token, TokenKind};

#[derive(Debug, Copy, Clone)]
pub enum ParseError {
    Err,
    UnexpectedSymbol(TokenKind, TokenKind),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            ParseError::Err => write!(f, "unknown parse error"),
            ParseError::UnexpectedSymbol(ref expected, ref actual) => write!(f, "Expected {:?} but found {:?}", expected, actual),
        }
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
//        match self {
//            ParseError::Err => "parse error",
//            ParseError::UnexpectedSymbol(expected, actual) => format!("").as_ref(),
//        }
        "parse error"
    }
}