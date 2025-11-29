use super::ast::*;

#[derive(Debug, Clone)]
pub struct IRProgram {
    pub functions: Vec<IRFunction>,
}

#[derive(Debug, Clone)]
pub struct IRFunction {
    pub name: String,
    pub instrs: Vec<IRInst>,
}

#[derive(Debug, Clone)]
pub enum IRInst {
    LoadImm(String, i64),
    Add(String, String, String),
    Sub(String, String, String),
    Mul(String, String, String),
    Div(String, String, String),
    Ret(String),
}

pub fn lower_ast(prog: &Program) -> Result<IRProgram, String> {
    let mut functions = Vec::new();

    for func in &prog.functions {
        let mut instrs = Vec::new();
        let mut var_map = std::collections::HashMap::new();
        let mut temp_counter = 0;

        for stmt in &func.body {
            match stmt {
                Stmt::Let(name, expr) => {
                    let tmp = lower_expr(expr, &mut instrs, &var_map, &mut temp_counter);
                    var_map.insert(name.clone(), tmp.clone());
                }
                Stmt::Return(expr) => {
                    let tmp = lower_expr(expr, &mut instrs, &var_map, &mut temp_counter);
                    instrs.push(IRInst::Ret(tmp));
                }
            }
        }

        functions.push(IRFunction {
            name: func.name.clone(),
            instrs,
        });
    }

    Ok(IRProgram { functions })
}

fn lower_expr(
    expr: &Expr,
    instrs: &mut Vec<IRInst>,
    vars: &std::collections::HashMap<String, String>,
    temp: &mut usize,
) -> String {
    match expr {
        Expr::Num(n) => {
            let t = format!("t{}", temp);
            *temp += 1;
            instrs.push(IRInst::LoadImm(t.clone(), *n));
            t
        }
        Expr::Var(v) => vars.get(v).unwrap().clone(),
        Expr::BinOp(l, op, r) => {
            let lt = lower_expr(l, instrs, vars, temp);
            let rt = lower_expr(r, instrs, vars, temp);
            let t = format!("t{}", temp);
            *temp += 1;

            let inst = match op {
                Op::Add => IRInst::Add(t.clone(), lt, rt),
                Op::Sub => IRInst::Sub(t.clone(), lt, rt),
                Op::Mul => IRInst::Mul(t.clone(), lt, rt),
                Op::Div => IRInst::Div(t.clone(), lt, rt),
            };
            instrs.push(inst);
            t
        }
    }
}
