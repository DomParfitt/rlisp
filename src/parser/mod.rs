use crate::lexer::{Token, TokenKind};
use crate::parser::error::ParseError;
use crate::parser::grammar::Expr;

pub mod error;
pub mod grammar;

type ParseResult = Result<Expr, ParseError>;

pub struct Parser {
    current: usize,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            current: 0,
            tokens: vec!(),
        }
    }

    pub fn parse(&mut self, tokens: Vec<Token>) -> ParseResult {
        self.tokens = tokens;
        self.current = 0;

        self.list()
    }

    fn list(&mut self) -> ParseResult {
        let mut exprs: Vec<Expr> = vec!();
        while let Some(_) = self.peek() {
            let expr = self.expr();
            match expr {
                Ok(expr) => exprs.push(expr),
                Err(err) => return expr,
            }
        }

        Ok(Expr::List(exprs))
    }

    // Expr ::= Base | '(' SubExpr ')'
    fn expr(&mut self) -> ParseResult {
        if let Ok(base) = self.base() {
            return Ok(base);
        }

        self.expect(TokenKind::OpenParen)?;
        let expr = self.sub_expr()?;
        self.expect(TokenKind::CloseParen)?;

        Ok(expr)
    }

    // Convenience method for Expr in brackets
    fn sub_expr(&mut self) -> ParseResult {
        if let Ok(base) = self.base() {
            return Ok(base);
        }

        if let Ok(bin_op) = self.binary_op() {
            return Ok(bin_op);
        }

        if let Ok(bind) = self.bind() {
            return Ok(bind);
        }

        if let Ok(if_expr) = self.if_expr() {
            return Ok(if_expr);
        }

        self.expr()
    }

    // BinaryOp ::= <operator> Expr Expr
    fn binary_op(&mut self) -> ParseResult {
        let op = self.expect_any(vec!(
            TokenKind::Plus,
            TokenKind::Minus,
            TokenKind::Star,
            TokenKind::Div,
            TokenKind::Eq,
            TokenKind::GreaterThan,
            TokenKind::GreaterThan,
            TokenKind::LessThan,
            TokenKind::LessThanEq
        ))?;

        let kind = op.kind;

        let left = self.expr()?;
        let right = self.expr()?;

        Ok(Expr::BinaryOp(kind, Box::new(left), Box::new(right)))
    }

    // Bind ::= 'def' Ident Expr
    fn bind(&mut self) -> ParseResult {
        self.expect(TokenKind::Def)?;
        let id = self.ident()?;
        let value = self.expr()?;

        Ok(Expr::Bind(Box::new(id), Box::new(value)))
    }

    fn def_fn(&mut self) -> ParseResult {
        self.expect(TokenKind::Fn)?;
        let args = self.list()?;
        let body = self.expr()?;

        Ok(Expr::FnDef(Box::new(args), Box::new(body)))
    }

    // If ::= 'if' Expr Expr Expr
    fn if_expr(&mut self) -> ParseResult {
        self.expect(TokenKind::If)?;
        let condition = self.expr()?;
        let result = self.expr()?;
        let alternate = self.expr()?;

        Ok(Expr::If(Box::new(condition), Box::new(result), Box::new(alternate)))
    }

    fn base(&mut self) -> ParseResult {
        if let Ok(value) = self.value() {
            return Ok(value);
        }

        self.ident()
    }

    fn ident(&mut self) -> ParseResult {
        let token = self.expect(TokenKind::Ident)?;
        return Ok(Expr::Ident(token.value.clone()));
    }

    fn value(&mut self) -> ParseResult {
        let token = self.expect(TokenKind::Num)?;
        return Ok(Expr::Value(token.value.parse().unwrap()));
    }

    fn next(&mut self) -> Option<&Token> {
        if self.current < self.tokens.len() {
            let token = &self.tokens[self.current];
            if token.kind == TokenKind::EOF {
                return None;
            }
            self.current += 1;
            return Some(token);
        }

        None
    }

    fn peek(&self) -> Option<&Token> {
        if self.current < self.tokens.len() {
            let token = &self.tokens[self.current];
            if token.kind == TokenKind::EOF {
                return None;
            }
            return Some(token);
        }

        None
    }

    fn expect(&mut self, kind: TokenKind) -> Result<&Token, ParseError> {
        if let Some(token) = self.peek() {
            if token.kind == kind {
                if let Some(token) = self.next() {
                    return Ok(token);
                }
            } else {
                return Err(ParseError::UnexpectedSymbol(kind, token.kind));
            }
        }

        Err(ParseError::UnexpectedSymbol(kind, TokenKind::EOF))
    }

    fn expect_any(&mut self, kinds: Vec<TokenKind>) -> Result<&Token, ParseError> {
        if let Some(token) = self.peek() {
            for (idx, kind) in kinds.clone().into_iter().enumerate() {
                if token.kind == kind {
                    if let Some(token) = self.next() {
                        return Ok(token);
                    } else {
                        return Err(ParseError::UnexpectedSymbol(kind, TokenKind::EOF));
                    }
                }

                if idx == kinds.len() - 1 {
                    return Err(ParseError::UnexpectedSymbol(kind, token.kind));
                }
            }
        }

        return Err(ParseError::Err);
    }
}


