#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Value),
    BinaryOp {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
        r#type: Type,
    },
    BooleanOp {
        left: Box<Expr>,
        op: BooleanOp,
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
        //r#type: Option<Type>,
    },
    FuncDef {
        name: String,
        params: Vec<(String, Type)>,
        body: Box<Expr>,
        return_type: Option<Type>,
    },
    Return {
        value: Option<Box<Expr>>,
    },
    Value {
        value: Option<Box<Expr>>,
    },
    If {
        condition: Box<Expr>,
        body: Box<Expr>,
        r#else: Box<Expr>,
    },
    Block {
        statements: Vec<Expr>,
        returns: Box<Expr>,
    },
    NotExist,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Int,
    Float,
    Str,
    Bool,
    Unknown,
    Void,
}

impl Type {
    pub fn translate(&self) -> String {
        match self {
            Type::Int => String::from("i32"),
            Type::Float => String::from("f64"),
            Type::Str => String::from("String"),
            Type::Unknown => {
                eprintln!("Debug info : {:?}", self);
                panic!("Unknown type made it into code generation phase");
            }
            Type::Void => String::from("()"),
            Type::Bool => String::from("bool"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i32),
    Float(f64),
    Str(String),
    Bool(bool),
}

impl Value {
    pub fn parse_type(&self) -> Type {
        match self {
            Value::Int(_) => Type::Int,
            Value::Float(_) => Type::Float,
            Value::Str(_) => Type::Str,
            Value::Bool(_) => Type::Bool,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BooleanOp {
    And,
    Or,
}
#[derive(Debug, Clone)]
pub struct FuncSignature {
    pub param_types: Vec<Type>,
    pub return_type: Type,
}
