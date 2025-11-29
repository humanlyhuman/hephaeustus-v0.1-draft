#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Fn,
    Let,
    Return,
    Arrow,
    Ident(String),
    Number(i64),
    LParen,
    RParen,
    LBrace,
    RBrace,
    Colon,
    Equals,
    Plus,
    Minus,
    Star,
    Slash,
    Newline,
}

pub fn lex(src: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = src.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\t' | '\r' => { chars.next(); }
            '\n' => { chars.next(); tokens.push(Token::Newline); }
            '(' => { chars.next(); tokens.push(Token::LParen); }
            ')' => { chars.next(); tokens.push(Token::RParen); }
            '{' => { chars.next(); tokens.push(Token::LBrace); }
            '}' => { chars.next(); tokens.push(Token::RBrace); }
            ':' => { chars.next(); tokens.push(Token::Colon); }
            '=' => { chars.next(); tokens.push(Token::Equals); }
            '+' => { chars.next(); tokens.push(Token::Plus); }
            '*' => { chars.next(); tokens.push(Token::Star); }
            '/' => { chars.next(); tokens.push(Token::Slash); }
            '-' => {
                chars.next();
                if chars.peek() == Some(&'>') {
                    chars.next();
                    tokens.push(Token::Arrow);
                } else {
                    tokens.push(Token::Minus);
                }
            }
            '0'..='9' => {
                let mut num = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_numeric() {
                        num.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(num.parse().unwrap()));
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut word = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        word.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(match word.as_str() {
                    "fn" => Token::Fn,
                    "let" => Token::Let,
                    "return" => Token::Return,
                    _ => Token::Ident(word),
                });
            }
            _ => return Err(format!("unexpected char: {}", ch)),
        }
    }

    Ok(tokens)
}
