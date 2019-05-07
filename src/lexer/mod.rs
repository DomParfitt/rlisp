#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TokenKind {
    OpenParen,
    CloseParen,
    Plus,
    Minus,
    Star,
    Div,
    Eq,
    GreaterThan,
    LessThan,
    GreaterThanEq,
    LessThanEq,
    Num,
    Def,
    Fn,
    If,
    Ident,
    String,
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
    pub line: usize,
    pub col: usize,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut line = 0;
    let mut col = 0;
    let mut tokens: Vec<Token> = vec!();

    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        if c.is_ascii_whitespace() {
            if c == '\n' || c == '\r' {
                line += 1;
                col = 0;
            } else {
                col += 1;
            }
            continue;
        }

        if c.is_ascii_digit() {
            let mut value = c.to_string();
            while let Some(c) = chars.peek() {
                if !c.is_ascii_digit() {
                    break;
                }
                value.push(chars.next().unwrap());
            }

            let token = Token {
                kind: TokenKind::Num,
                value,
                line,
                col,
            };
            col += token.value.len();
            tokens.push(token);
            continue;
        }

        if c.is_ascii_alphabetic() {
            let mut value = c.to_string();
            while let Some(c) = chars.peek() {
                if !c.is_ascii_alphanumeric() {
                    break;
                }
                value.push(chars.next().unwrap());
            }
            let kind = match value.as_ref() {
                "if" => TokenKind::If,
                "def" => TokenKind::Def,
                "fn" => TokenKind::Fn,
                _ => TokenKind::Ident,
            };

            let token = Token {
                kind,
                value,
                line,
                col,
            };
            col += token.value.len();
            tokens.push(token);
            continue;
        }

        let mut value = c.to_string();
        let kind = match c {
            '(' => TokenKind::OpenParen,
            ')' => TokenKind::CloseParen,
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Star,
            '/' => TokenKind::Div,
            '=' => TokenKind::Eq,
            '>' => {
                if let Some('=') = chars.peek() {
                    value.push(chars.next().unwrap());
                    TokenKind::GreaterThanEq
                } else {
                    TokenKind::GreaterThan
                }
            }
            '<' => {
                if let Some('=') = chars.peek() {
                    value.push(chars.next().unwrap());
                    TokenKind::LessThanEq
                } else {
                    TokenKind::LessThan
                }
            }
            '"' => {
                value = "".to_string();
                while let Some(c) = chars.next() {
                    if c == '"' {
                        break;
                    }
                    value.push(c);
                }
                TokenKind::String
            }
            _ => continue
        };

        let token = Token {
            kind,
            value,
            line,
            col,
        };
        col += token.value.len();
        tokens.push(token);
    }

    tokens.push(Token {
        kind: TokenKind::EOF,
        value: "EOF".to_string(),
        line,
        col,
    });
    tokens
}
