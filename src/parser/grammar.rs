use crate::lexer::TokenKind;

#[derive(Debug, Clone)]
pub enum Expr {
    List(Vec<Expr>),
    BinaryOp(TokenKind, Box<Expr>, Box<Expr>),
    Bind(Box<Expr>, Box<Expr>),
    FnDef(Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Base(Box<Expr>),
    Ident(String),
    Value(isize),
//    Empty,
}