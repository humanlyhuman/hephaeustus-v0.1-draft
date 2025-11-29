use crate::lexer::Tok;

#[derive(Debug, Clone)]
pub enum Inst {
    Label(String),
    Op(String, Vec<Arg>),
}

#[derive(Debug, Clone)]
pub enum Arg {
    Reg(String),
    Cap(String),
    Imm(i64),
    Label(String),
}

pub fn parse(toks: &[Tok]) -> Result<Vec<Inst>, String> {
    let mut out = Vec::new();
    let mut i = 0;

    while i < toks.len() {
        match &toks[i] {
            Tok::Ident(s) => {
                if i + 1 < toks.len() && matches!(toks[i + 1], Tok::Colon) {
                    out.push(Inst::Label(s.clone()));
                    i += 2;
                } else {
                    let op = s.clone();
                    i += 1;
                    let mut args = Vec::new();

                    while i < toks.len() && !matches!(toks[i], Tok::Newline) {
                        match &toks[i] {
                            Tok::Ident(arg) => {
                                if arg.starts_with('r') {
                                    args.push(Arg::Reg(arg.clone()));
                                } else if arg.starts_with('c') {
                                    args.push(Arg::Cap(arg.clone()));
                                } else {
                                    args.push(Arg::Label(arg.clone()));
                                }
                                i += 1;
                            }
                            Tok::Number(n) => {
                                args.push(Arg::Imm(*n));
                                i += 1;
                            }
                            Tok::Comma => {
                                i += 1;
                            }
                            _ => {
                                return Err(format!("unexpected token at position {}", i));
                            }
                        }
                    }

                    out.push(Inst::Op(op, args));
                }
            }
            Tok::Newline => {
                i += 1;
            }
            _ => {
                return Err(format!("unexpected token at position {}", i));
            }
        }
    }

    Ok(out)
}
