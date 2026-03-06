#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Value),
    BinaryOp {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    Call {
        function: String,
        args: Vec<Expr>,
    },
    Var {
        name: String,
    },
    Let {
        name: String,
        value: Box<Expr>,
        r#type: Option<Type>,
    },
    FuncDef {
        name: String,
        params: Vec<String>,
        body: Box<Expr>,
        return_type: Option<Type>,
    },
    Return {
        value: Option<Box<Expr>>,
    },
    Block {
        statements: Vec<Expr>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Int,
    Float,
    Str,
    Unknown,
    Void,
}

#[derive(Debug, Clone)]
pub enum Value {
    Int(i32),
    Float(f64),
    Str(String),
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}
