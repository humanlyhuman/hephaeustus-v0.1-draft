#[derive(Debug, Clone)]
pub struct Program {
    pub functions: Vec<Function>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub ret_type: String,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let(String, Expr),
    Return(Expr),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Var(String),
    Num(i64),
    BinOp(Box<Expr>, Op, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}
