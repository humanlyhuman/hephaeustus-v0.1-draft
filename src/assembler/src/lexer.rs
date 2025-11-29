#[derive(Debug, Clone)]
pub enum Tok {
    Ident(String),
    Number(i64),
    Colon,
    Comma,
    Newline,
}

pub fn lex(src: &str) -> Result<Vec<Tok>, String> {
    let mut toks = Vec::new();
    let mut cur = String::new();
    let mut in_comment = false;

    for ch in src.chars() {
        if in_comment {
            if ch == '\n' {
                in_comment = false;
                toks.push(Tok::Newline);
            }
            continue;
        }

        match ch {
            ';' => {
                if !cur.is_empty() {
                    toks.push(parse_token(&cur));
                    cur.clear();
                }
                in_comment = true;
            }
            ':' => {
                if !cur.is_empty() {
                    toks.push(parse_token(&cur));
                    cur.clear();
                }
                toks.push(Tok::Colon);
            }
            ',' => {
                if !cur.is_empty() {
                    toks.push(parse_token(&cur));
                    cur.clear();
                }
                toks.push(Tok::Comma);
            }
            '\n' => {
                if !cur.is_empty() {
                    toks.push(parse_token(&cur));
                    cur.clear();
                }
                toks.push(Tok::Newline);
            }
            ' ' | '\t' | '\r' => {
                if !cur.is_empty() {
                    toks.push(parse_token(&cur));
                    cur.clear();
                }
            }
            _ => cur.push(ch),
        }
    }

    if !cur.is_empty() {
        toks.push(parse_token(&cur));
    }

    Ok(toks)
}

fn parse_token(s: &str) -> Tok {
    if let Ok(n) = s.parse::<i64>() {
        Tok::Number(n)
    } else if s.starts_with("0x") || s.starts_with("0X") {
        if let Ok(n) = i64::from_str_radix(&s[2..], 16) {
            Tok::Number(n)
        } else {
            Tok::Ident(s.to_string())
        }
    } else {
        Tok::Ident(s.to_string())
    }
}
