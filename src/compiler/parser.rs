use super::lexer::Token;
use super::ast::*;

pub fn parse(tokens: &[Token]) -> Result<Program, String> {
    let mut i = 0;
    let mut functions = Vec::new();

    while i < tokens.len() {
        if tokens[i] == Token::Newline {
            i += 1;
            continue;
        }

        if tokens[i] == Token::Fn {
            let func = parse_function(tokens, &mut i)?;
            functions.push(func);
        } else {
            return Err(format!("unexpected token {:?}", tokens[i]));
        }
    }

    Ok(Program { functions })
}

fn parse_function(tokens: &[Token], i: &mut usize) -> Result<Function, String> {
    *i += 1; // skip 'fn'

    let name = match &tokens[*i] {
        Token::Ident(s) => s.clone(),
        _ => return Err("expected function name".to_string()),
    };
    *i += 1;

    expect(tokens, i, Token::LParen)?;
    expect(tokens, i, Token::RParen)?;
    expect(tokens, i, Token::Arrow)?;

    let ret_type = match &tokens[*i] {
        Token::Ident(s) => s.clone(),
        _ => return Err("expected return type".to_string()),
    };
    *i += 1;

    expect(tokens, i, Token::LBrace)?;

    let mut body = Vec::new();

    while *i < tokens.len() && tokens[*i] != Token::RBrace {
        if tokens[*i] == Token::Newline {
            *i += 1;
            continue;
        }

        let stmt = parse_stmt(tokens, i)?;
        body.push(stmt);
    }

    expect(tokens, i, Token::RBrace)?;

    Ok(Function { name, ret_type, body })
}

fn parse_stmt(tokens: &[Token], i: &mut usize) -> Result<Stmt, String> {
    match &tokens[*i] {
        Token::Let => {
            *i += 1;
            let name = match &tokens[*i] {
                Token::Ident(s) => s.clone(),
                _ => return Err("expected variable name".to_string()),
            };
            *i += 1;
            expect(tokens, i, Token::Equals)?;
            let expr = parse_expr(tokens, i)?;
            Ok(Stmt::Let(name, expr))
        }
        Token::Return => {
            *i += 1;
            let expr = parse_expr(tokens, i)?;
            Ok(Stmt::Return(expr))
        }
        _ => Err(format!("unexpected token {:?}", tokens[*i])),
    }
}

fn parse_expr(tokens: &[Token], i: &mut usize) -> Result<Expr, String> {
    let mut left = parse_primary(tokens, i)?;

    while *i < tokens.len() {
        let op = match &tokens[*i] {
            Token::Plus => Op::Add,
            Token::Minus => Op::Sub,
            Token::Star => Op::Mul,
            Token::Slash => Op::Div,
            _ => break,
        };
        *i += 1;

        let right = parse_primary(tokens, i)?;
        left = Expr::BinOp(Box::new(left), op, Box::new(right));
    }

    Ok(left)
}

fn parse_primary(tokens: &[Token], i: &mut usize) -> Result<Expr, String> {
    match &tokens[*i] {
        Token::Ident(s) => {
            let name = s.clone();
            *i += 1;
            Ok(Expr::Var(name))
        }
        Token::Number(n) => {
            let val = *n;
            *i += 1;
            Ok(Expr::Num(val))
        }
        _ => Err(format!("unexpected token {:?}", tokens[*i])),
    }
}

fn expect(tokens: &[Token], i: &mut usize, tok: Token) -> Result<(), String> {
    if tokens[*i] == tok {
        *i += 1;
        Ok(())
    } else {
        Err(format!("expected {:?}, got {:?}", tok, tokens[*i]))
    }
}
